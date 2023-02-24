use anyhow::{Result, Context, Ok};
use hyper::{Body, Client};

#[tokio::main]
async fn main() -> Result<()> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    let client: Client<_, Body> = Client::builder().build(https);
    let url = "https://pokeapi.co/api/v2/pokemon/arceus"
        .parse()
        .context("Parsing URL")?;
    let resp = client.get(url)
        .await
        .context("Performing an HTTP request")?;
    println!("{:#?}", resp);

    Ok(())
}
