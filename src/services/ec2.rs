use std::collections::HashSet;

use crate::AWSError;
use aws_config::Config;
use aws_sdk_ec2::{
    model::{Image, Instance},
    output::DescribeImagesOutput,
    Client,
};
use cached::proc_macro::cached;

pub fn get_client(config: &Config) -> Client {
    Client::new(config)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{String::from("ec2_images")}"#
)]
pub async fn list_instances(client: &Client) -> Result<Vec<Instance>, AWSError> {
    let mut instances: Vec<Instance> = vec![];
    let mut next_token: Option<String> = None;
    loop {
        let resp = client
            .describe_instances()
            .set_next_token(next_token.clone())
            .send()
            .await
            .map_err(AWSError::new)?;
        for reservation in resp.reservations().unwrap_or_default() {
            for instance in reservation.instances().unwrap_or_default() {
                instances.push(instance.to_owned());
            }
        }
        next_token = resp.next_token().map(|a| a.to_string());
        if next_token.is_none() {
            break;
        }
    }
    Ok(instances)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{String::from("ec2_images")}"#
)]
pub async fn list_amis(client: &Client) -> Result<DescribeImagesOutput, AWSError> {
    client
        .describe_images()
        .owners("self")
        .send()
        .await
        .map_err(AWSError::new)
}

fn get_ami_ids_from_instances(instances: Vec<Instance>) -> HashSet<String> {
    instances
        .iter()
        .map(|instance| instance.image_id.as_ref())
        .filter(|image| image.is_some())
        .map(|image| image.unwrap().to_owned())
        .collect()
}
pub async fn get_unused_amis(client: &Client) -> Result<Vec<Image>, AWSError> {
    let instances = list_instances(client).await?;
    let used_ami_ids = get_ami_ids_from_instances(instances);
    let images = list_amis(client).await?.images.unwrap_or_default();
    let unused_images: Vec<Image> = images
        .iter()
        .filter(|image| !used_ami_ids.contains(image.image_id.as_ref().unwrap()))
        .map(|image| image.to_owned())
        .collect();
    Ok(unused_images)
}
