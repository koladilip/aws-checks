use std::collections::HashMap;
use aws_sdk_s3::{output::{ListBucketsOutput, GetBucketVersioningOutput, GetBucketAclOutput, GetBucketPolicyOutput}, Client};
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

#[cached(result = true, key = "String", convert = r#"{format!("s3_bucket_acl_for_{}", bucket_name)}"#)]
async fn get_bucket_acl(client: &Client, bucket_name: &String) -> Result<GetBucketAclOutput, AWSError> {
    client.get_bucket_acl().bucket(bucket_name).send().await.map_err(AWSError::new)
}

#[cached(result = true, key = "String", convert = r#"{String::from("list_bucket_acls")}"#)]
pub async fn list_bucket_acls(client: &Client) -> Result<HashMap<String, GetBucketAclOutput>, AWSError>{
    let buckets_response: ListBucketsOutput = list_buckets(client).await?;
    
    if let Some(buckets) = buckets_response.buckets {
        let mut bucket_acls = HashMap::new();
        for bucket_name in buckets.iter().map(|bucket| bucket.name().unwrap().to_string()) {
            bucket_acls.insert(bucket_name.clone(), get_bucket_acl(client, &bucket_name).await?);
        }
        Ok(bucket_acls)
    } else {
        Ok(HashMap::new())
    }    
}

#[cached(result = true, key = "String", convert = r#"{format!("s3_bucket_policy_for_{}", bucket_name)}"#)]
async fn get_bucket_policy(client: &Client, bucket_name: &String) -> Result<GetBucketPolicyOutput, AWSError> {
    let response = client.get_bucket_policy().bucket(bucket_name).send().await;
    match response {
        Ok(result) => Ok(result),
        Err(aws_sdk_s3::SdkError::ServiceError { err, ..})  if err.code() == Some("NoSuchBucketPolicy") =>  {
            Ok(GetBucketPolicyOutput::builder().build())
        }
        _ => response.map_err(AWSError::new)
    }
}

#[cached(result = true, key = "String", convert = r#"{String::from("list_bucket_polices")}"#)]
pub async fn list_bucket_polices(client: &Client) -> Result<HashMap<String, GetBucketPolicyOutput>, AWSError>{
    let buckets_response: ListBucketsOutput = list_buckets(client).await?;
    
    if let Some(buckets) = buckets_response.buckets {
        let mut bucket_policies = HashMap::new();
        for bucket_name in buckets.iter().map(|bucket| bucket.name().unwrap().to_string()) {
            bucket_policies.insert(bucket_name.clone(), get_bucket_policy(client, &bucket_name).await?);
        }
        Ok(bucket_policies)
    } else {
        Ok(HashMap::new())
    }    
}