use ansi_term::Colour::{Blue, Red};
use aws_config;
use aws_sdk_sts::{error::GetCallerIdentityError, Client, SdkError};
use std::fmt;

pub struct IAMIdentity {
    pub principal_type: PrincipalType,
    pub account: Option<String>,
    pub arn: Option<String>,
}

pub async fn get_identity() -> Result<IAMIdentity, SdkError<GetCallerIdentityError>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let identity = client.get_caller_identity();

    return match identity.send().await {
        Ok(identity) => Ok(IAMIdentity {
            principal_type: get_principal_type_from_user_id(&identity.user_id.unwrap()),
            account: identity.account,
            arn: identity.arn,
        }),
        Err(error) => Err(error),
    };
}

pub fn get_principal_type_from_user_id(user_id: &str) -> PrincipalType {
    if user_id.contains(":") {
        let split: Vec<&str> = user_id.split(":").collect();

        if split[0].len() == 12 {
            PrincipalType::FederatedUser(FederatedUser {
                account: split[0].to_string(),
                caller_specified_role_name: split[1].to_string(),
            })
        } else {
            PrincipalType::AssumedRole(AssumedRole {
                role_id: split[0].to_string(),
                caller_specified_role_name: split[1].to_string(),
            })
        }
    } else {
        if user_id.len() == 12 {
            PrincipalType::Account(Account {
                account_id: user_id.to_string(),
            })
        } else {
            PrincipalType::User(User {
                unique_id: user_id.to_string(),
            })
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PrincipalType {
    Account(Account),
    User(User),
    FederatedUser(FederatedUser),
    AssumedRole(AssumedRole),
}

#[derive(PartialEq, Debug)]
pub struct Account {
    account_id: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.account_id,
            Red.bold().paint("Root Account!")
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct User {
    unique_id: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.unique_id, Blue.paint("User"))
    }
}

#[derive(PartialEq, Debug)]
pub struct FederatedUser {
    account: String,
    caller_specified_role_name: String,
}

impl fmt::Display for FederatedUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}))", self.account, Blue.paint("FederatedUser"))
    }
}

#[derive(PartialEq, Debug)]
pub struct AssumedRole {
    role_id: String,
    caller_specified_role_name: String,
}

impl fmt::Display for AssumedRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}))", self.role_id, Blue.paint("AssumedRole"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test for root user 123456789102
    #[test]
    fn get_principal_type_from_user_id_detects_iam_user() {
        let iam_user = "AIDAXXXXXXXXXXXXXXXXX";
        let expected = PrincipalType::User(User {
            unique_id: "AIDAXXXXXXXXXXXXXXXXX".to_string(),
        });

        let result = get_principal_type_from_user_id(iam_user);

        assert_eq!(result, expected);
    }

    #[test]
    fn get_principal_type_from_user_id_detects_role() {
        let iam_user = "AROAXXXXXXXXXXXXXXXXX:timmy.test";
        let expected = PrincipalType::AssumedRole(AssumedRole {
            role_id: "AROAXXXXXXXXXXXXXXXXX".to_string(),
            caller_specified_role_name: "timmy.test".to_string(),
        });

        let result = get_principal_type_from_user_id(iam_user);

        assert_eq!(result, expected);
    }

    #[test]
    fn get_principal_type_from_user_id_detects_federated_user() {
        let iam_user = "123456789012:timmy.test";
        let expected = PrincipalType::FederatedUser(FederatedUser {
            account: "123456789012".to_string(),
            caller_specified_role_name: "timmy.test".to_string(),
        });

        let result = get_principal_type_from_user_id(iam_user);

        assert_eq!(result, expected);
    }

    #[test]
    fn get_principal_type_from_user_id_detects_root_user() {
        let iam_user = "123456789012";
        let expected = PrincipalType::Account(Account {
            account_id: "123456789012".to_string(),
        });

        let result = get_principal_type_from_user_id(iam_user);

        assert_eq!(result, expected);
    }
}
