use super::BotModule;
use crate::command::Command;

mod ban_command;
mod unban_command;
mod kick_command;

pub struct ModerationModule;

impl BotModule for ModerationModule {
    fn name(&self) -> String {
        String::from("moderation")
    }

    fn desc(&self) -> String {
        String::from("Moderation commands.")
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(ban_command::BanCommand {}),
            Box::new(unban_command::UnBanCommand {}),
            Box::new(kick_command::KickCommand {}),
        ]
    }
}
