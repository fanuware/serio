use clap::Parser;

const VERSION: &str = env!("VERGEN_GIT_DESCRIBE");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "Serial device communication utility")]
pub struct Args {
    #[clap(short, long)]
    pub device: Option<String>,

    #[clap(short, long, default_value_t = 115200)]
    pub baudrate: u32,
}

impl Args {
    pub fn parse_or_exit() -> Self {
        match Args::try_parse() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
