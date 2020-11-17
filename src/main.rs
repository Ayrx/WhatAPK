use anyhow::Result;
use clap::{App, AppSettings, Arg};
use serde::{Deserialize, Serialize};
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
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Write results to a JSON file.")
                .takes_value(true),
        )
        .get_matches();

    let apk = matches.value_of("APK").unwrap();

    let f = File::open(apk)?;
    let fr = BufReader::new(f);

    let zip = ZipArchive::new(fr)?;

    let files: HashSet<String> = zip.file_names().map(|x| x.to_owned()).collect();
    let results = run_checks(files)?;

    for r in &results {
        r.print_results();
    }

    if let Some(o) = matches.value_of("OUTPUT") {
        let file = File::create(o)?;
        serde_json::to_writer(file, &results)?;
    }

    Ok(())
}

fn run_checks(files: HashSet<String>) -> Result<Vec<CheckResults>> {
    let mut results = Vec::new();

    if let Some(c) = rules::vkey::check(&files) {
        results.push(c);
    }

    if let Some(c) = rules::reactnative::check(&files) {
        results.push(c);
    }

    if let Some(c) = rules::kony::check(&files) {
        results.push(c);
    }

    if let Some(c) = rules::rootbeer::check(&files) {
        results.push(c);
    }

    Ok(results)
}

#[derive(Serialize, Deserialize)]
pub struct CheckResults {
    pub name: String,
    pub matches: Vec<String>,
}

impl CheckResults {
    fn print_results(&self) {
        println!("========================================");
        println!("[+] {} detected", self.name);

        println!("[+] Files:");
        for m in &self.matches {
            println!("{}", m);
        }
    }
}
