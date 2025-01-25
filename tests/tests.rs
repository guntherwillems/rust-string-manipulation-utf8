use string_manipulation_utf8::CharString;
use string_manipulation_utf8::{indexof, str_remove, substr, substr_end, substring, substru, str_concat};

#[test]
fn test_substr() {
    let s1: &str = "0123456789";
    let s2: String = "0123456789".to_owned();

    // ------------------------------ function ---------------------------------

    // start_index >= 0
    assert_eq!(substr(s1, 0, 1), "0");
    assert_eq!(substr(s1, 0, 9), "012345678");
    assert_eq!(substr(s1, 0, 10), s1);
    assert_eq!(substr(s1, 2, 3), "234");

    assert_eq!(substr(s1, 9, 1), "9");
    assert_eq!(substr(s1, 9, 2), "9");
    assert_eq!(substr(s1, 8, 2), "89");
    assert_eq!(substr(s1, 5, 3), "567");

    // start_index < 0
    assert_eq!(substr(s1, -5, 3), "567");
    assert_eq!(substr(s1, -4, 4), "6789");
    assert_eq!(substr(s1, -10, 3), "012");
    assert_eq!(substr(s1, -10, 1), "0");
    assert_eq!(substr(s1, 0, 3), "012"); // Same as previous

    // length < 0
    assert_eq!(substr(s1, 5, -3), "345");
    assert_eq!(substr(s1, 0, -1), "0");
    assert_eq!(substr(s1, 0, -2), "0"); // Past boundary
    assert_eq!(substr(s1, 0, -5), "0"); // Past boundary
    assert_eq!(substr(s1, -5, -3), "345");
    assert_eq!(substr(s1, -1, -3), "789");

    // Empty string results
    assert_eq!(substr(s1, 2, 0), ""); // 0 length
    assert_eq!(substr(s1, 0, 0), ""); // 0 length
    assert_eq!(substr(s1, -4, 0), ""); // 0 length

    // Empty string search
    assert_eq!(substr("", 0, 11), "");
    assert_eq!(substr("", 0, 100), "");
    assert_eq!(substr("", 3, 2), "");
    assert_eq!(substr("", -3, 2), "");
    assert_eq!(substr("", -3, -2), "");
    assert_eq!(substr("", 100, -100), "");
    assert_eq!(substr("", isize::MAX, isize::MIN), "");
    assert_eq!(substr("", -100, 100), "");
    assert_eq!(substr("", isize::MIN, isize::MAX), "");

    // Past string boundaries
    assert_eq!(substr(s1, 0, 11), s1);
    assert_eq!(substr(s1, 0, 100), s1);
    assert_eq!(substr(s1, 10, 2), "");
    assert_eq!(substr(s1, 10, 1), "");
    assert_eq!(substr(s1, -11, 1), "");

    assert_eq!(substr(s1, -5, -100), "012345");
    assert_eq!(substr(s1, -5, isize::MIN), "012345");

    assert_eq!(substr(s1, -100, 100), "");
    assert_eq!(substr(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr(s1, -100, 1), "");
    assert_eq!(substr(s1, isize::MIN, 1), "");
    assert_eq!(substr(s1, isize::MIN, 2), "");

    assert_eq!(substr(s1, 5, -100), "012345");
    assert_eq!(substr(s1, 5, isize::MIN), "012345");

    assert_eq!(substr(s1, 2, 100), "23456789");
    assert_eq!(substr(s1, 2, isize::MAX), "23456789");

    assert_eq!(substr(s1, 100, 1), "");
    assert_eq!(substr(s1, isize::MAX, 1), "");

    assert_eq!(substr(s1, 100, -3), "");
    assert_eq!(substr(s1, isize::MAX, -3), "");

    assert_eq!(substr(s1, 100, -100), "");
    assert_eq!(substr(s1, isize::MAX, isize::MIN), "");

    assert_eq!(substr(s1, -100, 100), "");
    assert_eq!(substr(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr(s1, -100, -3), "");
    assert_eq!(substr(s1, isize::MIN, -3), "");

    // -------------------------------- str ------------------------------------

    assert_eq!(s1.substr(2, 3), "234");
    assert_eq!(s1.substr(-5, 3), "567");
    assert_eq!(s1.substr(-5, -3), "345");
    assert_eq!(s1.substr(5, -3), "345");

    // Empty string results
    assert_eq!(s1.substr(2, 0), ""); // 0 length
    assert_eq!(s1.substr(0, 0), ""); // 0 length
    assert_eq!(s1.substr(-4, 0), ""); // 0 length

    // Past string boundaries
    assert_eq!(s1.substr(2, 100), "23456789");
    assert_eq!(s1.substr(2, isize::MAX), "23456789");
    assert_eq!(s1.substr(2, isize::MIN), "012");
    assert_eq!(s1.substr(isize::MIN, isize::MAX), "");

    // ------------------------------- String ----------------------------------

    assert_eq!(s2.substr(2, 3), "234");
    assert_eq!(s2.substr(-5, 3), "567");
    assert_eq!(s2.substr(-5, -3), "345");
    assert_eq!(s2.substr(5, -3), "345");
    assert_eq!(s2.substr(2, 100), "23456789");

    // Empty string results
    assert_eq!(s2.substr(2, 0), ""); // 0 length
    assert_eq!(s2.substr(0, 0), ""); // 0 length
    assert_eq!(s2.substr(-4, 0), ""); // 0 length

    // Past string boundaries
    assert_eq!(s2.substr(2, 100), "23456789");
    assert_eq!(s2.substr(2, isize::MAX), "23456789");
    assert_eq!(s2.substr(2, isize::MIN), "012");
    assert_eq!(s2.substr(isize::MIN, isize::MAX), "");

    // Test with accented characters. Check UTF-8 code points.
    let s3: String = "Test 123 éèçà 123 test home".to_owned();

    assert_eq!(s3.substr(14, 3), "123");
    assert_eq!(s3.substr(23, 4), "home");
    assert_eq!(s3.substr(500, 4), "");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substru() {
    let s1: &str = "0123456789";

    assert_eq!(substru(s1, 0, 2), "01");
    assert_eq!(substru(s1, 2, 3), "234");
    assert_eq!(substru(s1, 0, 9), "012345678");
    assert_eq!(substru(s1, 0, 10), "0123456789");

    assert_eq!(substru(s1, 0, 1), "0");
    assert_eq!(substru(s1, 9, 1), "9");
    assert_eq!(substru(s1, 9, 2), "9");
    assert_eq!(substru(s1, 8, 2), "89");
    assert_eq!(substru(s1, 0, 3), "012");
    assert_eq!(substru(s1, 2, 0), ""); // 0 length
    assert_eq!(substru(s1, 0, 0), ""); // 0 length
    assert_eq!(substru(s1, 100, 0), ""); // 0 length
    assert_eq!(substru("", 3, 2), "");

    // Past boundary
    assert_eq!(substru(s1, 10, 1), "");
    assert_eq!(substru(s1, 100, 1), "");
    assert_eq!(substru(s1, 0, 11), s1);
    assert_eq!(substru(s1, 0, 100), s1);
    assert_eq!(substru(s1, usize::MAX, 1), "");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substr_end() {
    let s1: &str = "0123456789";
    let s2: String = "0123456789".to_owned();

    // get charcters util end of string

    assert_eq!(substr_end(s1, 0), s1);
    assert_eq!(substr_end(s1, 2), "23456789");
    assert_eq!(substr_end(s1, 9), "9");
    assert_eq!(substr_end(s1, 10), ""); // Out of bounds
    assert_eq!(substr_end(s1, -1), "9");
    assert_eq!(substr_end(s1, -3), "789");
    assert_eq!(substr_end(s1, -9), "123456789");
    assert_eq!(substr_end(s1, -10), s1);
    assert_eq!(substr_end(s1, -11), ""); // Out of bounds

    // Out of bounds
    assert_eq!(substr_end(s1, -100), "");
    assert_eq!(substr_end(s1, isize::MIN), "");
    assert_eq!(substr_end(s1, 100), "");
    assert_eq!(substr_end(s1, isize::MAX), "");

    // str
    assert_eq!(s1.substr_end(0), s1);
    assert_eq!(s1.substr_end(2), "23456789");
    assert_eq!(s1.substr_end(9), "9");
    assert_eq!(s1.substr_end(10), "");
    assert_eq!(s1.substr_end(-3), "789");

    // get charcters util end of string
    assert_eq!(s2.substr_end(0), s1);
    assert_eq!(s2.substr_end(2), "23456789");
    assert_eq!(s2.substr_end(9), "9");
    assert_eq!(s2.substr_end(10), "");
    assert_eq!(s2.substr_end(-3), "789");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substring() {
    let s1: &str = "0123456789";
    let s2: String = "0123456789".to_owned();

    assert_eq!(substring(s1, 2, 3), "2");
    assert_eq!(substring(s1, 2, 9), "2345678");
    assert_eq!(substring(s1, 2, 10), "23456789");
    assert_eq!(substring(s1, 2, 11), "23456789");
    assert_eq!(substring(s1, -2, 3), "012");
    assert_eq!(substring(s1, -2, 50), s1);
    assert_eq!(substring(s1, 9, 2), "2345678"); // Swap
    assert_eq!(substring(s1, 0, 1), "0");
    assert_eq!(substring(s1, 0, 2), "01");
    assert_eq!(substring(s1, -1, 1), "0");
    assert_eq!(substring(s1, -1, 0), "");
    assert_eq!(substring(s1, 0, 0), "");
    assert_eq!(substring(s1, 2, 2), "");

    assert_eq!(substring(s1, isize::MAX, 2), "23456789"); // Swap
    assert_eq!(substring(s1, isize::MIN, isize::MAX), s1);
    assert_eq!(substring(s1, isize::MAX, isize::MIN), s1); // Swap
    assert_eq!(substring(s1, 100, -100), s1); // Swap
    assert_eq!(substring(s1, -100, 100), s1);

    assert_eq!(s1.substring(2, 3), "2");
    assert_eq!(s1.substring(2, 9), "2345678");
    assert_eq!(s1.substring(2, 10), "23456789");
    assert_eq!(s1.substring(2, 11), "23456789");
    assert_eq!(s1.substring(-2, 3), "012");
    assert_eq!(s1.substring(-2, 50), "0123456789");
    assert_eq!(s1.substring(9, 2), "2345678"); // Swap

    assert_eq!(s2.substring(2, 3), "2");
    assert_eq!(s2.substring(2, 9), "2345678");
    assert_eq!(s2.substring(2, 10), "23456789");
    assert_eq!(s2.substring(2, 11), "23456789");
    assert_eq!(s2.substring(-2, 3), "012");
    assert_eq!(s2.substring(-2, 50), "0123456789");
    assert_eq!(s2.substring(9, 2), "2345678"); // Swap
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let s2: String = s1.to_owned();
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof(s1, "123", 8), Some(14));
    assert_eq!(indexof(s1, "Test", 0), Some(0));
    assert_eq!(indexof(s1, "home", 10), Some(23));
    assert_eq!(indexof(s1, "e", 0), Some(1));
    assert_eq!(indexof(s1, "T", 0), Some(0));
    assert_eq!(indexof(s1, "ç", 3), Some(11));
    assert_eq!(indexof(s1, "e", 26), Some(26));
    assert_eq!(indexof(s1, "e", 27), None); // Out of bounds
    assert_eq!(indexof(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof(s1, "", 0), None); // Search nothing
    assert_eq!(indexof(s1, "not found", 0), None); // Not found

    // str
    assert_eq!(s1.indexof(&searchstring, 0), Some(18));
    assert_eq!(s1.indexof(&searchstring, 10), Some(18));
    assert_eq!(s1.indexof("123", 8), Some(14));
    assert_eq!(s1.indexof("home", 10), Some(23));
    assert_eq!(s1.indexof("home", 100), None); // Out of bounds
    assert_eq!(s1.indexof("", 0), None); // Search nothing
    assert_eq!(s1.indexof("not found", 0), None); // Not found

    // String
    assert_eq!(s2.indexof(&searchstring, 0), Some(18));
    assert_eq!(s2.indexof(&searchstring, 10), Some(18));
    assert_eq!(s2.indexof("123", 8), Some(14));
    assert_eq!(s2.indexof("home", 10), Some(23));
    assert_eq!(s2.indexof("home", 100), None); // Out of bounds
    assert_eq!(s2.indexof("", 0), None); // Search nothing
    assert_eq!(s2.indexof("not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_str_remove() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let s2: String = s1.to_owned();

    assert_eq!(str_remove(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
    assert_eq!(str_remove(s1, 500, 500), s1); // Remove characters that don't exist. Out of bounds.

    // str
    assert_eq!(s1.str_remove(14, 4), "Test 123 éèçà test home");
    assert_eq!(s1.str_remove(3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(s1.str_remove(0, 0), s1); // Remove nothing.
    assert_eq!(s1.str_remove(500, 2), s1); // Remove characters that don't exist. Out of bounds.

    // String
    assert_eq!(s2.str_remove(14, 4), "Test 123 éèçà test home");
    assert_eq!(s2.str_remove(3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(s2.str_remove(0, 0), s2); // Remove nothing.
    assert_eq!(s2.str_remove(500, 2), s2); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_str_concat() {
    // let s1: String = "test éèçà 123 test".to_owned(); // Also correct
    let s1: &str = "test éèçà";
    let s2: &str = "test";
    let s3: String = "éèçà".to_owned();

    assert_eq!(str_concat!(s1, " ", s2, " ", &s3), "test éèçà test éèçà");
}

// -----------------------------------------------------------------------------
