use crate::identity::{IAMIdentity, PrincipalType};
use ansi_term::Colour::{Blue, Green};
use std::collections::HashMap;

pub fn output_identity_to_stdout(identity: IAMIdentity, accounts: HashMap<String, String>) {
    match (identity.principal_type, identity.account, identity.arn) {
        (principal_type, Some(account_number), Some(arn)) => {
            let user_id = match principal_type {
                PrincipalType::Account(account) => account.to_string(),
                PrincipalType::User(user) => user.to_string(),
                PrincipalType::FederatedUser(federated_user) => federated_user.to_string(),
                PrincipalType::AssumedRole(assumed_role) => assumed_role.to_string(),
            };
            println!("{} {}", Green.bold().paint("user id:"), user_id);
            match accounts.get(&account_number) {
                Some(account_name) => {
                    println!(
                        "{} {} ({})",
                        Green.bold().paint("account:"),
                        account_name,
                        Blue.paint(account_number)
                    );
                }
                None => println!("{} {}", Green.bold().paint("account:"), account_number),
            }
            println!("{} {}", Green.bold().paint("arn:"), arn);
        }
        (_, _, _) => panic!("AWS sent an unexpected response, bailing..."),
    };
}
