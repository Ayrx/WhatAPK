use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re =
        RegexSet::new(&[r"assets/www/cordova\.js", r"assets/www/cordova_plugins\.js"]).unwrap();

    let mut matches = Vec::new();

    for path in f {
        if re.is_match(path.as_str()) {
            matches.push(path.clone());
        }
    }

    matches.sort();
    if matches.is_empty() {
        return None;
    }

    Some(crate::CheckResults {
        name: "Apache Cordova".to_owned(),
        matches,
    })
}
