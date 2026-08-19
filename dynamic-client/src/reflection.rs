use prost::Message;
use prost_reflect::DescriptorPool;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use tokio_stream::iter;
use tonic::transport::Channel;
use tonic_reflection::pb::v1alpha::{
    ServerReflectionRequest, server_reflection_client::ServerReflectionClient,
    server_reflection_request::MessageRequest, server_reflection_response::MessageResponse,
};

pub async fn list_services(channel: Channel) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut client = ServerReflectionClient::new(channel);

    let request = ServerReflectionRequest {
        host: String::new(),
        message_request: Some(MessageRequest::ListServices("".into())),
    };

    let request_stream = iter(vec![request]);
    let mut response_stream = client
        .server_reflection_info(request_stream)
        .await?
        .into_inner();

    let mut service_names = Vec::new();

    if let Some(response) = response_stream.message().await? {
        match response.message_response {
            Some(MessageResponse::ListServicesResponse(list)) => {
                for svc in list.service {
                    // Skip the reflection service itself to save network overhead
                    if !svc.name.contains("ServerReflection") {
                        service_names.push(svc.name);
                    }
                }
            }
            Some(MessageResponse::ErrorResponse(err)) => {
                return Err(format!("Reflection ListServices Error: {}", err.error_message).into());
            }
            _ => return Err("Unexpected response to ListServices".into()),
        }
    };
    Ok(service_names)
}

pub async fn fetch_descriptor_pool(
    channel: Channel,
    symbol: &str,
) -> Result<DescriptorPool, Box<dyn std::error::Error>> {
    let mut client = ServerReflectionClient::new(channel);

    let request = ServerReflectionRequest {
        host: String::new(),
        message_request: Some(MessageRequest::FileContainingSymbol(symbol.to_string())),
    };

    let request_stream = iter(vec![request]);
    let mut response_stream = client
        .server_reflection_info(request_stream)
        .await?
        .into_inner();

    let mut fd_set = FileDescriptorSet::default();
    if let Some(response) = response_stream.message().await? {
        match response.message_response {
                Some(tonic_reflection::pb::v1alpha::server_reflection_response::MessageResponse::FileDescriptorResponse(fd_res)) => {
                    // The server returns a list of raw byte vectors.
                    // Each vector is a serialized FileDescriptorProto.
                    for raw_fd in fd_res.file_descriptor_proto {
                        let fd_proto = FileDescriptorProto::decode(raw_fd.as_slice())?;
                        fd_set.file.push(fd_proto);
                    }
                }
                Some(tonic_reflection::pb::v1alpha::server_reflection_response::MessageResponse::ErrorResponse(err)) => {
                    return Err(format!("Reflection RPC Error {}: {}", err.error_code, err.error_message).into());
                }
                _ => return Err("Unexpected or empty reflection response".into()),
            }
    } else {
        return Err("Server closed reflection stream prematurely".into());
    }
    let mut pool_bytes = Vec::new();
    fd_set.encode(&mut pool_bytes)?;

    let pool = DescriptorPool::decode(pool_bytes.as_slice())?;
    Ok(pool)
}
