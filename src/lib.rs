use std::{
    path::PathBuf,
    process::{self, Command},
};

pub fn find_not_working_version<'a>(
    working_version: &String,
    not_working_version: &String,
    script_to_run: impl Fn(&String) -> bool,
    list_of_versions: &'a Vec<String>,
) -> Option<&'a String> {
    let index_working_version = list_of_versions.iter().position(|v| v == working_version);
    let index_not_working_version = list_of_versions
        .iter()
        .position(|v| v == not_working_version);
    match (index_working_version, index_not_working_version) {
        (Some(index_working_version), Some(index_not_working_version)) => {
            let list_versions_to_test =
                &list_of_versions[index_working_version..index_not_working_version + 1];

            let partition_point =
                list_versions_to_test.partition_point(|version| script_to_run(version));
            list_versions_to_test.get(partition_point + 1)
        }
        (err1, err2) => {
            todo!(
                "Failed to find indice, non found working index{:?}, found working index {:?}",
                err1,
                err2
            )
        }
    }
}

pub fn is_ok_script(path_to_script: &PathBuf, package_name: &String, version: &String) -> bool {
    println!("Trying {}@{}", package_name, version);
    let install_result = Command::new("pnpm")
        .arg("add")
        .arg(format!("{package_name}@{version}"))
        .output();

    if let Err(install_error) = install_result {
        println!("Command failed with {}", install_error);
        process::exit(1);
    }

    let script_output = Command::new(path_to_script).output().unwrap();

    script_output.status.success()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn test_version(version: &String) -> bool {
            match version.cmp(&String::from("5.0.0")) {
                std::cmp::Ordering::Less => false,
                _ => true,
            }
        }
        let versions = vec![
            String::from("3.0.1"),
            String::from("4.0.8"),
            String::from("4.5.0"),
            String::from("5.0.0"),
            String::from("5.4.0"),
        ];
        let not_working_version = find_not_working_version(
            &String::from("4.0.8"),
            &String::from("5.4.0"),
            test_version,
            &versions,
        );
        assert_eq!(not_working_version, Some(&String::from("5.0.0")))
    }
}
