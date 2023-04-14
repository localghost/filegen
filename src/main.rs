use anyhow::Result;
use anyhow::ensure;
use clap::Parser;
use rayon::ThreadPoolBuilder;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[clap(help = "Directory to generate the files into")]
    dir: PathBuf,

    #[clap(long, value_parser, help = "Size of individual file")]
    file_size: u32,

    #[clap(long, value_parser, help = "Number of files to generate")]
    file_num: u32,
}

fn create_files(basedir: PathBuf, num_files: u32, file_size: u32) -> Result<()> {
    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new()
        .num_threads(std::cmp::max(cpus / 2, 1))
        .build()?;
    pool.scope(|_scope| {
        (0..num_files).for_each(|idx| create_file(basedir.join(format!("file_{}", idx)), file_size).unwrap() );
    });
    Ok(())
}

fn create_file(path: PathBuf, file_size: u32) -> Result<()> {
    let rng = thread_rng();
    std::fs::write(
        path,
        rng.sample_iter(Alphanumeric)
            .take(file_size as usize)
            .collect::<Vec<_>>(),
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(args.dir.exists(), "Path {:?} does not exist", args.dir);
    ensure!(args.dir.is_dir(), "Path {:?} is not a directory", args.dir);

    create_files(args.dir, args.file_num, args.file_size)?;

    Ok(())
}
