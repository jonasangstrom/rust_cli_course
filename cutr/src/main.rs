use anyhow::{Result, bail};
use clap::Parser;
use std::{num::NonZeroUsize, ops::Range};

fn main() {
    let mut args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let delimiter = parse_delimiter(&args.delimiter)?;
    println!("{args:?}");
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of 'cut'
struct Args {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,
    /// Field delimiter
    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,
    /// Selected fields
    #[command(flatten)]
    extract: ArgsExtract,
}
#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    ///Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,
    ///Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,
    ///Selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

fn parse_delimiter(string_delimiter: &String) -> Result<u8> {
    let bytes_delimiter = string_delimiter.as_bytes();
    match bytes_delimiter.len() {
        1 => Ok(*bytes_delimiter.first().unwrap()),
        _ => bail!(r#"--delim "{string_delimiter}" must be a single byte"#),
    }
}

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn parse_pos(range: String) -> Result<PositionList> {
    if range.len() == 0 {
        bail!(r#"illegal list value: "{range}""#);
    }
    let mut position_list = PositionList::new();
    let parts = range.split(",");
    for part in parts {
        let range_values = part.split("-");
        let mut range_position: usize = 0;
        let mut start_index: usize = 0;
        let mut end_index: usize = 0;
        for range_value in range_values {
            let index = index_from_string(range_value)?;
            if range_position == 0 {
                start_index = index;
            } else if range_position == 1 {
                end_index = index;
            } else {
                bail!(r#"illegal list value: "{range}""#);
            }
            range_position += 1;
        }
        if range_position != 1 {
            if start_index >= end_index {
                bail!(
                    r#"First number in range ({start_index}) must be lower than second number ({end_index})"#
                );
            }
        } else {
            end_index = start_index;
        }

        let range = Range {
            start: start_index - 1,
            end: end_index,
        };

        position_list.push(range);
    }

    Ok(position_list)
}

fn index_from_string(string_index: &str) -> Result<usize> {
    match string_index.parse::<NonZeroUsize>() {
        Ok(value) => Ok(usize::from(value)),
        Err(_) => bail!(r#"illegal list value: "{string_index}""#),
    }
}

#[cfg(test)]
mod parse_post_unit_tests {
    use super::parse_pos;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_string_error() {
        assert!(parse_pos("".to_string()).is_err());
    }
    #[test]
    fn zero_not_allowed() {
        // Zero is an error
        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
    }
    #[test]
    fn range_with_zero_is_not_allowed() {
        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
    }
    #[test]
    fn illegal_sign_not_allowed() {
        let res = parse_pos("?1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "?1""#,);

        let res = parse_pos("?1-2".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "?1""#,);

        let res = parse_pos("1-?2".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "?2""#,);
    }
    #[test]
    fn non_number_not_allowed() {
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#,);

        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#,);
    }
    #[test]
    fn wonky_ranges_not_allowed() {
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());
    }
    #[test]
    fn first_number_must_be_less_than_second() {
        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );
    }
    #[test]
    fn test_parse_single_value() {
        // First number must be less than second

        // All the following are acceptable
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
    }
    #[test]
    fn test_parse_two_values() {
        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
    }

    #[test]
    fn test_parse_range() {
        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
    }

    fn test_parse_ranges() {
        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
