# rust-string-manipulation-utf8

**A Rust library for string manipulation using character indexing (UTF-8)**

Library name: [string_manipulation_utf8](https://crates.io/crates/string_manipulation_utf8)

This library implements string manipulation functions that use character indexing instead of byte indexing. It works with UTF-8-encoded strings as implemented in Rust.

It provides common string functions, such as indexof, substr, and substring, that are available in other programming languages.

The functions can be called directly or used as methods on the 'str' (string slice) and 'String' types.

Library functions:

- indexof : find the position of one string within another (returns None if not found)
- indexof_1 : find the position of one string within another (returns -1 if not found)
- substr : extract a substring using a start index and a length (signed values); negative values count backwards
- substru : extract a substring using a start index and a length (unsigned values)
- substr_end : extract a substring from a start index to the end of the string
- substring : extract a substring between a start index and an end index (not included)
- str_remove : remove a substring from a string
- str_concat! : macro to concatenate multiple strings (all strings are borrowed)

> The Rust standard library doesn’t provide built-in grapheme-cluster segmentation, so it can’t treat a sequence of code points — such as a base character plus combining diacritical marks — as a single user-perceived character.  
> Example:  
> e + combining acute = e + ´ = \u{0065}\u{0301} = é (two code points with 3 bytes, hex. 65 CC 81)  
> Versus the character é = \u{00E9} with one code point for 2 bytes, hex. C3 A9  
> This library uses the Rust standard library and hence will count such combined characters as multiple characters.

> Simple microbenchmarks were used to find the fastest algorithms. [GitHub rust-string-manip-benchmark](https://github.com/guntherwillems/rust-string-manip-benchmark)

Installation:

Run the following Cargo command in your project directory:  
`cargo add string_manipulation_utf8`  
Or add the following line to your Cargo.toml file:  
`string_manipulation_utf8 = "1.0.0"`

To compile and run the example code in examples/main.rs:  
`cargo run --example main`

To compile and run the tests in tests/tests.rs:  
`cargo test`


## Using character indexing


### indexof

Find the character position of one string within another, starting the search at character index 'start_index'. Returns None if no match is found. The first character has index 0.
Maximum index that can found usize::MAX.


Syntax:

- `str.indexof(searchstring: &str, start_index: usize) -> Option<usize>`
- `string.indexof(searchstring: &str, start_index: usize) -> Option<usize>`
- `indexof(s: &str, searchstring: &str, start_index: usize) -> Option<usize>`

Example:

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

    // Find position "test" after string position 6
    match s1.indexof("test", 6) { // Result: Some(14)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }
}
~~~


### indexof_1

Find the character position of one string within another, starting the search at character index 'start_index'. Returns -1 if no match is found. The first character has index 0.  
This function works like indexof, but returns -1 if no match is found. It converts the result of indexof().
Maximum index that can found isize::MAX.


Syntax:

- `str.indexof_1(searchstring: &str, start_index: usize) -> isize`
- `string.indexof_1(searchstring: &str, start_index: usize) -> isize`
- `indexof_1(s: &str, searchstring: &str, start_index: usize) -> isize`

Example:

~~~rust
use string_manipulation_utf8::CharString; // String and str methods
use string_manipulation_utf8::indexof_1; // str function

fn main() {
    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();
    let mut pos: isize;

    match s1.indexof_1("test", 0) { // Result: 0
        -1 => println!("Not found"),
        pos => println!("Found at position: {pos}")
    }

    pos = s1.indexof_1("testing", 0); // Result: -1
    if pos == -1 {
        println!("Not found");
    } else {
        println!("Found at position: {}", pos);
    }

    pos = s2.indexof_1("test", 0); // Result: 0
    if pos == -1 {
        println!("Not found");
    } else {
        println!("Found at position: {}", pos);
    }

    pos = indexof_1(s1, "test", 0); // Result: 0
    if pos == -1 {
        println!("Not found");
    } else {
        println!("Found at position: {}", pos);
    }

    pos = indexof_1(&s2, "test", 0); // Result: 0
    if pos == -1 {
        println!("Not found");
    } else {
        println!("Found at position: {}", pos);
    }

    // Find position "test" after string position 6
    match s1.indexof_1("test", 6) { // Result: 14
        -1 => println!("Not found"),
        pos => println!("Found at position: {pos}")
    }
}
~~~


### substr

Extract a substring beginning at character index 'start_index' and containing 'length' characters.  
Negative values count backwards:  
  A negative 'start_index' counts backwards from the end of the string.  
  A negative 'length' counts backwards from 'start_index'.  
If start_index is outside the bounds of the string, an empty string is returned. (Similar to C++ std::substr() and C# String.Substring.)  
Set 'length' to isize::MAX or isize::MIN to extract a substring up to the end or beginning of the string, respectively, without calculating the length. (Alternatively, see substr_end in this library.)  
The first character has index 0.

If 'start_index' and 'length' are positive, substru is a faster.


Syntax:

- `str.substr(start_index: isize, length: isize) -> String`
- `string.substr(start_index: isize, length: isize) -> String`
- `substr(s: &str, start_index: isize, length: isize) -> String`

Example:

~~~rust
use string_manipulation_utf8::CharString; // String and str methods
use string_manipulation_utf8::substr;

fn main() {
    assert_eq!("0123456789".substr(2, 3), "234");
    assert_eq!("0123456789".substr(-5, 3), "567");
    assert_eq!("0123456789".substr(-5, -3), "345"); // Negative length counts backwards
    assert_eq!("0123456789".substr(5, -3), "345"); // Negative length counts backwards
    assert_eq!("0123456789".substr(2, 0), ""); // Take nothing
    assert_eq!("0123456789".substr(0, 0), ""); // Take nothing
    assert_eq!("0123456789".substr(-4, 0), ""); // Take nothing

    assert_eq!("0123456789".substr(5, isize::MAX), "56789");
    assert_eq!("0123456789".substr(isize::MAX, 1), ""); // Out of bounds
    assert_eq!("0123456789".substr(isize::MAX, isize::MIN), ""); // Out of bounds
    assert_eq!("0123456789".substr(isize::MIN, isize::MAX), ""); // Out of bounds

    let s1: &str = "test éèçà 123 test";
    let s2: String = s1.to_owned();

    println!("substr str: {}", s1.substr(10, 3)); // Result: "123"
    println!("substr String: {}", s2.substr(10, 3)); // Result: "123"
    println!("substr function: {}", substr(s1, 10, 3)); // Result: "123"
}
~~~

Note:

> To extract a substring from 'start_index' to the end of the string:  
> substr(string, start_index, isize::MAX)  
> substr_end(string, start_index)  
> substr(string, start_index, string.chars().count() as isize - start_index)


## substru

Works like substr, but only accepts unsigned values for 'start_index' and 'length'.  
For positive values, this is faster than using substr.  
It interpolates the following code: s.chars().skip(start_index).take(length).collect::<String>()


Syntax:

- `str.substru(start_index: usize, length: usize) -> String`
- `string.substru(start_index: usize, length: usize) -> String`
- `substru(s: &str, start_index: usize, length: usize) -> String`


## substring

Extracts a substring beginning at character index 'start_index' up to and *excluding* the character index 'end_index'.

Equivalent to JavaScript substring with 2 parameters.  
If 'start_index' equals 'end_index', substring() returns an empty string.  
If 'start_index' is greater than 'end_index', the two indexes are swapped.  
Any argument value less than 0 is treated as 0.  
Any argument value greater than the string length is treated as the string length.  
The first character has index 0.


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

Extract a substring from character index 'start_index' to the end of the string.  
A negative 'start_index' counts backwards from the end of the string.  
If start_index is outside the bounds of the string, an empty string is returned.  
(Similar to C++ std::substr() and C# String.Substring.)  
The first character has index 0.

> Produces the same result as: substr(string, start_index, isize::MAX)


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

Remove a substring from a string. Beginning at character index 'start_index'  
remove 'length' characters. The first character has index 0.


Syntax:

- `str.str_remove(start_index: usize, length: usize) -> String`
- `string.str_remove(start_index: usize, length: usize) -> String`
- `str_remove(s: &str, start_index: usize, length: usize) -> String`


Example:

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

A macro for concatenating multiple strings.  
All strings are borrowed.  
First allocates the required capacity, then appends the strings.


Syntax:

`str_concat!(&str1, &str2, ...)`

Example:

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

    let result1: String = str_concat!(&s1, &s2, &s3);
    println!("{result1}"); // Result: "string1string2string3"

    let s2: &str = "string2"; // Adding a string slice
    let result2: String = str_concat!(&s1, s2, &s3);
    println!("{result2}"); // Result: "string1string2string3"
}
~~~

Alternatives using standard Rust.

The Rust 'std::concat!' macro only works with literals. Example: `concat!("test", 10, 'b', true)`

Using the std::format macro (slower)  
`format!("{}{}{}", s1, s2, s3)`

When concatenating strings with the + operator, ownership of the first string is moved; subsequent strings are borrowed.  
`s1.clone() + &s2 + &s3`  
`s1.to_owned() + &s2 + &s3`


## Standard Rust methods

Some standard Rust methods that do not depend on character or byte indexing:

- replace : replace all matches of a pattern with another string.
- replacen : replace the first N matches of a pattern with another string.
- strip_prefix : return a string slice with the prefix removed if the search string is found at the beginning of the string.
- strip_suffix : return a string slice with the suffix removed if the search string is found at the end of the string.

- contains : check whether the given pattern matches a substring of this string slice.
- starts_with : check whether the given pattern matches a prefix of this string slice.
- ends_with : check whether the given pattern matches a suffix of this string slice.
- is_empty : check whether this String has a length of zero.
- chars() : use the chars iterator to extract a substring.
