use crate::identity::IAMIdentity;
use ansi_term::Colour::{Blue, Green};
use std::collections::HashMap;

pub fn output_identity_to_stdout(identity: IAMIdentity, accounts: HashMap<String, String>) {
    match (identity.user_id, identity.account, identity.arn) {
        (Some(user_id), Some(account_number), Some(arn)) => {
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
