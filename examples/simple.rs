use clap::{App, Arg};
use jmap::{client::Auth, ClientBuilder};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = App::new("jmap")
        .arg(Arg::new("server").required(true))
        .arg(Arg::new("email").required(true))
        .arg(Arg::new("password").required(true))
        .get_matches();

    let server = matches.value_of("server").unwrap();
    let email = matches.value_of("email").unwrap().to_string();
    let password = matches.value_of("password").unwrap().to_string();

    let mut client = ClientBuilder::new().build()?;
    let session = client
        .auth(server, Auth::Basic(email, Some(password)))
        .await?;

    let res = client.mailbox(&session).await?;
    // dbg!(&res);

    fs::write("temp/jmap.json", serde_json::to_vec(&res)?).await?;

    Ok(())
}
