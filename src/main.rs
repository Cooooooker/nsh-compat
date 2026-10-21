//! `nsh-compat record <version>`: records the NSH checkout (`../nsh`, or
//! `NSH_DIR`) as release `<version>` in `releases/v<version>/`.
//! `nsh-compat check`: the checks `cargo test` runs, without the test
//! harness.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        ["record", version] => nsh_compat::record(version.trim_start_matches('v')).map(|dir| {
            println!("recorded {}", dir.display());
        }),
        ["check"] => nsh_compat::checkout_apps_run()
            .and_then(|()| nsh_compat::nothing_released_is_removed())
            .and_then(|()| nsh_compat::released_apps_run())
            .map(|()| {
                if !nsh_compat::any_release() {
                    println!("no release is recorded: nothing is frozen");
                }
                println!("{} is compatible", nsh_compat::nsh_dir().display());
            }),
        _ => Err("usage: nsh-compat record <version> | nsh-compat check".to_owned()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
