use aws_sdk_iam::Client;
use std::fmt::Debug;

use clap::Parser;
use rust_aws_security_reporter::services::iam;
use rust_aws_security_reporter::common::credentials;

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

async fn show_users(client: &Client){
    let resp = iam::list_users(client).await;

    for user in resp.unwrap().users().unwrap_or_default() {
        println!("Name:        {}", user.user_name().unwrap_or_default());
        println!("UserID:     {:?}", user.user_id().unwrap_or_default());
        println!();
    }
}

async fn show_props(client: &Client){
    let resp = iam::list_user_policies(client).await;

    for (user_name, policy) in resp.unwrap() {
        println!("Name: {}", user_name);
        println!("Policy: {:?}", policy.policy_document());
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
    
    show_users(&client).await;
    show_props(&client).await;
}
