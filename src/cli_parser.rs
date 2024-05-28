use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Cli {
    pub file_path: std::path::PathBuf,
    pub options: Options,
}

impl Cli {
    pub fn try_from(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.skip(1);
        let file_path: PathBuf =
            PathBuf::from(args.next().ok_or(format!("Error. Missing file_path!"))?);

        let options = Options::try_from(args.map(|flag| flag.replace("--", "")))?;
        Ok(Self { file_path, options })
    }
}

#[derive(Debug, PartialEq)]
pub struct Options {
    pub little_endian: bool,
    pub count: bool,
}

impl Options {
    pub fn try_from(flags: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut options = Self {
            little_endian: true,
            count: true,
        };
        for flag in flags {
            match flag.as_str() {
                "big" => options.little_endian = false,
                "no-count" => options.count = false,
                _ => return Err(format!("Unknow type of flag: {flag}")),
            }
        }
        Ok(options)
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
            Cli::try_from(args).unwrap(),
            Cli {
                file_path: String::from("file.txt").into(),
                options: Options {
                    little_endian: true,
                    count: true
                }
            }
        )
    }

    #[test]
    fn file_path_with_flags() {
        let args = "<program_name> file.txt --big --no-count"
            .split_whitespace()
            .map(String::from);

        assert_eq!(
            Cli::try_from(args).unwrap(),
            Cli {
                file_path: String::from("file.txt").into(),
                options: Options {
                    little_endian: false,
                    count: false
                }
            }
        )
    }
}
