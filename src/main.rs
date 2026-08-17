use tonic::transport::Server;

use crate::{
    echo_handler::{MyEchoService, echo::echo_service_server::EchoServiceServer},
    hello_handler::{MyHelloWorldService, hello_world::hello_service_server::HelloServiceServer},
};

pub mod echo_handler;
pub mod hello_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyEchoService::default();
    let hello_service = MyHelloWorldService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(service))
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}

// fn main() {
//     println!("helo")
// }
