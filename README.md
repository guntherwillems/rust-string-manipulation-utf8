# rust-string-manipulation-utf8

**A Rust library with string manipulation functions using character indexing (UTF-8)**

Library name: [string_manipulation_utf8](https://crates.io/crates/string_manipulation_utf8)

An implementation of string manipulation functions using character indexing instead of bytes. It uses UTF-8 encoded strings as implemented in Rust.

This library also has common string functions like indexof, substr and substring that exist in other programming languages.

It can be used as functions, or methods from 'str' type (string slice) and 'String' type.

Library functions:

- indexof : get the position from one string into another
- substr : get a substring of a string using start index and length (signed values)
- substru : get a substring of a string using start index and length (unsigned values)
- substr_end : get a substring from start index till the end of the string
- substring : get a substring of a string using start and end index (not included)
- str_remove : Remove a substring from a string
- str_concat! : macro to concatenate multiple strings

Standard Rust functions:

Functions independent of character and byte indexing in Rust.

- replace : replaces all matches of a pattern with another string
- replacen : replaces first N matches of a pattern with another string
- strip_prefix : returns a string slice with the prefix removed

- contains : check if a string contains another string
- starts_with : check if a string starts with another string
- ends_with : check if a string ends with another string
- is_empty : check if a String has a length of zero

> The Rust standard library doesn't support Unicode grapheme clusters (with combining diacritical marks) where multiple code points are required to form one character.  
> Example:  
> e + combining acute = e + ´ = \u{0065}\u{0301} = é (two code points with 3 bytes, hex. 65 CC 81)  
> Versus the character é = \u{00E9} with one code point for 2 bytes, hex. C3 A9  
> This library uses the Rust standard library and hence will count such combined characters as multiple characters.

See section 'Using byte positioning' for examples with native byte indexing.

> Simple benchmarking code was used to find the faster algorithms. [GitHub rust-string-manip-benchmark](https://github.com/guntherwillems/rust-string-manip-benchmark)

To compile and run the example code in examples/main.rs:  
`cargo run --example main`

To compile and run the tests in tests/tests.rs:  
`carto test`

Install:  
Run the following Cargo command in your project directory:  
`cargo add string_manipulation_utf8 `  
Or add the following line to your Cargo.toml:  
`string_manipulation_utf8 = "0.2.0"`


## Using character positioning


### indexof

Get the character position from one string into another. Start searching from character 'start_index'. Returns None if not found. Index of the first character is 0.


Syntax:

- `str.indexof(searchstring: &str, start_index: usize) -> Option<usize>`
- `string.indexof(searchstring: &str, start_index: usize) -> Option<usize>`
- `indexof(s: &str, searchstring: &str, start_index: usize) -> Option<usize>`

Example:

Return the character index of "test" in the given string. Start searching at the beginning of the string. Result position is 0 because "test" starts at the beginning of the string.

~~~rust
use string_manipulation_utf8::CharString; // String and str methods
use string_manipulation_utf8::indexof; // str function

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    match s1.indexof("test", 0) { // Result: Some(0)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match s2.indexof("test", 0) { // Result: Some(0)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match indexof(s1, "test", 0) { // Result: Some(0)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match indexof(&s2, "test", 0) { // Result: Some(0)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }
}
~~~


Return the character index of "test" in the given string. Start searching from character index 6. The result is position 14.

~~~rust
use string_manipulation_utf8::indexof;
use string_manipulation_utf8::CharString; // String and str methods.

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    match s1.indexof("test", 6) { // Result: Some(14)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match s2.indexof("test", 6) { // Result: Some(14)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match indexof(s1, "test", 6) { // Result: Some(14)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    match indexof(&s2, "test", 6) { // Result: Some(14)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }
}
~~~


### substr

Get a substring of a string, beginning at character index 'start_index' and take 'length' characters.  
Negative numbers count backwards:  
  'start_index' from the end of the string.  
  'length' from 'start_index'.  
If start_index exceeds the string boundary limits, return an empty string. (Similar to C++ std::substr() and c# String.Substring.)  
'length' can be isize::MAX or isize::MIN to get the substring until the positive or negative string boundary without the need to calculate the length. (Alternatively, see substr_end in this library.)  
Index of the first character is 0.

If 'start_index' and 'length' are positive, substru is a little faster like string.chars().skip(start_index).take(length).collect() that it interpolates. See substru and section 'Standard Rust methods' for examples.

Syntax:

- `str.substr(start_index: isize, length: isize) -> String`
- `string.substr(start_index: isize, length: isize) -> String`
- `substr(s: &str, start_index: isize, length: isize) -> String`

Example:

~~~rust
use string_manipulation_utf8::CharString; // String and str methods

fn main() {
    assert_eq!("0123456789".substr(2, 3), "234");
    assert_eq!("0123456789".substr(-5, 3), "567");
    assert_eq!("0123456789".substr(-5, -3), "345"); // Negative length counts backwards
    assert_eq!("0123456789".substr(5, -3), "345"); // Negative length counts backwards
    assert_eq!("0123456789".substr(2, 0), ""); // Take nothing
    assert_eq!("0123456789".substr(0, 0), ""); // Take nothing
    assert_eq!("0123456789".substr(-4, 0), ""); // Take nothing

    assert_eq!("0123456789".substr(isize::MAX, 1), "");
    assert_eq!("0123456789".substr(isize::MAX, isize::MIN), "");
    assert_eq!("0123456789".substr(isize::MIN, isize::MAX), "");
}
~~~

Example:

~~~rust
use string_manipulation_utf8::substr;
use string_manipulation_utf8::CharString; // String and str methods

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    println!("substr str: {}", s1.substr(10, 3)); // Result: "123"
    println!("substr String: {}", s2.substr(10, 3)); // Result: "123"
    println!("substr function: {}", substr(s1, 10, 3)); // Result: "123"
}
~~~

Remark:

> To get a substring from 'start_index' until the end of the string:  
> substr(string, start_index, isize::MAX)  
> substr_end(string, start_index)  
> substr(string, start_index, string.chars().count() is isize - start_index)


## substru

Same as substr, but only accepts unsiged values for 'start_index' and 'length'.  
For positive numbers this is faster than using substr.  
It interpolates the code: s.chars().skip(start_index).take(length).collect::<String>()

Syntax:

- `str.substru(start_index: usize, length: usize) -> String`
- `string.substru(start_index: usize, length: usize) -> String`
- `substru(s: &str, start_index: usize, length: usize) -> String`


## substring

Get a substring of a string beginning at character index 'start_index' up to and *excluding* the character index 'end_index'.

Equivalent of JavaScript substring with 2 parameters.  
If 'start_index' is equal to 'end_index', substring() returns an empty string.  
If 'start_index' is greater than 'end_index', swap 'start_index' and 'end_index'.  
Any argument value that is less than 0 is treated as if it were 0.  
Any argument value that is greater than string length is treated as if it were string length.  
Index of the first character is 0.

Syntax:

- `str.substring(start_index: isize, end_index: isize) -> String`
- `string.substring(start_index: isize, end_index: isize) -> String`
- `substring(s: &str, start_index: isize, end_index: isize) -> String`

Example:

~~~rust
use string_manipulation_utf8::CharString; // String and str methods
use string_manipulation_utf8::substring; // str function

fn main() {
    println!("{}", substring("0123456789", 2, 3)); // Result: 2
    println!("{}", substring("0123456789", 2, 9)); // Result: 2345678
    println!("{}", substring("0123456789", 2, 10)); // Result: 23456789
    println!("{}", substring("0123456789", 2, 11)); // Result: 23456789
    println!("{}", substring("0123456789", -2, 3)); // Result: 012
    println!("{}", substring("0123456789", -2, 50)); // Result: 0123456789
    println!("{}", substring("0123456789", 9, 2)); // Result: 2345678
    
    let str: &str = "test éèçà 123 test";
    let string: String = str.to_owned();

    println!("{}", str.substring(10, 14)); // 123
    println!("{}", string.substring(10, 14)); // 123
    println!("{}", substring(str, 10, 14)); // 123
}
~~~


## substr_end

Get a substring from character index 'start_index' till end of the string.  
'start_index' can be negative to count backwards from the end of the string.  
If start_index exceeds the string boundary limits, return an empty string.  
(Similar to C++ std::substr() and c# String.Substring.)  
Index of the first character is 0.

> Because Rust doesn't have a practical default value for function parameters, substr_end()  
> replaces substr(string, start_index), string.substr(start_index).  
> Same result with: substr(string, start_index, isize::MAX)

Syntax:

- `substr_end(s: &str, start_index: isize) -> String`
- `string.substr_end(start_index: isize) -> String`
- `str.substr_end(start_index: isize) -> String`

~~~rust
assert_eq!("0123456789".substr_end(2), "23456789");
assert_eq!("0123456789".substr_end(0), "0123456789");
assert_eq!("0123456789".substr_end(9), "9");
assert_eq!("0123456789".substr_end(10), "");
assert_eq!("0123456789".substr_end(-3), "789");
~~~

Example:

~~~rust
use string_manipulation_utf8::substr_end;
use string_manipulation_utf8::CharString; // String and str methods // str function

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    println!("substr_end str: {}", s1.substr_end(10)); // Result: "123 test"
    println!("substr_end String: {}", s2.substr_end(10)); // Result: "123 test"
    println!("substr_end function: {}", substr_end(s1, 10)); // Result: "123 test"
    println!("substr_end function: {}", substr_end(&s2, 10)); // Result: "123 test"
}
~~~


### str_remove

Remove a substring from a string. Beginning at character index 'start_index' and take 'length' characters.  
Index of the first character is 0.

Syntax:

- `str.str_remove(start_index: usize, length: usize) -> String`
- `string.str_remove(start_index: usize, length: usize) -> String`
- `str_remove(s: &str, start_index: usize, length: usize) -> String`


Examples:

~~~rust
use string_manipulation_utf8::str_remove;
use string_manipulation_utf8::CharString; // String and str methods // str function

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    println!("str_remove str: {}", s1.str_remove(10, 4)); // Result: "test éèçà test"
    println!("str_remove String: {}", s2.str_remove(10, 4)); // Result: "test éèçà test"
    println!("str_remove function: {}", str_remove(s1, 10, 4)); // Result: "test éèçà test"
}
~~~


### str_concat

Macro to concatenate multiple strings.  
All strings are borrowed.  
First allocates the needed capacity, then adds the stings.

Syntax:

`str_concat!(&str1, &str2, ...)`

Examples:

~~~rust
use string_manipulation_utf8::str_concat;

fn main() {
    println!(
        "{}",
        str_concat!("test", " ", "123 ", "éèçà ", "123 ", "test home")
    ); // Result: "test 123 éèçà 123 test home"

    let s1: String = "string1".to_owned();
    let s2: String = "string2".to_owned();
    let s3: String = "string3".to_owned();
    let result: String = str_concat!(&s1, &s2, &s3);
    println!("{result}"); // Result: "string1string2string3"

    let s2: &str = "string2"; // Adding a string slice
    let result: String = str_concat!(&s1, s2, &s3);
    println!("{result}"); // Result: "string1string2string3"
}
~~~

Alternatives with Rust statements.

The Rust 'std::concat!' macro only works with literals. Ex. `concat!("test", 10, 'b', true)`

Using the std::format macro.  
`format!("{}{}{}", s1, s2, s3)`

When adding strings with the + operator, the first string is moved (move of ownership), from the second string it's borrowed.  
`s1.clone() + &s2 + &s3`  
`s1.to_owned() + &s2 + &s3`


### Standard Rust methods

Standard Rust methods independent of character or byte indexing.

- replace : Replaces all matches of a pattern with another string.
- replacen : Replaces first N matches of a pattern with another string.
- strip_prefix : Returns a string slice with the prefix removed if the search string is found at the beginning of the string.
- strip_suffix : Return a string slice with suffix removed if the search string is found at the end of the string.

- contains : Check if the given pattern matches a sub-slice of this string slice.
- starts_with : Check if the given pattern matches a prefix of this string slice.
- ends_with : Check if the given pattern matches a suffix of this string slice.
- is_empty : Check if this String has a length of zero.
- chars() : Getting a substring with the chars iterator.

Examples:

~~~rust
fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    println!("{}", s1.replace("test", "new")); // Result: "new éèçà 123 new"
    println!("{}", s2.replacen("test", "new", 1)); // Result: "new éèçà 123 test"

    match s1.strip_prefix("test ") {
        // Result: Some("éèçà 123 test")
        Some(s) => println!("Found: {}", s),
        None => println!("Not found"),
    };

    let result = match s2.strip_prefix("test ") {
        // Result: Some("éèçà 123 test")
        Some(s) => s,
        None => &s2,
    };
    println!("{result}");

    assert_eq!(s1.contains("123"), true);
    assert_eq!(s2.contains("123"), true);
    assert_eq!(s1.starts_with("test"), true);
    assert_eq!(s2.ends_with("test"), true);
    assert_eq!(s1.is_empty(), false);
    assert_eq!(s2.is_empty(), false);
}
~~~

Getting a substring with the Rust chars() module that returns an iterator over the string characters. Skip(), take() and count() consume the chars iterator.

~~~rust
fn main() {
    let str: &str = "test éèçà 123 test";
    let string: String = str.to_owned();

    let start_index: usize = 5;
    let length: usize = 4;

    // All 4 results return éèçà

    // With type annotation
    let _s1: String = string.chars().skip(start_index).take(length).collect();
    let _s2: String = str.chars().skip(start_index).take(length).collect();

    // Without type annotation
    let _s3 = string.chars().skip(start_index).take(length).collect::<String>();
    let _s4 = str.chars().skip(start_index).take(length).collect::<String>();
    
    let _total1 = str.chars().count(); // Length in characters (=18). Consumes the chars iterator.
    let _total2 = string.chars().count(); // Length in characters (=18). Consumes the chars iterator.
}
~~~


## Using byte positioning

Get a substring using byte positions with standard Rust methods.

Using a string slice:

~~~rust
use string_manipulation_utf8::str_concat;

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: &str = "éèçà ";

    let s2_pos_o: Option<usize> = s1.find(s2);
    if s2_pos_o.is_some() {
        let s2_len: usize = s2.len(); // Length in bytes
        let s2_pos: usize = s2_pos_o.unwrap(); // Position of s2 in s1 in bytes
        let s2_pos_end: usize = s2_pos + s2_len; // Position of the last character of s2 in s1 in bytes

        // Remove s2 from s1. Result: test 123 test
        println!("{}", s1[..s2_pos].to_owned() + &s1[s2_pos_end..]);

        // Same using the macro str_concat! from this library. Result: test 123 test
        println!("{}", str_concat!(&s1[..s2_pos], &s1[s2_pos_end..]));

        // Get characters from s1 after s2
        println!("{}", &s1[s2_pos_end..]); // Result: 123 test
    }
}
~~~

Using a string:

~~~rust
use string_manipulation_utf8::str_concat;

fn main() {
    let s1: String = "test éèçà 123 test".to_owned();
    let s2: &str = "éèçà ";

    let s2_pos_o: Option<usize> = s1.find(s2);
    if s2_pos_o.is_some() {
        let s2_len: usize = s2.len(); // Length in bytes
        let s2_pos: usize = s2_pos_o.unwrap(); // Position in bytes
        let s2_pos_end: usize = s2_pos + s2_len;

        // Remove s2 from s1. Result: test 123 test
        println!("{}", s1[..s2_pos].to_owned() + &s1[s2_pos_end..]);

        // Same using the macro str_concat! from this library. Result: test 123 test
        println!("{}", str_concat!(&s1[..s2_pos], &s1[s2_pos_end..]));

        // Get characters from s1 after s2 inside s1
        println!("{}", &s1[s2_pos_end..]); // Result: 123 test
    }
}
~~~

Shorter version:

~~~rust
use string_manipulation_utf8::str_concat;

fn main() {
    // let s1: String = "test éèçà 123 test".to_owned(); // Also works with a string
    let s1: &str = "test éèçà 123 test";
    let s2: &str = "éèçà ";

    let s2_pos_o: Option<usize> = s1.find(s2);
    if s2_pos_o.is_some() {
        println!("{}", str_concat!(&s1[..s2_pos_o.unwrap()], &s1[s2_pos_o.unwrap() + s2.len()..]));
        // Result: test 123 test
    }
}
~~~
