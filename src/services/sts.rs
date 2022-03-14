use aws_sdk_sts::{Client, output::GetCallerIdentityOutput};
use cached::proc_macro::cached;
use aws_types::config::Config;

use crate::AWSError;

pub fn get_client(config: &Config) -> Client {
    Client::new(config)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{String::from("sts_identity")}"#
)]
pub async fn get_identity(client: &Client) -> Result<GetCallerIdentityOutput, AWSError>{
    client.get_caller_identity().send().await.map_err(AWSError::new)
}