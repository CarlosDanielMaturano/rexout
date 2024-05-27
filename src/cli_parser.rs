use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Cli {
    pub file_path: std::path::PathBuf,
    pub flags: Vec<String>,
}

impl Cli {
    pub fn try_from_args(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.skip(1);
        let file_path: PathBuf =
            PathBuf::from(args.next().ok_or(format!("Error. Missing file_path!"))?);

        let flags = args
            .map(|flag| flag.replace("--", ""))
            .collect::<Vec<_>>();
        Ok(Self { file_path, flags })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_path_without_flags() {
        let args = "<program_name> file.txt"
            .split_whitespace()
            .map(String::from);

        assert_eq!(
            Cli::try_from_args(args).unwrap(),
            Cli {
                file_path: String::from("file.txt").into(),
                flags: vec![]
            }
        )
    }

    #[test]
    fn file_path_with_flags() {
        let args = "<program_name> file.txt --little --no-offset"
            .split_whitespace()
            .map(String::from);

        assert_eq!(
            Cli::try_from_args(args).unwrap(),
            Cli {
                file_path: String::from("file.txt").into(),
                flags: vec![
                    String::from("little"),
                    String::from("no-offset"),
                ]
            }
        )
    }
}
