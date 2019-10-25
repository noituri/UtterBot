use crate::command::{Command, CommandPerms, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;
use crate::database::models::Server;

pub struct AboutCommand;

impl Command for AboutCommand {
    fn name(&self) -> String {
        String::from("about")
    }

    fn desc(&self) -> String {
        String::from("shows information about this bot.")
    }

    fn enabled(&self) -> bool {
        true
    }

    fn use_in_dm(&self) -> bool {
        true
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        None
    }

    fn perms(&self) -> Option<Vec<CommandPerms>> {
        None
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, _: Option<Server>) -> Result<(), String> {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("About");
                e.description(r#"
                    **Created by:**
                    Mikołaj '[Noituri](https://github.com/noituri)' Radkowski

                    **Source code:**
                    Link -> [click](https://github.com/noituri/universalbot)
                    Discord Library -> [serenity](https://github.com/serenity-rs/serenity)

                    **Version:**
                    0.0.1 alpha
                "#);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }
}