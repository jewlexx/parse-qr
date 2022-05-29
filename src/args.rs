use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct Args {
    /// The path to the QR code image to be parsed
    pub file_path: OsString,
}

pub fn parse_args() -> anyhow::Result<Args> {
    let file_path = match std::env::args_os().next() {
        Some(arg) => arg,
        None => anyhow::bail!("No file path provided.\n"),
    };

    let args = Args { file_path };

    Ok(args)
}
