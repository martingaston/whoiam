use std::env;
mod identity;
mod io;
mod parse_toml;

#[tokio::main]
async fn main() {
    // if all 3 envs are set, aws call will be based on envs - else will read from profile
    // this is for some debug logs to determine how a user is logging in, totally useless but cool
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID");
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY");
    let aws_default_region = env::var("AWS_DEFAULT_REGION");

    let accounts = parse_toml::parse_whoiam();

    match identity::get_identity().await {
        Ok(identity) => io::output_identity_to_stdout(identity, accounts),
        Err(error) => {
            println!("unable to make aws getcalleridentity call");
            println!("the error message was: {}", error.to_string());
        }
    }
}
