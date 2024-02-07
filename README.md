<div align="center" >
<h1>TikTok Live Rust</h1>

*Connect to TikTok live in 3 lines*

<div align="center" >

<a href="https://crates.io/crates/tokrust" target="blank" >
    <img src="https://img.shields.io/crates/v/tokrust.svg" >
</a>

<a href="https://discord.gg/the-crew" target="blank" >
    <img src="https://img.shields.io/discord/872886394358485012.svg?color=7289da&logo=discord" >
</a>

</div>
</div>

# Introduction
A Rust library. Use it to receive live stream events such as comments and gifts in realtime from [TikTok LIVE](https://www.tiktok.com/live) No credentials are required.

**NOTE:** This is not an official API. It's a reverse engineering project.

###### Fork From [jwdeveloper/TikTokLiveRust](https://github.com/jwdeveloper/TikTokLiveRust)
This project has been forked from https://github.com/jwdeveloper/TikTokLiveRust. All credit for the original work goes there.


#### Overview
- [Getting started](#getting-started)
- [Documentation](https://docs.rs/tokrust/latest/tokrust/)
- [Contributing](#contributing)

## Getting started
```toml
[dependencies]
tokrust = "0.0.1"
```

```rust
#[tokio::main]
async fn main() {
    let user_name = "el_capitano1988";
    let mut client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(handle_event)
        .build();

    client.connect().await;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim() == "stop"
    {
        //client.disconnect();
    }
}

fn handle_event(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    match event {
        TikTokLiveEvent::OnMember(joinEvent) =>
            {
                println!("user: {}  joined", joinEvent.raw_data.user.nickname);
            }
        TikTokLiveEvent::OnChat(chatEvent) =>
            {
                println!("user: {} -> {} ", chatEvent.raw_data.user.nickname, chatEvent.raw_data.content);
            }
        TikTokLiveEvent::OnGift(giftEvent) =>
            {
                let nick = &giftEvent.raw_data.user.nickname;
                let gift_name = &giftEvent.raw_data.gift.name;
                let gifts_amount = giftEvent.raw_data.gift.combo;

                println!("user: {} sends gift: {} x {}", nick, gift_name, gifts_amount);
            }
        _ => {}
    }
}

fn configure(settings: &mut TikTokLiveSettings)
{
    settings.http_data.time_out = Duration::from_secs(12);
}

```

## Contributing
Your improvements are welcome! Feel free to open an <a href="https://github.com/withRHEN/tokrust/issues">issue</a> or <a href="https://github.com/withRHEN/tokrust/pulls">pull request</a>.
