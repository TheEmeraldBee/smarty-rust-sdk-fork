use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_extract_api::extraction::ExtractionResult;
use crate::us_extract_api::lookup::Lookup;

pub struct USExtractClient {
    client: Client
}

impl USExtractClient {
    pub fn new(base_url: Url, options: Options) -> Result<USExtractClient, ParseError> {
        Ok(USExtractClient { client: Client::new(base_url, options, "")? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let req = self.client.reqwest_client.request(Method::POST, self.client.url.clone());

        let response = send_request(req).await?;

        let result = match response.json::<ExtractionResult>().await {
            Ok(result) => result,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        lookup.result = result;

        Ok(())
    }
}
