# discordcat

## Configuration

```console
$ discordcat --configure
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
$ echo hoge | ./target/debug/discordcat --webhook https://discordapp.com/api/webhooks/hoge/huga
```

```
OPTIONS:
        --username <username>
        --channel <channel>
```
