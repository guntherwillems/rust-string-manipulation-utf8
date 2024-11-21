/// Library string_manipulation
/// Author: Gunther Willems
/// Started 11/2024
/// License: MIT
/// 
/// An implementation of string manipulation functions using character indexing
/// instead of bytes. This library also has common string functions like indexof,
/// substr and substring that exist in other programming languages.
/// It can be used as functions or methods from 'str' type (string slice) or 'String' type.
/// Overview:
///   indexof : get the position from one string into another
///   substr : get a substring of a string using start index and length (signed values)
///   substru : get a substring of a string using start index and length (unsigned values)
///   subst_to_end : get a substring from start index till the end of the string
///   substring : get a substring of a string using start and end index (not included)
///   str_remove : Remove a substring from a string
///   str_concat! : macro to concatenate multiple strings
use std::cmp::Ordering;

pub trait CharString {
    fn indexof(&self, searchstring: &str, start_index: usize) -> Option<usize>;
    fn substr(&self, start_index: isize, length: isize) -> String;
    fn substru(&self, start_index: usize, length: usize) -> String;
    fn substr_end(&self, start_index: isize) -> String;
    fn substring(&self, start_index: isize, end_index: isize) -> String;
    fn str_remove(&self, start_index: usize, length: usize) -> String;
}

impl CharString for str {
    fn indexof(&self, searchstring: &str, start_index: usize) -> Option<usize> {
        indexof(&self, searchstring, start_index)
    }

    fn substr(&self, start_index: isize, length: isize) -> String {
        substr(&self, start_index, length)
    }

    fn substru(&self, start_index: usize, length: usize) -> String {
        substru(&self, start_index, length)
    }

    fn substr_end(&self, start_index: isize) -> String {
        substr_end(&self, start_index)
    }

    fn substring(&self, start_index: isize, end_index: isize) -> String {
        substring(&self, start_index, end_index)
    }

    fn str_remove(&self, start_index: usize, length: usize) -> String {
        str_remove(&self, start_index, length)
    }
}

impl CharString for String {
    fn indexof(&self, searchstring: &str, start_index: usize) -> Option<usize> {
        indexof(&self, searchstring, start_index)
    }

    fn str_remove(&self, start_index: usize, length: usize) -> String {
        str_remove(&self, start_index, length)
    }

    fn substr(&self, start_index: isize, length: isize) -> String {
        substr(&self, start_index, length)
    }

    fn substru(&self, start_index: usize, length: usize) -> String {
        substru(&self, start_index, length)
    }

    fn substr_end(&self, start_index: isize) -> String {
        substr_end(&self, start_index)
    }

    fn substring(&self, start_index: isize, end_index: isize) -> String {
        substring(&self, start_index, end_index)
    }
}

// -------------------------------------------------------------------------

/// Macro to concatenate multiple strings.
/// Allocates the needed capacity and adds the stings.
/// All strings are borrowed.
/// Example: let s: String = str_concat!(&s1, &s2, &s3);
/// The arguments are string slices (&str). The number of arguments can be 2 or more.
/// Example:
///   fn main() {
///     let s1: String = "string1".to_owned();
///     let s2: String = "string2".to_owned();
///     let s3: String = "string3".to_owned();
///     let result1: String = str_concat!(&s1, &s2, &s3);
///     let result2: String = str_concat!(&s1, "string2", &s3);
///   }
#[macro_export]
macro_rules! str_concat {
        ($($arg:expr),+) => {
            {
                let mut len = 0;
                $(
                    len += $arg.len();
                )*
                let mut result = String::with_capacity(len);
                $(
                    result.push_str($arg);
                )*
                result
            }
        };
    }

// -------------------------------------------------------------------------

/// Theoretically calculate the start and end position within a string with a total of length characters.
/// Numbers can be negative to count from the end of the string (start_index) and backwards (length).
/// total_length = total length of the string.
/// start_index = start index (can be negative).
/// length = how many characters to count from the start_index (can be negative).
/// If length is 0, empty string, start and end are equal to 0 (0, 0).
/// If start_index exceeds the string boundary limits, return an empty string (0, 0).
/// Returns Tuple (start, end) positions. Result is a range from 'start' index till 'end' index (not included).
#[inline]
fn calc_start_end(total_length: usize, start_index: isize, length: isize) -> (usize, usize) {
    let start: isize;
    let last: isize;
    let total_length: isize = total_length as isize;

    if total_length == 0 || length == 0 || start_index < -total_length || start_index >= total_length {
        return (0, 0);
    }

    let length: isize = length.clamp(-total_length, total_length);

    // Negative start_index, count backwards from the end
    if start_index >= 0 {
        start = start_index
    } else {
        start = total_length + start_index
    }

    // Negative length, count backwards from start position
    if length > 0 {
        last = (start + length - 1).clamp(0, total_length - 1) // Overflow possible: 2, isize::MAX
    } else {
        last = (start + length + 1).clamp(0, total_length + 1)
    }

    if start > last {
        (last as usize, (start + 1) as usize)
    } else {
        (start as usize, (last + 1) as usize)
    }
}

// -------------------------------------------------------------------------

/// Get the character position from one string into another. Start searching
/// from character index 'start_index'. Returns None if not found. Index of
/// the first character is 0.
pub fn indexof(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = start_index;
    let search_len: usize = searchstring.chars().count();
    let total_len: usize = s.len();
    let mut match_count: usize; // How many characters match from the search position
    let mut next_index: usize;

    // Iterate through the String characters from start_index
    for c in s.chars().skip(start_index) {
        // Relatively fast because searchstring is mostly short
        if c == searchstring.chars().next().unwrap() {
            match_count = 1;

            // sc: char = each subsequent character in searchstring
            for sc in searchstring.chars().skip(1) {
                next_index = char_index + match_count;
                if next_index >= total_len || s.chars().nth(next_index).unwrap() != sc {
                    break;
                }
                match_count += 1;
            }
            // If entire search value matches, return index
            if match_count == search_len {
                return Some(char_index);
            }
        }
        char_index += 1;
    }

    None // No match found
}

// -------------------------------------------------------------------------

/// Remove a substring from a string. Beginning at character index 'start_index'
/// and take 'length' characters. Index of the first character is 0.
pub fn str_remove(s: &str, start_index: usize, length: usize) -> String {
    let mut chars: std::str::Chars = s.chars();
    let mut s: String = s.to_owned();

    if length == 0 {
        return s;
    }

    // Counted in bytes
    let len_byte: usize = s.len(); // Total length in bytes
    let mut start_byte: usize = 0; // Byte position start_index
    let mut start_byte_found: bool = false; // start_index byte position found
    let end_byte: usize; // Byte position end to remove
    let mut pos_byte: usize = 0; // character byte position

    // Counted in characters
    let end_index: usize = start_index + length;
    let mut c: char;
    let mut pos: usize = 0; // character position

    loop {
        c = chars.next().unwrap();

        if pos == start_index {
            start_byte = pos_byte;
            start_byte_found = true;
        } else if pos == end_index {
            end_byte = pos_byte;
            break;
        }

        pos += 1;
        pos_byte += c.len_utf8();

        // If end of String is reached
        if pos_byte == len_byte {
            if start_byte_found {
                end_byte = pos_byte;
                break;
            }

            // In all other cases return the unmodified string
            return s;
        }
    }

    s.replace_range(start_byte..end_byte, ""); // Remove the range
    s
}

// -------------------------------------------------------------------------

/// Get a substring of a string, beginning at character index 'start_index'
/// and take 'length' characters. Index of the first character is 0.
/// Negative numbers count backwards:
///  'start_index' from the end of the string.
///  'length' from 'start_index'.
/// If start_index exceeds the string boundary limits, return an empty string.
/// (Similar to C++ std::substr() and c# String.Substring.)
/// Examples:
///   "0123456789".substr(2, 3)   => "234"
///   "0123456789".substr(-5, 3)  => "567"
///   "0123456789".substr(-5, -3) => "345"
///   "0123456789".substr(5, -3)  => "345"
///   "0123456789".substr(2, 0)   => ""
///   "0123456789".substr(0, 0)   => ""
///   "0123456789".substr(-4, 0)  => ""
/// To get the characters until the end of the string:
///   "0123456789".substr(2, isize::MAX)  => "23456789"
/// Or substr_end(start_index)
///   "0123456789".substr_end(2)  => "23456789"
pub fn substr(s: &str, start_index: isize, length: isize) -> String {
    let total_length: usize = s.chars().count();
    let (start, end) = calc_start_end(total_length, start_index, length);
    // println!("{} {} {} - {} {}", total_length, start_index, length, start, end); // Debug

    s.chars().skip(start).take(end - start).collect::<String>()
}

// -------------------------------------------------------------------------

/// Get a substring of a string, beginning at character index 'start_index'
/// and take 'length' characters. Using unsigned start_index and length.
/// Index of the first character is 0.
#[inline]
pub fn substru(s: &str, start_index: usize, length: usize) -> String {
    s.chars().skip(start_index).take(length).collect::<String>()
}

// -------------------------------------------------------------------------

/// Get a substring from character index 'start_index' till end of the string.
/// 'start_index' can be negative to count backwards from the end.
/// If start_index exceeds the string boundary limits, return an empty string (0, 0).
/// (Similar to C++ std::substr() and c# String.Substring.)
/// Index of the first character is 0.
pub fn substr_end(s: &str, start_index: isize) -> String {
    let total_length: isize = s.chars().count() as isize;

    // Out of string boundaries
    if (start_index < -total_length) || (start_index > total_length) {
        return String::new();
    }

    let start: isize = if start_index >= 0 {
        start_index
    } else {
        total_length + start_index
    };

    s.chars()
        .skip(start as usize)
        .take((total_length - start) as usize)
        .collect::<String>()
}

// -------------------------------------------------------------------------

/// Get a substring of a string beginning at character index start_index up
/// to and *excluding* the character index end_index.
/// Equivalent of JavaScript substring with 2 parameters.
/// If start_index is equal to end_index, substring() returns an empty string.
/// If start_index is greater than end_index, swap start_index and end_index.
/// Any argument value that is less than 0 is treated as if it were 0.
/// Any argument value that is greater than string length is treated as if it were string length.
/// Index of the first character is 0.
pub fn substring(s: &str, start_index: isize, end_index: isize) -> String {
    let mut start: isize;
    let mut end: isize;

    // If start_index is greater than end_index, swap start_index and end_index.
    match start_index.cmp(&end_index) {
        Ordering::Less => {
            start = start_index;
            end = end_index;
        }
        Ordering::Greater => {
            start = end_index;
            end = start_index;
        }
        Ordering::Equal => {
            return String::new();
        }
    }

    let total_length: usize = s.chars().count();
    // If 'start_index' is less than 0 is treated as if it were 0.
    start = isize::max(0, start);
    // If end_index is greater than string length, it is treated as if it were string length.
    end = isize::min(end, total_length as isize);

    s.chars()
        .skip(start as usize)
        .take((end - start) as usize)
        .collect::<String>()
}
