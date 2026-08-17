use chrono::Utc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod echo {
    tonic::include_proto!("echo");
}
#[derive(Debug, Default)]
pub struct MyEchoService {}

use echo::echo_service_server::EchoService;
use echo::{EchoRequest, EchoResponse};

#[tonic::async_trait]
impl EchoService for MyEchoService {
    async fn ping(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let msg = request.into_inner().message;
        println!("Received unary ping: {}", msg);

        let reply = EchoResponse {
            reply: format!("Pong: {}", msg),
            timestamp: Utc::now().timestamp(),
        };

        Ok(Response::new(reply))
    }
    type StreamEchoStream = ReceiverStream<Result<EchoResponse, Status>>;

    async fn stream_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<Self::StreamEchoStream>, Status> {
        let msg = request.into_inner().message;
        println!("Received stream request: {}", msg);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 1..=6 {
                let reply = EchoResponse {
                    reply: format!("Stream msg {} for '{}'", i, msg),
                    timestamp: Utc::now().timestamp(),
                };

                if tx.send(Ok(reply)).await.is_err() {
                    break; // Client disconnected
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
