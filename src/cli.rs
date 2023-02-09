use anyhow::{anyhow, Result};
use getopts::Options;
use std::env;

pub struct Settings {
    pub redis_host: String,
    pub enable_connection_pool: bool,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse_cli() -> Result<Option<Settings>> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("r", "redis-host", "host running Redis", "NAME");
    opts.optflag(
        "",
        "enable-connection-pool",
        "enable connection pool - false by default",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => return Err(anyhow!("error parsing cli flags: {:?}", e)),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(None);
    }

    let redis_host = matches
        .opt_str("r")
        .ok_or_else(|| anyhow!("The redis connection parameter must be provided"))?;
    if !matches.free.is_empty() {
        print_usage(&program, opts);
        return Err(anyhow!("Unknown args: {:?}", matches.free));
    };

    let enable_connection_pool = matches.opt_present("enable-connection-pool");

    Ok(Some(Settings {
        redis_host,
        enable_connection_pool,
    }))
}
