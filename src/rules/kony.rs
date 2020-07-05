use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) -> Option<crate::CheckResults> {
    let re = RegexSet::new(&[
        r"lib/.*/libkonyjsvm.so",
        r"assets/js/common-jslibs.kfm",
        r"assets/js/startup.js",
        r"assets/application.properties",
        r"assets/pluginversions.properties",
        r"assets/konyappluabytecode.o.mp3",
    ])
    .unwrap();

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
        name: "Kony Visualizer".to_owned(),
        matches,
    })
}
