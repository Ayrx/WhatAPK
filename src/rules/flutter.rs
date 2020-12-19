use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re = RegexSet::new(&[r"lib/.*/libflutter.so"]).unwrap();

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
        name: "Flutter".to_owned(),
        matches,
    })
}
