use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re = RegexSet::new(&[
        r"lib/.*/libreactnativejni.so",
        r"assets/index.android.bundle",
        r"assets/index.android.bundle.meta",
    ])
    .unwrap();

    let mut matches: Vec<String> = f.iter().filter(|path| re.is_match(path)).cloned().collect();
    matches.sort();
    if matches.is_empty() {
        return None;
    }

    Some(crate::CheckResults {
        name: "React Native".to_owned(),
        matches,
    })
}
