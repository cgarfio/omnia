use std::{
  net::{IpAddr, SocketAddr},
  num::NonZeroU16,
};

#[derive(clap::Parser, Debug)]
#[command(author, disable_help_subcommand = true, propagate_version = true, version)]
pub struct Args {
  #[arg(
    long = "bind",
    env = "BIND",
    value_name = "BIND",
    default_value = "127.0.0.1:8080",
    help_heading = "Networking"
  )]
  pub socket: SocketAddr,

  #[arg(long, env = "ADDRESS", help_heading = "Networking")]
  ip: Option<IpAddr>,

  #[arg(long, env = "PORT", help_heading = "Networking")]
  port: Option<NonZeroU16>,
}

impl Args {
  pub fn parse() -> Self {
    use clap::Parser;

    let mut args = Self::parse_from(std::env::args_os());

    args.effective_socket();

    args
  }

  fn effective_socket(&mut self) {
    let ip = self.ip.unwrap_or(self.socket.ip());
    let port = self.port.map(|it| it.get()).unwrap_or(self.socket.port());

    self.socket.set_ip(ip);
    self.socket.set_port(port);
  }
}
