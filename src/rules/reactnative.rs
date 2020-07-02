use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) {
    let re = RegexSet::new(&[
        r"lib/.*/libreactnativejni.so",
        r"assets/index.android.bundle",
        r"assets/index.android.bundle.meta",
    ])
    .unwrap();

    let mut matches = Vec::new();

    for path in f {
        if re.is_match(path.as_str()) {
            matches.push(path);
        }
    }

    matches.sort();
    if !matches.is_empty() {
        crate::print_match("React Native", matches);
    }
}
