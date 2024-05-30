use std::{
    path::PathBuf,
    process::{self, Command},
};

use bisect_npm_version::{find_not_working_version, is_ok_script};

fn main() {
    // Get package name, working version, current version from command line
    let args: Vec<String> = std::env::args().collect();
    let package_name = &args[1];
    let not_working_version = &args[2];
    let working_version = &args[3];
    let script_path = &args[4];

    let list_of_versions = Command::new("npm")
        .args(["view", package_name, "versions", "--json"])
        .output()
        .expect("ntm")
        .stdout;

    let unparsed_list_of_versions = match String::from_utf8(list_of_versions) {
        Ok(v) => v,
        Err(_) => {
            println!("Cannot get versions");
            process::exit(1)
        }
    };

    let list_of_versions = match serde_json::from_str::<Vec<String>>(&unparsed_list_of_versions) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "There was an error parsing list of {package_name} versions, {}",
                e
            );
            process::exit(1)
        }
    };
    println!("Versions of {package_name}, {:?}", list_of_versions);

    let default_value = String::from("rip is peace");
    let find_not_working_version = find_not_working_version(
        working_version,
        not_working_version,
        |version: &String| is_ok_script(&PathBuf::from(script_path), package_name, version),
        &list_of_versions,
    )
    .unwrap_or(&default_value);

    println!("Failing version: {find_not_working_version}");
}
