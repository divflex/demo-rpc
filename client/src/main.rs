use anyhow::Ok;
use rpc::HelloClient;
use tarpc::{client::Config, context, serde_transport::tcp};
use tokio_serde::formats::Json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let transport = tcp::connect("localhost:9322", Json::default).await?;

    let client = HelloClient::new(Config::default(), transport).spawn();

    let hi = client.say_hi(context::current()).await?;

    println!("{}", hi);

    Ok(())
}
