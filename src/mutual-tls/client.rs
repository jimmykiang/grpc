pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use pb::{echo_client::EchoClient, EchoRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_root_ca_cert = tokio::fs::read("../grpc/tls/ca.crt").await?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = tokio::fs::read("../grpc/tls/client.crt").await?;
    let client_key = tokio::fs::read("../grpc/tls/client.pem").await?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::from_static("http://[::1]:50051")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    let request= EchoRequest {
        message: "foo".into()
    };

    let mut stream = client
        .server_streaming_echo(request)
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("NOTE = {:?}", feature);
    }

    println!("Connected...now sleeping for 2 seconds...");

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Disconnect
    drop(stream);
    drop(client);

    println!("Disconnected...");

    Ok(())
}
