use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

use hello_world::hello_service_server::HelloService;
use hello_world::{HelloRequest, HelloResponse};
#[derive(Debug, Default)]
pub struct MyHelloWorldService {}

#[tonic::async_trait]
impl HelloService for MyHelloWorldService {
    type SayHelloStreamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);
        let name = request.into_inner().name;
        let reply = HelloResponse {
            message: format!("Hello {}!", name),
        };
        Ok(Response::new(reply))
    }

    async fn say_hello_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
        let name = request.into_inner().name;

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let greetings = vec![
                format!("Hello, {}! (1/3)", name),
                format!("Hi Again, {}! (2,3)", name),
                format!("Greeatings, {}! (3/3", name),
            ];

            for greating in greetings {
                if tx
                    .send(Ok(HelloResponse { message: greating }))
                    .await
                    .is_err()
                {
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
