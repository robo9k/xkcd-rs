use clap::Parser as _;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[clap(parse(try_from_str))]
    num: Option<xkcd::ComicNumber>,
}

impl Cli {
    fn comic_id(&self) -> xkcd::ComicId {
        self.num
            .map_or_else(|| xkcd::ComicId::Current, |id| id.into())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    let comic_id = cli.comic_id();
    println!("comic id {:?}", comic_id);

    Ok(())
}
