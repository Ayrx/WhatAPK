use anyhow::Result;
use clap::{App, AppSettings, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use zip::read::ZipArchive;

mod rules;

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("APK")
                .help("APK file to analyze.")
                .takes_value(true),
        )
        .get_matches();

    let apk = matches.value_of("APK").unwrap();

    let f = File::open(apk)?;
    let fr = BufReader::new(f);

    let zip = ZipArchive::new(fr)?;

    let files: HashSet<String> = zip.file_names().map(|x| x.to_owned()).collect();
    run_checks(files)?;

    Ok(())
}

fn run_checks(files: HashSet<String>) -> Result<()> {
    rules::vkey::check(&files);
    rules::reactnative::check(&files);
    rules::kony::check(&files);
    Ok(())
}

fn print_match(match_name: &str, matches: Vec<&String>) {
    println!("========================================");
    println!("[+] {} detected", match_name);

    println!("[+] Files:");
    for m in &matches {
        println!("{}", m);
    }
}
