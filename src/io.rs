use toml::Value;

use crate::identity::IAMIdentity;

// TODO: this feels uggo
pub fn output_identity_to_stdout(identity: IAMIdentity, accounts: Value) {
    println!("user id: {}", identity.user_id.unwrap_or("".to_string()));
    match identity.account {
        Some(account_number) => match accounts.get(&account_number) {
            Some(account_name) => {
                println!(
                    "account: {} ({})",
                    account_name.as_str().unwrap(),
                    account_number
                )
            }
            None => println!("account: {}", account_number),
        },
        None => (),
    }
    println!("arn: {}", identity.arn.unwrap_or("".to_string()));
}
