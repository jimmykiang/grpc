pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use pb::{echo_client::EchoClient, EchoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:50051").await.unwrap();

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
