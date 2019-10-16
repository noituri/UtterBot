mod main;
mod moderation;

pub mod bot_modules {
    use crate::command::Command;

    pub trait BotModule {
        fn name(&self) -> String;
        fn desc(&self) -> String;
        fn enabled(&self) -> bool;
        fn commands(&self) -> Vec<Box<dyn Command>>;
    }

    pub fn get_modules() -> Vec<Box<dyn BotModule>> {
        vec! [
            Box::new(super::main::MainModule{})
        ]
    }

    pub fn find_module(name: &str) -> Result<Box<dyn BotModule>, String> {
        for m in get_modules() {
            if m.name() == name.to_lowercase() {
                return Ok(m);
            }
        }

        Err(String::from("Module does not exist!"))
    }
}