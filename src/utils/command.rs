/// Command struct to build a command with arguments.
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

impl Command {
    /// Create a new `Command` instance from the command name and list of arguments.
    pub fn build(command: &str, args: &[&str]) -> Self {
        let args = args.iter().map(|s| s.to_string()).collect();
        Self {
            command: command.to_string(),
            args,
        }
    }

    /// Get the full command string.
    pub fn command(&self) -> String {
        format!("{} {}", self.command, self.args.join(" "))
    }
}
