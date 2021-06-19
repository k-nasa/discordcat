# discordcat - CLI utility to post files and command output to discord

## Setup

```console
$ discordcat --setup
```

## Installing

```
wget https://github.com/k-nasa/discordcat/releases/download/0.1.0/discordcat_x86_64-unknown-linux-gnu.tar.gz
tar -xf discordcat_x86_64-unknown-linux-gnu.tar.gz
sudo mv ./discordcat_x86_64-unknown-linux-gnu/discordcat /usr/local/bin/
sudo chmod +x /usr/local/bin/discordcat
```

## Usage

send message

```console
$ echo "hello world" | discordcat
```

send file

```
$ discordcat -f README.md --filename readme
$ discordcat -f ./README.md
```


specify webhook url

```
$ echo hoge | discordcat --webhook https://discordapp.com/api/webhooks/hoge/huga
```

```
OPTIONS:
        --username <username>
        --channel <channel>
```
