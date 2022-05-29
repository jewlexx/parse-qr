mod args;

fn main() -> anyhow::Result<()> {
    let args = args::parse_args()?;

    let image = image::open(args.file_path)?.to_luma8();
    let mut qr = rqrr::PreparedImage::prepare(image);
    let grids = qr.detect_grids();

    if grids.is_empty() {
        anyhow::bail!("No grid detected.\n");
    }

    if grids.len() > 1 {
        println!("More than one grid. Will print all detected grids.\n");
    }

    for grid in grids {
        let (_, content) = grid.decode()?;

        println!("{}\n", content);
    }

    Ok(())
}
