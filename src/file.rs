use std::io::ErrorKind;

pub fn read_file_content_as_u8(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => format!("Error. No such file or directory: {path}."),
        ErrorKind::PermissionDenied => {
            format!("Error. Could not open {path}. Permission Denied.")
        }
        _ => format!("An unknown type of error ocurred: {err}"),
    })
}
