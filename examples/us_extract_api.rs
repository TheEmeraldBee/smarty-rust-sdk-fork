extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::Options;
use smarty_rust_sdk::us_extract_api::client::USExtractClient;
use smarty_rust_sdk::us_extract_api::lookup::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        text: "Meet me at 3214 N University Ave Provo UT 84604 just after 3pm.".to_string(),
        aggressive: true,
        addresses_with_line_breaks: false,
        addresses_per_line: 1,
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(std::env::var("SMARTY_AUTH_ID")?, std::env::var("SMARTY_AUTH_TOKEN")?);

    let mut options = Options::default();
    options.license = "us-core-cloud".to_string();

    options.authentication = authentication;

    let client = USExtractClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.result)?);

    Ok(())
}