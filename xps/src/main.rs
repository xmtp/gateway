use anyhow::Result;
use clap::Parser;
use lib_xps::run;

#[derive(Parser, Debug)]
#[command(
    name = "xps",
    version = "0.1.0",
    about = "XMTP Postal Service"
)]
struct Args {
    #[arg(short = 'p', long = "port", default_value_t = 0)]
    port: u16,
    #[arg(short = 's', long = "host", default_value = "127.0.0.1")]
    host: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    crate::run(args.host, args.port).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_0() -> Result<()> {
        let arg_list = vec!["xps", "-p", "0"];
        let args = Args::parse_from(arg_list);
        assert_eq!(args.port, 0);
        Ok(())
    }

    #[test]
    fn test_port_25() -> Result<()> {
        let arg_list = vec!["xps", "--port", "25"];
        let args = Args::parse_from(arg_list);
        assert_eq!(args.port, 25);
        Ok(())
    }

    #[test]
    fn test_host_test_net() -> Result<()> {
        let arg_list = vec!["xps", "-s", "test.net"];
        let args = Args::parse_from(arg_list);
        assert_eq!(args.host, "test.net");
        Ok(())
    }

    #[test]
    fn test_host_test_0000() -> Result<()> {
        let arg_list = vec!["xps", "--host", "0.0.0.0"];
        let args = Args::parse_from(arg_list);
        assert_eq!(args.host, "0.0.0.0");
        Ok(())
    }

    #[test]
    fn test_default() -> Result<()> {
        let arg_list = vec!["xps"];
        let args = Args::parse_from(arg_list);
        assert_eq!(args.port, 0);
        assert_eq!(args.host, "127.0.0.1");
        Ok(())
    }
}