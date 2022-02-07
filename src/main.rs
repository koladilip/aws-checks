use std::fmt::Debug;

use aws_sdk_s3::{Client, Error};
use aws_smithy_types::date_time::Format;
use clap::Parser;
use rust_aws_security_reporter::services::s3;
use rust_aws_security_reporter::utils::credentials;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the AWS region
    #[clap(short, long, default_value = "ap-south-1")]
    region: String,

    /// Name of the AWS region
    #[clap(short, long)]
    profile: String,
}

async fn show_buckets(client: &Client) -> Result<(), Error> {
    let resp = s3::buckets(client).await?;

    for bucket in resp.buckets().unwrap_or_default() {
        println!("Name:        {}", bucket.name().unwrap_or_default());
        println!("Created:     {:?}", bucket.creation_date().unwrap().fmt(Format::DateTime).unwrap());
        println!();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let region = args.region;
    let profile: String = args.profile;

    let shared_config = credentials::get_from_profile(region, profile).await;
    let client = Client::new(&shared_config);
    show_buckets(&client).await
}
