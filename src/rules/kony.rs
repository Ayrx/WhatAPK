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

    let mut matches: Vec<String> = f.iter().filter(|path| re.is_match(path)).cloned().collect();
    matches.sort();
    if matches.is_empty() {
        return None;
    }

    Some(crate::CheckResults {
        name: "Kony Visualizer".to_owned(),
        matches,
    })
}
