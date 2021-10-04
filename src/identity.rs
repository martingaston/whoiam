use aws_config;
use aws_sdk_sts::{error::GetCallerIdentityError, Client, SdkError};
pub struct IAMIdentity {
    pub user_id: Option<String>,
    pub account: Option<String>,
    pub arn: Option<String>,
}

pub async fn get_identity() -> Result<IAMIdentity, SdkError<GetCallerIdentityError>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let identity = client.get_caller_identity();

    return match identity.send().await {
        Ok(identity) => Ok(IAMIdentity {
            user_id: identity.user_id,
            account: identity.account,
            arn: identity.arn,
        }),
        Err(error) => Err(error),
    };
}
