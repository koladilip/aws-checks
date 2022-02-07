use std::future::Future;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_types::region::Region;

/// Get Credentials AWS Profile
pub fn get_from_profile(
    region: String,
    profile: String) -> impl Future<Output = aws_config::Config> {
    let profile_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile)
        .build();

    aws_config::from_env()
        .credentials_provider(profile_provider)
        .region(Region::new(region))
        .load()
}
