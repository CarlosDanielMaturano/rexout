use std::io::ErrorKind;

pub fn read_file_content(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => format!("Error. No such file or directory: {path}."),
        ErrorKind::PermissionDenied => {
            format!("Error. Could not open {path}. Permission Denied.")
        }
        _ => format!("An unknown type of error ocurred: {err}"),
    })
}

#[cfg(test)]
mod test {
    use crate::file_operations::read_file_content;

    #[test]
    fn read_alphabet_as_hex() {
        let file_path = "mock_files/alphabet.txt";
        let left = (0..=25)
            .map(|value| value + ('a' as u8))
            .collect::<Vec<_>>();
        let right = &read_file_content(file_path).unwrap()[..=25];
        assert_eq!(left, right);
    }

    #[test]
    #[should_panic]
    fn file_does_not_exits() {
        let _ = read_file_content("file_that_does_not_exists").unwrap();
    }
}
