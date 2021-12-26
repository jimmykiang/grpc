pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use pb::{EchoRequest, EchoResponse};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct EchoServer;

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
    let server_identity = Identity::from_pem(cert, key);

    let client_ca_cert = tokio::fs::read("../grpc/tls/ca.crt").await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    let addr = "[::1]:50051".parse().unwrap();
    let server = EchoServer::default();

    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        .tls_config(tls)?
        .add_service(pb::echo_server::EchoServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
