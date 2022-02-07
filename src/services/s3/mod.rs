use std::future::Future;
use aws_sdk_s3::{Client, SdkError, output::ListBucketsOutput, error::ListBucketsError};

pub fn buckets(client: &Client) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>> {
    client.list_buckets().send()
}