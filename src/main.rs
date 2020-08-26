use anyhow::Result;
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};
use std::process::exit;

const CFG_FLAG: &str = "configure";
const USERNAME_FLAG: &str = "username";
const CHANNEL_FLAG: &str = "channel";

#[tokio::main]
async fn main() -> Result<()> {
    let app = build_app();

    let matches = app.get_matches();

    if matches.is_present(CFG_FLAG) {
        configure_discord_webhook()?;
        exit(0)
    }

    let mut pipe_arg = String::new();
    std::io::stdin().read_to_string(&mut pipe_arg)?;

    if pipe_arg.ends_with("\n") {
        pipe_arg.remove(pipe_arg.len() - 1);
    }

    let setting: Setting = Setting::load_setting()?;

    let client = reqwest::Client::new();

    let username = matches.value_of(USERNAME_FLAG).unwrap_or_default();
    let msg = Msg {
        content: pipe_arg,
        username: username.to_string(),
    };

    let channel = if matches.is_present(CHANNEL_FLAG) {
        matches.value_of(CHANNEL_FLAG).unwrap()
    } else {
        setting.default_channel()
    };

    let resp = client
        .post(setting.channels.get(channel).unwrap())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&msg)?)
        .send()
        .await?;

    if resp.status() == 204 {
        println!("\x1b[01;32mSend message \"{}\"\x1b[m", msg.content);
    }

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
                .help("TODO")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(USERNAME_FLAG)
                .long("username")
                .help("TODO")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(CHANNEL_FLAG)
                .long("channel")
                .short("c")
                .help("TODO")
                .takes_value(true),
        )
}

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    default_channel: String,
    channels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub content: String,
    pub username: String,
}

impl Setting {
    fn new() -> Self {
        Setting {
            default_channel: String::new(),
            channels: HashMap::new(),
        }
    }

    fn default_channel(&self) -> &str {
        &self.default_channel
    }

    fn set_default_channel(self, default_channel: String) -> Self {
        Setting {
            default_channel,
            channels: self.channels,
        }
    }

    fn set_channels(self, channels: HashMap<String, String>) -> Self {
        Setting {
            default_channel: self.default_channel,
            channels,
        }
    }

    fn append_channel(self, k: String, v: String) -> Self {
        let mut channels = self.channels;
        channels.insert(k, v);

        Setting {
            default_channel: self.default_channel,
            channels,
        }
    }

    fn load_setting() -> Result<Self> {
        let mut f = fs::File::open(&get_config_path())?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let setting: Setting = toml::from_str(&s)?;

        Ok(setting)
    }
}

fn get_config_path() -> String {
    let home = env!("HOME");
    format!("{}/.discordcat", home)
}

fn configure_discord_webhook() -> Result<()> {
    let print_msg = |s| {
        print!("{}", s);
        io::stdout().flush().unwrap();
    };

    print_msg("nickname for channel:");
    let channle_name = read_line()?;

    print_msg("Please input webhook url:");
    let webhook_url = read_line()?;

    let config_file_present = std::path::Path::new(&get_config_path()).exists();

    if !config_file_present {
        let mut f = fs::File::create(&get_config_path())?;

        let mut channels = HashMap::new();
        channels.insert(channle_name.to_string(), webhook_url);

        let setting = Setting::new()
            .set_default_channel(channle_name)
            .set_channels(channels);

        write!(f, "{}", toml::to_string(&setting)?)?;
        f.flush()?;
    } else {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&get_config_path())?;

        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let setting: Setting = toml::from_str(&s)?;
        let setting = setting.append_channel(channle_name, webhook_url);

        f.seek(SeekFrom::Start(0)).unwrap();
        write!(f, "{}", toml::to_string(&setting)?)?;
        f.flush()?;
    }

    Ok(())
}

fn read_line() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
