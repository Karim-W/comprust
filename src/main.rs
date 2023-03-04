pub mod cli;
pub mod compressor;

fn main() {
    let args = cli::Cli::new();
    let is_file: bool = args.source.is_file();
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    if is_file {
        paths.push(args.source);
    } else {
        let mut dir = std::fs::read_dir(args.source).unwrap();
        while let Some(Ok(entry)) = dir.next() {
            if entry.path().is_file() {
                paths.push(entry.path());
            }
        }
    }
    let compressor = compressor::Compress::new(paths, args.destination);
    compressor.compress();
}
