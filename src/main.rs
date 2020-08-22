use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

use anyhow::Result;

use std::io;
use std::io::Write;
use std::process::exit;

const CFG_FLAG: &str = "configure";

fn main() -> Result<()> {
    let app = build_app();

    let matches = app.get_matches();

    if matches.is_present(CFG_FLAG) {
        configure_discord_webhook();
        exit(0)
    }

    Ok(())
}

fn configure_discord_webhook() -> Result<()> {
    let mut input = String::new();

    let print_msg = |s| {
        print!("{}", s);
        io::stdout().flush().unwrap();
    };

    print_msg("nickname for channel:");
    let channle_name = io::stdin().read_line(&mut input)?;

    print_msg("Please input webhook url");
    let webhook_url = io::stdin().read_line(&mut input)?;

    Ok(())
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name(CFG_FLAG)
                .long("configure")
                .short("c")
                .help("TODO")
                .takes_value(false),
        )
}
