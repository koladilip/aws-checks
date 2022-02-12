use crate::common::AWSError;
use aws_sdk_s3::{
    output::{
        GetBucketAclOutput, GetBucketLifecycleConfigurationOutput, GetBucketLoggingOutput,
        GetBucketPolicyOutput, GetBucketVersioningOutput, ListBucketsOutput,
    },
    Client,
};
use aws_types::config::Config;
use cached::proc_macro::cached;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    pub versioning: GetBucketVersioningOutput,
    pub logging: GetBucketLoggingOutput,
    pub acl: GetBucketAclOutput,
    pub policy: GetBucketPolicyOutput,
    pub lifecycle: GetBucketLifecycleConfigurationOutput,
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{String::from("s3_buckets")}"#
)]
async fn list_buckets(client: &Client) -> Result<ListBucketsOutput, AWSError> {
    client.list_buckets().send().await.map_err(AWSError::new)
}

async fn get_bucket_versioning(
    client: &Client,
    bucket_name: &String,
) -> Result<GetBucketVersioningOutput, AWSError> {
    client
        .get_bucket_versioning()
        .bucket(bucket_name)
        .send()
        .await
        .map_err(AWSError::new)
}

pub async fn list_buckets_versioning(
    client: &Client,
) -> Result<HashMap<String, GetBucketVersioningOutput>, AWSError> {
    let buckets_response = list_buckets(client).await?;
    if let Some(buckets) = buckets_response.buckets {
        let mut buckets_versioning = HashMap::new();
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            buckets_versioning.insert(
                bucket_name.clone(),
                get_bucket_versioning(client, &bucket_name).await?,
            );
        }
        Ok(buckets_versioning)
    } else {
        Ok(HashMap::new())
    }
}

async fn get_bucket_acl(
    client: &Client,
    bucket_name: &String,
) -> Result<GetBucketAclOutput, AWSError> {
    client
        .get_bucket_acl()
        .bucket(bucket_name)
        .send()
        .await
        .map_err(AWSError::new)
}

pub async fn list_bucket_acls(
    client: &Client,
) -> Result<HashMap<String, GetBucketAclOutput>, AWSError> {
    let buckets_response = list_buckets(client).await?;
    if let Some(buckets) = buckets_response.buckets {
        let mut bucket_acls = HashMap::new();
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            bucket_acls.insert(
                bucket_name.clone(),
                get_bucket_acl(client, &bucket_name).await?,
            );
        }
        Ok(bucket_acls)
    } else {
        Ok(HashMap::new())
    }
}

async fn get_bucket_policy(
    client: &Client,
    bucket_name: &String,
) -> Result<GetBucketPolicyOutput, AWSError> {
    let response = client.get_bucket_policy().bucket(bucket_name).send().await;
    match response {
        Ok(result) => Ok(result),
        Err(aws_sdk_s3::SdkError::ServiceError { err, .. })
            if err.code() == Some("NoSuchBucketPolicy") =>
        {
            Ok(GetBucketPolicyOutput::builder().build())
        }
        _ => response.map_err(AWSError::new),
    }
}

pub async fn list_bucket_polices(
    client: &Client,
) -> Result<HashMap<String, GetBucketPolicyOutput>, AWSError> {
    let buckets_response: ListBucketsOutput = list_buckets(client).await?;
    if let Some(buckets) = buckets_response.buckets {
        let mut bucket_policies = HashMap::new();
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            bucket_policies.insert(
                bucket_name.clone(),
                get_bucket_policy(client, &bucket_name).await?,
            );
        }
        Ok(bucket_policies)
    } else {
        Ok(HashMap::new())
    }
}

async fn get_bucket_logging(
    client: &Client,
    bucket_name: &String,
) -> Result<GetBucketLoggingOutput, AWSError> {
    client
        .get_bucket_logging()
        .bucket(bucket_name)
        .send()
        .await
        .map_err(AWSError::new)
}

pub async fn list_buckets_logging(
    client: &Client,
) -> Result<HashMap<String, GetBucketLoggingOutput>, AWSError> {
    let buckets_response: ListBucketsOutput = list_buckets(client).await?;
    if let Some(buckets) = buckets_response.buckets {
        let mut buckets_logging = HashMap::new();
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            buckets_logging.insert(
                bucket_name.clone(),
                get_bucket_logging(client, &bucket_name).await?,
            );
        }
        Ok(buckets_logging)
    } else {
        Ok(HashMap::new())
    }
}

async fn get_bucket_lifecycle_configuration(
    client: &Client,
    bucket_name: &String,
) -> Result<GetBucketLifecycleConfigurationOutput, AWSError> {
    let response = client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket_name)
        .send()
        .await;
    match response {
        Ok(result) => Ok(result),
        Err(aws_sdk_s3::SdkError::ServiceError { err, .. })
            if err.code() == Some("NoSuchLifecycleConfiguration") =>
        {
            Ok(GetBucketLifecycleConfigurationOutput::builder().build())
        }
        _ => response.map_err(AWSError::new),
    }
}

pub async fn list_buckets_lifecycle_configuration(
    client: &Client,
) -> Result<HashMap<String, GetBucketLifecycleConfigurationOutput>, AWSError> {
    let buckets_response = list_buckets(client).await?;
    if let Some(buckets) = buckets_response.buckets {
        let mut buckets_lifecycle = HashMap::new();
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            buckets_lifecycle.insert(
                bucket_name.clone(),
                get_bucket_lifecycle_configuration(client, &bucket_name).await?,
            );
        }
        Ok(buckets_lifecycle)
    } else {
        Ok(HashMap::new())
    }
}

pub async fn get_bucket_info(client: &Client, name: String) -> Result<Bucket, AWSError> {
    let (acl, policy, logging, versioning, lifecycle) = tokio::join!(
        get_bucket_acl(client, &name),
        get_bucket_policy(client, &name),
        get_bucket_logging(client, &name),
        get_bucket_versioning(client, &name),
        get_bucket_lifecycle_configuration(client, &name),
    );
    Ok(Bucket {
        name,
        acl: acl?,
        policy: policy?,
        logging: logging?,
        versioning: versioning?,
        lifecycle: lifecycle?,
    })
}

pub fn get_client(config: &Config) -> Client {
    Client::new(config)
}

pub async fn list_buckets_info(client: &Client) -> Result<Vec<Bucket>, AWSError> {
    let buckets_response = list_buckets(&client).await?;
    let mut buckets_info = Vec::new();

    if let Some(buckets) = buckets_response.buckets {
        for bucket_name in buckets
            .iter()
            .map(|bucket| bucket.name().unwrap().to_string())
        {
            buckets_info.push(get_bucket_info(&client, bucket_name).await?);
        }
    }
    Ok(buckets_info)
}
