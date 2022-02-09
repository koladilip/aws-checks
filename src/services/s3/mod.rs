use std::collections::HashMap;
use aws_sdk_s3::{output::{ListBucketsOutput, GetBucketVersioningOutput}, Client};
use cached::proc_macro::cached;
use crate::common::AWSError;

#[cached(result = true, key = "String", convert = r#"{String::from("s3_buckets")}"#)]
pub async fn list_buckets(client: &Client) -> Result<ListBucketsOutput, AWSError> {
    client.list_buckets().send().await.map_err(AWSError::new)
}

#[cached(result = true, key = "String", convert = r#"{format!("s3_bucket_versioning_for_{}", bucket_name)}"#)]
async fn get_bucket_versioning(client: &Client, bucket_name: &String) -> Result<GetBucketVersioningOutput, AWSError> {
    client.get_bucket_versioning().bucket(bucket_name).send().await.map_err(AWSError::new)
}

#[cached(result = true, key = "String", convert = r#"{String::from("list_buckets_versioning")}"#)]
pub async fn list_buckets_versioning(client: &Client) -> Result<HashMap<String, GetBucketVersioningOutput>, AWSError>{
    let buckets_response: ListBucketsOutput = list_buckets(client).await?;
    
    if let Some(buckets) = buckets_response.buckets {
        let mut buckets_versioning = HashMap::new();
        for bucket_name in buckets.iter().map(|bucket| bucket.name().unwrap().to_string()) {
            buckets_versioning.insert(bucket_name.clone(), get_bucket_versioning(client, &bucket_name).await?);
        }
        Ok(buckets_versioning)
    } else {
        Ok(HashMap::new())
    }    
}