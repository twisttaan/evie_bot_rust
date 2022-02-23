// make a list of commands that are added by using a decorator

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub alias: Vec<String>,
    pub description: String,
    pub execute: fn(msg: &Message, http: &Arc<HttpClient>) -> Box<dyn Future<Output = ()>>,
}

// commands
const commands: Vec<Command> = vec![
    Command {
        name: "ping".to_owned(),
        alias: vec!["pong".to_owned()],
        description: "Pong!".to_owned(),
        execute: ping,
    },
    Command {
        name: "help".to_owned(),
        alias: vec!["h".to_owned()],
        description: "Help!".to_owned(),
        execute: help,
    },
];

pub fn find_command(name: &str) -> Option<Command> {
    for command in &commands.commands {
        if command.name == name || command.alias.contains(&name.to_owned()) {
            return Some(command.clone());
        }
    }
    None
}
