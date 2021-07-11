use std::env;

use clap_generate::{
    generators::{Bash, Zsh},
    generate_to,
};

include!("src/cli.rs");

fn main() {
    let outdir = env!("CARGO_MANIFEST_DIR");

    let mut app = build();
    app.set_bin_name(crate_name!());

    generate_to::<Bash, _, _>(&mut app, "zli", &outdir);
    generate_to::<Zsh, _, _>(&mut app, "zli", &outdir);
}
