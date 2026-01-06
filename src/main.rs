mod cli;

#[tokio::main]
async fn main() {
  let args = cli::Args::parse();
  dbg!(&args);
}
