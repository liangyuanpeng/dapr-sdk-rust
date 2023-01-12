#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::sleep(std::time::Duration::new(2, 0));

    // Get the Dapr port and create a connection
    // let port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
    let addr = format!("https://127.0.0.1:{}", 61506);

    // Create the client
    let mut client = dapr::Client::<dapr::client::TonicClient>::connect(addr).await?;

    let key = String::from("hello");

    // let val = String::from("world").into_bytes();

    let store_name = String::from("configstore");

    // save key-value pair in the state store
    let configResult =client.get_configuration(store_name, vec![(key)],None).await?;

    println!("Successfully saved!{:?}",configResult.items);

    Ok(())
}