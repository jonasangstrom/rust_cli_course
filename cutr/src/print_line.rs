use crate::position_list::PositionList;
use anyhow::{Result, bail};

pub fn print_line_to_stdout_fields(
    line: &String,
    delimiter: &u8,
    position_list: &PositionList,
) -> Result<()> {
    let char_delim = *delimiter as char;
    let parts = line.split(char_delim);
    let vec_part: Vec<&str> = parts.clone().collect();
    let mut list_index: usize = 0;
    let list_length = position_list.len();
    for range in position_list {
        list_index += 1;
        let range_length = range.len();
        let mut range_index: usize = 0;
        for index in range.clone().into_iter() {
            range_index += 1;
            let part = vec_part[index];
            if let Some(stripped_part) = part.strip_suffix("\n") {
                print!("{stripped_part}")
            } else {
                print!("{part}")
            };
            if list_index != list_length {
                print!("{char_delim}");
            } else if range_index != range_length {
                print!("{char_delim}");
            }
        }
    }
    println!("");
    Ok(())
}

pub fn print_line_to_stdout_chars(
    line: &String,
    _delimiter: &u8,
    position_list: &PositionList,
) -> Result<()> {
    let char_vec: Vec<char> = line.chars().collect();
    let line_size = char_vec.len();
    for og_range in position_list {
        let start = og_range.start;
        if start > line_size - 1 {
            continue;
        }
        let end: usize;
        if og_range.end > line_size {
            end = line_size;
        } else {
            end = og_range.end;
        }

        let slice = &char_vec[start..end];
        for char in slice {
            print!("{char}");
        }
    }
    println!("");
    Ok(())
}

pub fn print_line_to_stdout_bytes(
    _line: &String,
    _delimiter: &u8,
    _position_list: &PositionList,
) -> Result<()> {
    bail!("Not implemented!")
}
