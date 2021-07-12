use std::env;

use clap_generate::{
    generate_to,
    generators::{Bash, Zsh},
};

include!("src/cli.rs");

fn main() {
    let outdir;

    match env::var("OUT_DIR") {
        Result::Ok(dir) => {
            outdir = dir;
        }
        Result::Err(_err) => return,
    }

    let mut app = build();
    app.set_bin_name(crate_name!());

    generate_to::<Bash, _, _>(&mut app, "zli", &outdir);
    generate_to::<Zsh, _, _>(&mut app, "zli", &outdir);
}
