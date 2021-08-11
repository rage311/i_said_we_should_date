use std::env;
use std::process::Command;

fn usage(me: String) {
    println!(
        "Usage: {} timestamp child_binary, e.g.:\n{} 2147483647 /path/to/my/binary\n{} 2038-01-19T03:14:07Z /path/to/my/binary",
        me, me, me
    );
}

fn main() {
    let mut args = env::args();

    // four shalt thou not count, neither count thou two, excepting that thou then proceed to three
    if env::args().count() != 3 {
        usage(args.nth(0).unwrap());
        std::process::exit(1);
    }

    let _me = args.nth(0).unwrap();

    let timestamp = args.nth(0).unwrap();
    let child_bin_full = args.nth(0).unwrap();
    let child_bin_parts: Vec<&str> = child_bin_full.split(" ").collect();

    let child_bin = child_bin_parts.get(0).unwrap();
    let mut cmd = Command::new(child_bin);
    if child_bin_parts.len() > 1 {
        cmd.args(&child_bin_parts[1..]);
    }

    let result = cmd
        .env("DT_OVERRIDE", timestamp)
        .env(
            "LD_PRELOAD",
            std::fs::canonicalize("./libld_preload_rust.so")
                .expect("Unable to find libld_preload_rust.so in current directory"),
        )
        .status();

    if let Err(e) = result {
        eprintln!("Command failed to start '{}': {}", child_bin, e);
    }
}
