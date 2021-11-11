use clap::Parser;
use photorganizer::options::Options;
use photorganizer::scanner::Scanner;
use rayon::prelude::*;
use std::{path::{Path, PathBuf}, fs};

fn copy_threaded(entries: &Vec<PathBuf>, options: &Options) {
    entries.par_iter().for_each(|entry| {
        let new_path = photorganizer::generate_new_path(&entry, &options);
        fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        fs::copy(&entry, &new_path).unwrap();
    })
}

fn copy(entries: &Vec<PathBuf>, options: &Options) {
    entries.iter().for_each(|entry| {
        let new_path = photorganizer::generate_new_path(&entry, &options);
        fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        fs::copy(&entry, &new_path).unwrap();
    })
}

fn main() {
    let options = Options::parse();
    let scanner = Scanner::from_directory(&Path::new(&options.source));

    if options.parallel {
        copy_threaded(&scanner.entries, &options);
    } else {
        copy(&scanner.entries, &options);
    }
}
