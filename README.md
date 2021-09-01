# **Robo Arc**
Learning Rust project. Yes, I'm very original and full of ideas, so I'm remaking what I already did with python, but hopefully better this time.

[Bot invite link!](https://discord.com/api/oauth2/authorize?client_id=551759974905151548&scope=bot+applications.commands&permissions=808971478)

![Bot profile picture](PFP.png "Bot's profile picture")

This bot is made using [serenity.rs](https://github.com/serenity-rs/serenity/) (currently using the development release [serenity.await](https://github.com/Lakelezz/serenity/blob/await/)), an async discord api wrapper for [Rust](https://www.rust-lang.org/).

## __Running the bot for yourself__

### __**Get the source and prepare it**__:

```bash
git clone git@gitlab.com:vicky5124/robo-arc.git # Over SSH
git clone https://gitlab.com/vicky5124/robo-arc.git # Over HTTPS

cd robo-arc
mv config.toml.example config.toml
```

### __**Discord token**__:

First, create an application [Here](https://discordapp.com/developers/applications/).
\
Go to the newly created application and head over the `Bot` tab.
\
In here you create a bot, enable the Server Member Intent, and copy the token. The token is what will be put on the `discord` variable inside `config.toml`.
\
> You can also create the invite link for the bot on the OAuth2 tab; Just select bot, the permissions you want the bot to have and copy the invite link.

### __**Other tokens**__:

- __osu!__
\
    Obtain the token [HERE](https://osu.ppy.sh/p/api/)
\
    NOTE: You will need to create an account if you do not already have one.
\
    NOTE: Do not create more than 1 account in the case of already having one, it's a bannable offence.

- __Sankaku__
\
    Create an account [HERE](https://idol.sankakucomplex.com/user/signup).
\
    Obtain the "passhash" from the location [THIS](https://forum.sankakucomplex.com/t/channel-api-for-discord-integration/2204/7) image shows.
\
    NOTE: The image shows the Chrome Dev Tools (f12), On firefox the tab is called "Storage", same key.


### __**PostgreSQL Database**__:

You'll need to have a psql server running. If you don't know how, I recommend using docker. [Here's](https://www.youtube.com/watch?v=aHbE3pTyG-Q) a video that will help you with that.

    NOTE: if you are using windows, docker requires the hyper-V module to be enabled, which breaks other virtualization software like VirtualBox or VMWare. If you use any of those softwares, consider setting the database on the system natively.

With a created database and you connected with a user, you'll need to create different tables, required by the bot.
\
You can do this by using sqlx migrations. To install it, run this command:
\
`cargo install sqlx-cli --git https://github.com/launchbadge/sqlx.git`
\
Then you will need to set a database url to your env_vars. In linux you can run this:
\
```bash
export DATABASE_URL="postgres://`postgres_username`:`postgres_user_password`@`postgres_host`:`postgres_port`/`postgres_database`"
export DATABASE_URL2=$DATABASE_URL
```
\
Followed with the creation of the database:
```bash
# Only run this the first time.
sqlx database create

# Apply the migrations
# Run this every time you update the bot.
cargo sqlx migrate run
```

### __**Redis Database**__:
So logging works, redis is also necessary. It is also recommended that you use docker for this.
[Here's](https://hub.docker.com/_/redis) the docker hub link. The readme has the commands you need to run the container.

    NOTE: This is currently only tested without a configuration, if any configuration options that you feel like you may need make stuff not work, feel free to open an issue.

Then you will need to set a database url to your env_vars. In linux you can run this:
\
```bash
export REDIS_URL="`redis_host`:`redis_port`"
```

### __**Eval command**__:
If you'd like to have an eval command, to evaluate python code within discord, you will need to use the `basic_python_bot_for_eval.py` file.

To run it, run the following commands:
```
# only once
Install python 3.8.5 or newer.
Install pipenv with `python3 -m pip install pipenv -U --user`

# run every update
`python3 -m pipenv sync`

# run to run the eval bot
python3 -m pipenv run python basic_python_bot_for_eval.py
```

### __**Running the bot**__:

It's as simple as just running:

```bash
export DATABASE_URL="postgres://`postgres_username`:`postgres_user_password`@`postgres_ip`:`postgres_port`/`postgres_database`"
# example: postgres://postgres:123456@localhost:5432/arcbot
# these values are the same as the ones you put on config.toml
cargo run --release
```

## Notes

- To get the idol command working, you *may* need to run this command first:
> `curl -H "\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\x14\x00U\x00s\x00e\x00r\x00-\x00A\x00g\x00e\x00n\x00t\x00\x00\x00\n\x00\x00\x00ä\x00M\x00o\x00z\x00i\x00l\x00l\x00a\x00/\x005\x00.\x000\x00 \x00(\x00W\x00i\x00n\x00d\x00o\x00w\x00s\x00 \x00N\x00T\x00 \x001\x000\x00.\x000\x00;\x00 \x00W\x00i\x00n\x006\x004\x00;\x00 \x00x\x006\x004\x00)\x00 \x00A\x00p\x00p\x00l\x00e\x00W\x00e\x00b\x00K\x00i\x00t\x00/\x005\x003\x007\x00.\x003\x006\x00 \x00(\x00K\x00H\x00T\x00M\x00L\x00,\x00 \x00l\x00i\x00k\x00e\x00 \x00G\x00e\x00c\x00k\x00o\x00)\x00 \x00C\x00h\x00r\x00o\x00m\x00e\x00/\x006\x007\x00.\x000\x00.\x003\x003\x009\x006\x00.\x008\x007\x00 \x00S\x00a\x00f\x00a\x00r\x00i\x00/\x005\x003\x007\x00.\x003\x006" https://iapi.sankakucomplex.com/post/index.json\?page=1\&limit=1\&tags=rating:safe`
