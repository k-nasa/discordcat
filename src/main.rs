use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

use anyhow::Result;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

const CFG_FLAG: &str = "configure";

const CFG_FILE_PATH: &str = "~/.discordcat";

fn main() -> Result<()> {
    let app = build_app();

    let matches = app.get_matches();

    if matches.is_present(CFG_FLAG) {
        configure_discord_webhook();
        exit(0)
    }

    Ok(())
}

struct Setting {
    default_channel: String,
    channels: HashMap<String, String>,
}

impl Setting {
    fn new() -> Self {
        Setting {
            default_channel: String::new(),
            channels: HashMap::new(),
        }
    }

    fn default_channel(self, default_channel: String) -> Self {
        Setting {
            default_channel,
            channels: self.channels,
        }
    }

    fn channels(self, channels: HashMap<String, String>) -> Self {
        Setting {
            default_channel: self.default_channel,
            channels,
        }
    }
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

    let config_file_present = fs::File::open(CFG_FILE_PATH).is_ok();

    if !config_file_present {
        let mut f = fs::File::create(CFG_FILE_PATH)?;

        let mut channels = HashMap::new();
        channels.insert(channle_name.to_string(), webhook_url.to_string());

        let setting = Setting::new()
            .default_channel(channle_name.to_string())
            .channels(channels);

        // TODO output setting to toml file
        todo!()
    }

    // TODO read config toml file
    // TODO append new channel
    todo!();

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
