use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppEnv {
    /// server listen address
    #[arg(short, long, default_value = "127.0.0.1:10085")]
    pub addr: String,

    /// graceful shutdown wait time in seconds
    #[arg(short, long, value_parser = parse_duration, default_value = "5")]
    pub wait_time: std::time::Duration,

    #[arg(short, long)]
    pub config: Option<String>,

    /// connect info
    #[arg(long, default_value = "127.0.0.1")]
    pub redis_host: String,

    /// connect info
    #[arg(long, default_value_t = 6379)]
    pub redis_port: u16,

    /// db num
    #[arg(long, default_value_t = 0)]
    pub redis_db: u32,

    /// auth user
    #[arg(long)]
    pub redis_user: Option<String>,

    /// auth passwd
    #[arg(long)]
    pub redis_passwd: Option<String>,

    /// connect info
    #[arg(long)]
    pub db_addr: Option<String>,

    /// db name
    #[arg(long)]
    pub db_name: Option<String>,

    /// user info
    #[arg(long)]
    pub db_user: Option<String>,

    /// passwd
    #[arg(long)]
    pub db_passwd: Option<String>,
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let seconds = arg.parse()?;
    Ok(std::time::Duration::from_secs(seconds))
}
