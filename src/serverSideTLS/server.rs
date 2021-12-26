pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use std::net::ToSocketAddrs;
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Identity, ServerTlsConfig};

use pb::{EchoRequest, EchoResponse};

#[derive(Debug)]
pub struct EchoServer {}

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {

    type ServerStreamingEchoStream = ReceiverStream<Result<EchoResponse, Status>>;

    async fn server_streaming_echo(
        &self,
        req: Request<EchoRequest>,
    ) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        println!("Client connected from: {:?}", req.remote_addr());

        println!("Message from Client: {:?}", req.into_inner().message);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {

            for _ in 0..4 {
                tx.send(Ok(EchoResponse {
                    message: format!("hello"),
                }))
                    .await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = tokio::fs::read("../grpc/tls/server.crt").await?;
    let key = tokio::fs::read("../grpc/tls/server.pem").await?;

    let identity = Identity::from_pem(cert, key);

    let server = EchoServer {};
    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(pb::echo_server::EchoServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
