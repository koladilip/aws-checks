use aws_sdk_s3::Client;
use std::fmt::Debug;

use aws_smithy_types::date_time::Format;
use clap::Parser;
use rust_aws_security_reporter::services::s3;
use rust_aws_security_reporter::common::credentials;

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

async fn show_buckets(client: &Client){
    let resp = s3::list_buckets(client).await;

    for bucket in resp.unwrap().buckets().unwrap_or_default() {
        println!("Name:        {}", bucket.name().unwrap_or_default());
        println!("Created:     {:?}", bucket.creation_date().unwrap().fmt(Format::DateTime).unwrap());
        println!();
    }
}

async fn show_bucket_props(client: &Client){
    let resp = s3::list_buckets_lifecycle_configuration(client).await;

    for (bucket_name, lifecycle) in resp.unwrap() {
        println!("Name: {}", bucket_name);
        println!("Rules: {:?}", lifecycle.rules());
        println!();
    }
}

#[tokio::main]
async fn main(){
    let args = Args::parse();
    let region = args.region;

    let profile: String = args.profile;

    let shared_config = credentials::get_from_profile(region, profile).await;
    let client = Client::new(&shared_config);
    
    show_buckets(&client).await;
    show_bucket_props(&client).await;
}
