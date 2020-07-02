use regex::RegexSet;
use std::collections::HashSet;

pub fn check(f: &HashSet<String>) {
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
            matches.push(path);
        }
    }

    matches.sort();
    if !matches.is_empty() {
        crate::print_match("Kony Visualizer", matches);
    }
}
