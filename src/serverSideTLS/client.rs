pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use pb::{echo_client::EchoClient, EchoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pem = tokio::fs::read("../grpc/tls/ca.crt").await?;
    let ca = Certificate::from_pem(pem);
    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("localhost");

    // let mut client = EchoClient::connect("http://[::1]:50051").await.unwrap();

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
