use url_shortener::db;
use url_shortener::run;

extern crate env_logger;

use clap::Arg;
use log::info;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_matches = clap::Command::new("url_shortener")
        .version("0.1.0")
        .about("A service that creates short URLs from long URLs")
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .help("port to listen on")
                .default_value("4000"),
        )
        .get_matches();

    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let pool = db::init_db(db::DATABASE_URL).await?;
    let port = cli_matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()?;

    info!("Starting server. port={}", port);

    run(pool, port).await?;

    Ok(())
}
