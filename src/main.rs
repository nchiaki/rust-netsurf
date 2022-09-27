use std::env;
use std::fs;
use hyper_tls::{HttpsConnector, TlsStream};
use hyper::{Client, Body, Method, Request, Uri};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

mod help;
mod letssurf;
mod httpio;


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>>
{
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut prmtbl = help::ParamMap::new();

    prmtbl = match help::parse_argv(argc, argv, prmtbl)
        {
            Ok(v) => v,
            Err(_) => return Ok(()),
        };

    if prmtbl.is_data_save()
    {
        let metadata = match fs::metadata(help::get_data_dir())
        {
            Ok(v) => v,
            Err(e) =>
            {
                println!("{}: {}", e, help::get_data_dir());
                return Ok(());
            },
        };
        //println!("{:?}", metadata);
        if ! metadata.is_dir()
        {
            println!("{} is not directory.", help::get_data_dir());
            return Ok(());
        }
    }

    letssurf::lets_surf(prmtbl).await?;

    Ok(())
}
