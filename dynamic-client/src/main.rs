mod client;
mod codec;
mod reflection;

use client::DynamicClient;
use prost_reflect::DynamicMessage;
use reflection::fetch_descriptor_pool;
use serde_json::json;
use tonic::transport::Endpoint;

use crate::reflection::list_services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    println!("Fetching schemas via Server Reflection...");
    let services_list = list_services(channel.clone()).await?;
    // .map_err(|e| e.to_string());
    println!("{:?}", services_list);

    for serv in services_list {
        let pool = fetch_descriptor_pool(channel.clone(), &serv).await?;
        let service = pool
            .get_service_by_name(&serv)
            // .get_method_by_name("helloworld.Greeter.SayHello")
            .unwrap();
        for method in service.methods() {
            println!("{:?}", method.input());
            break;
        }
    }
    // Fetch schema by querying the service symbol
    // let pool = fetch_descriptor_pool(channel.clone(), "hello_world.HelloService").await?;
    // println!("{:?}", pool);

    // // Extract definitions just like before
    // let service = pool
    //     .get_service_by_name("hello_world.HelloService")
    //     // .get_method_by_name("helloworld.Greeter.SayHello")
    //     .unwrap();

    // let method = service
    //     .methods()
    //     .find(|m| m.name() == "SayHello")
    //     .expect("Method 'SayHello' not found on service 'helloworld.Greeter'");
    // let req_desc = method.input();
    // let res_desc = method.output();

    // let json_payload = json!({ "name": "Rust Reflection Client" });
    // // let mut deserializer = serde_json::Deserializer::from_value(json_payload);
    // let message = DynamicMessage::deserialize(req_desc.clone(), json_payload)?;

    // let client = DynamicClient::new(channel);
    // let method_path = format!("/{}/{}", service.full_name(), method.name());

    // let response = client
    //     .unary_call(&method_path, req_desc, res_desc, message)
    //     .await?;

    // println!("Response: {}", serde_json::to_value(&response)?);

    Ok(())
}
