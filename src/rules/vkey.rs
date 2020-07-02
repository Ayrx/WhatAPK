use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) {
    let re = RegexSet::new(&[
        r"lib/.*/libchecks.so",
        r"lib/.*/libvtap.so",
        r"lib/.*/libvosWrapperEx.so",
        r"assets/vkeylicensepack",
        r"assets/vkeylicensepack.json",
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
        crate::print_match("V-Key", matches);
    }
}
