use crate::AWSError;
use aws_sdk_iam::output::GetUserPolicyOutput;
use aws_sdk_iam::output::ListUsersOutput;
use aws_sdk_iam::Client;
use cached::proc_macro::cached;
use std::collections::HashMap;
use aws_types::config::Config;

#[cached(
    result = true,
    key = "String",
    convert = r#"{String::from("iam_users")}"#
)]
pub async fn list_users(client: &Client) -> Result<ListUsersOutput, AWSError> {
    client.list_users().send().await.map_err(AWSError::new)
}

async fn get_user_policy(
    client: &Client,
    user_name: &String,
) -> Result<GetUserPolicyOutput, AWSError> {
    let response = client.get_user_policy().user_name(user_name).send().await;
    match response {
        Ok(result) => Ok(result),
        Err(aws_sdk_s3::SdkError::ServiceError { err, .. })
            if err.code() == Some("ValidationError") =>
        {
            Ok(GetUserPolicyOutput::builder().build())
        }
        _ => response.map_err(AWSError::new),
    }
}

pub async fn list_user_policies(
    client: &Client,
) -> Result<HashMap<String, GetUserPolicyOutput>, AWSError> {
    let users_response = list_users(client).await?;
    if let Some(users) = users_response.users {
        let mut user_policies = HashMap::new();
        for user_name in users.iter().map(|u| u.user_name().unwrap().to_string()) {
            user_policies.insert(
                user_name.clone(),
                get_user_policy(client, &user_name).await?,
            );
        }
        Ok(user_policies)
    } else {
        Ok(HashMap::new())
    }
}

pub fn get_client(config: &Config) -> Client {
    Client::new(config)
}
