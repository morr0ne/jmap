use clap::{App, Arg};
use jmap::ClientBuilder;
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
    let email = matches.value_of("email").unwrap();
    let password = matches.value_of("password").unwrap();

    let mut client = ClientBuilder::new().build()?;
    let session = client.auth(server, email, password).await?;

    let res = client.echo(&session).await?;
    dbg!(res);

    fs::write("temp/jmap.json", serde_json::to_vec(&session)?).await?;

    Ok(())
}
