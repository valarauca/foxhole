use clap::{App, ArgMatches};

/// SubCommand descrbies an operation within foxhole framework
pub trait SubCommand {
    /// Build constructs a subcommand that will be used within the application
    fn build(&self) -> App<'static, 'static>;

    /// Name of the subcommand for matching/error handling
    fn name(&self) -> &'static str;

    /// Runs the subcommand
    fn exec(&self, args: &ArgMatches<'_>) -> Result<(), String>;
}
