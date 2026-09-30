use krunker_rs::Client;
use std::{env, error::Error, io};

pub fn client_and_args(
    expected_args: usize,
    usage: &str,
) -> Result<(Client, Vec<String>), Box<dyn Error>> {
    let mut positional = Vec::new();
    let mut debug = false;
    for arg in env::args().skip(1) {
        if arg == "--debug" {
            debug = true;
        } else if arg.starts_with("--") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown flag {arg}. {usage}"),
            )
            .into());
        } else {
            positional.push(arg);
        }
    }

    let api_key = if positional.len() == expected_args + 1 {
        positional.remove(0)
    } else if positional.len() == expected_args {
        env::var("KRUNKER_API_KEY").map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Set KRUNKER_API_KEY or supply an API key as the first argument. {usage}"),
            )
        })?
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, usage).into());
    };

    Ok((Client::builder(api_key).debug(debug).build()?, positional))
}
