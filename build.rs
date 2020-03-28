use std::ffi::OsStr;
use std::fs;

fn main() {
    if cfg!(feature = "user_provided_lib") {
        return;
    }

    let files: Vec<_> = fs::read_dir("wren/src/vm")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();

            if path.extension() == Some(OsStr::new("c")) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let mut build = cc::Build::new();
    build.files(files);

    if cfg!(feature = "meta") {
        build.define("WREN_OPT_META", None);
        build.file("wren/src/optional/wren_opt_meta.c");
    }

    if cfg!(feature = "random") {
        build.define("WREN_OPT_RANDOM", None);
        build.file("wren/src/optional/wren_opt_random.c");
    }

    if cfg!(debug_assertions) {
        build.define("DEBUG", None);
    }

    build
        .include("wren/src/vm")
        .include("wren/src/optional")
        .include("wren/src/include")
        .compile("wren");
}
