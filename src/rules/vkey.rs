use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re = RegexSet::new(&[
        r"lib/.*/libchecks.so",
        r"lib/.*/libvtap.so",
        r"lib/.*/libvosWrapperEx.so",
        r"assets/vkeylicensepack",
        r"assets/vkeylicensepack.json",
    ])
    .unwrap();

    let mut matches: Vec<String> = f.iter().filter(|path| re.is_match(path)).cloned().collect();
    matches.sort();
    if matches.is_empty() {
        return None;
    }

    Some(crate::CheckResults {
        name: "V-Key".to_owned(),
        matches,
    })
}
