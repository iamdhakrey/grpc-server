use crate::codec::DynamicCodec;
use http::uri::PathAndQuery;
use prost_reflect::{DynamicMessage, MessageDescriptor};
use std::str::FromStr;
use tonic::{Request, Status, client::Grpc, transport::Channel};

#[derive(Clone)]
pub struct DynamicClient {
    channel: Channel,
}

impl DynamicClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    /// Dispatches a generic unary gRPC call using dynamic schemas.
    pub async fn unary_call(
        &self,
        method_uri: &str, // e.g., "/helloworld.Greeter/SayHello"
        req_desc: MessageDescriptor,
        res_desc: MessageDescriptor,
        payload: DynamicMessage,
    ) -> Result<DynamicMessage, Status> {
        // Instantiate the generic gRPC multiplexer over the HTTP/2 channel
        let mut grpc = Grpc::new(self.channel.clone());

        let path = PathAndQuery::from_str(method_uri)
            .map_err(|_| Status::internal("Invalid method URI string"))?;

        let codec = DynamicCodec::new(req_desc, res_desc);

        // Ensure the channel has capacity and is ready to transmit
        grpc.ready()
            .await
            .map_err(|e| Status::internal(format!("Channel not ready: {}", e)))?;

        // Dispatch the request
        let response = grpc.unary(Request::new(payload), path, codec).await?;

        Ok(response.into_inner())
    }
}
