pub fn find_not_working_version(
    working_version: String,
    not_working_version: String,
    script_to_run: fn(current_version: &String) -> bool,
    list_of_versions: &Vec<String>,
) -> Option<&String> {
    let index_working_version = list_of_versions.binary_search(&working_version);
    let index_not_working_version = list_of_versions.binary_search(&not_working_version);
    match (index_working_version, index_not_working_version) {
        (Ok(index_working_version), Ok(index_not_working_version)) => {
            let list_versions_to_test =
                &list_of_versions[index_working_version - 1..index_not_working_version];

            let partition_point =
                list_of_versions.partition_point(|version| !script_to_run(version));
            list_versions_to_test.get(partition_point)
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
            String::from("4.0.8"),
            String::from("5.4.0"),
            test_version,
            &versions,
        );
        assert_eq!(not_working_version, Some(&String::from("5.0.0")))
    }
}
