use std::fmt::Debug;

use clap::Parser;
use rust_aws_security_reporter::common::credentials;
use rust_aws_security_reporter::services::ec2;

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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let region = args.region;
    let profile: String = args.profile;

    let shared_config = credentials::get_from_profile(region, profile).await;
    // let iam_client = iam::get_client(&shared_config);
    // let s3_client = s3::get_client(&shared_config);
    let ec2_client = ec2::get_client(&shared_config);
    println!("{:#?}", ec2::get_unused_amis(&ec2_client).await.unwrap());
}
