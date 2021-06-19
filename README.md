# discordcat - CLI utility to post files and command output to discord

[![Actions Status](https://github.com/k-nasa/discordcat/workflows/CI/badge.svg)](https://github.com/k-nasa/discordcat/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/discordcat.svg)](https://crates.io/crates/discordcat)

## DEMO

https://user-images.githubusercontent.com/23740172/122637416-757e2e00-d129-11eb-95c9-8769a80059bb.mov


## Installing


### using cargo
```
cargo install discordcat
```


### using brew

```
brew install k-nasa/tap/discordcat
```

### using wget

```
wget https://github.com/k-nasa/discordcat/releases/download/0.2.0/discordcat_x86_64-unknown-linux-gnu.tar.gz
tar -xf discordcat_x86_64-unknown-linux-gnu.tar.gz
sudo mv ./discordcat_x86_64-unknown-linux-gnu/discordcat /usr/local/bin/
sudo chmod +x /usr/local/bin/discordcat
```

## Setup

```bash
$ discordcat --setup
```

## Usage

### send message

```bash
$ echo "hello discord!!" | discordcat
```

### send file

```bash
$ discordcat -f ./README.md
$ discordcat -f README.md --filename readme
```


### specify webhook url

```bash
$ echo hello | discordcat --webhook https://discordapp.com/api/webhooks/hoge/huga
```

```bash
discordcat 0.2.0
CLI utility to post files and command output to discord

USAGE:
    discordcat [FLAGS] [OPTIONS]

FLAGS:
        --setup
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --username <username>
    -c, --channel <channel>
    -f, --file <file>
        --filename <filename>
        --webhook <webhook_url>
```

## Contribution

1. Fork it ( http://github.com/k-nasa/discordcat )
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## Licence

[MIT](https://github.com/k-nasa/discordcat/blob/master/LICENCE)

## Author

[k-nasa](https://github.com/k-nasa)
