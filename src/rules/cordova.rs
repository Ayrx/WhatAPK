use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re =
        RegexSet::new(&[r"assets/www/cordova\.js", r"assets/www/cordova_plugins\.js"]).unwrap();

    let mut matches: Vec<String> = f.iter().filter(|path| re.is_match(path)).cloned().collect();
    matches.sort();
    if matches.is_empty() {
        return None;
    }

    Some(crate::CheckResults {
        name: "Apache Cordova".to_owned(),
        matches,
    })
}
