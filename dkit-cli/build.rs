// build.rs — Generate man pages for dkit and its subcommands using clap_mangen.
//
// Man pages are written to $OUT_DIR/man/ during `cargo build`.
// To install them, copy the generated .1 files to a man page directory
// (e.g. /usr/local/share/man/man1/).

#[path = "src/cli.rs"]
mod cli;

use clap::CommandFactory;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir).expect("failed to create man directory");

    let cmd = cli::Cli::command();

    // Generate man page for the top-level command
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buf = Vec::new();
    man.render(&mut buf).expect("failed to render man page");
    fs::write(man_dir.join("dkit.1"), buf).expect("failed to write dkit.1");

    // Generate man pages for each subcommand
    for sub in cmd.get_subcommands() {
        let sub_name = sub.get_name().to_owned();
        let filename = format!("dkit-{sub_name}.1");
        let man = clap_mangen::Man::new(sub.clone());
        let mut buf = Vec::new();
        man.render(&mut buf).expect("failed to render man page");
        fs::write(man_dir.join(&filename), buf)
            .unwrap_or_else(|_| panic!("failed to write {filename}"));
    }

    // Write the man directory path so the binary can reference it if needed
    println!("cargo:rustc-env=DKIT_MAN_DIR={}", man_dir.display());
}
