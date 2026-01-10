use crate::cli::Cli;

pub fn execute() {
    let markdown = clap_markdown::help_markdown::<Cli>();
    println!("{}", markdown);
}
