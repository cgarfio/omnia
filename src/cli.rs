#[derive(clap::Parser, Debug)]
#[command(author, disable_help_subcommand = true, propagate_version = true, version)]
pub struct Args;

impl Args {
  pub fn parse() -> Self {
    use clap::Parser;

    Self::parse_from(std::env::args_os())
  }
}
