use std::fs::read_to_string;

use split_tool::libresplit::lssplitfile::LibreSplitSplitFile;

#[test]
fn test_construct_from_titles_list() {
    // Create LibreSplit file from vector of strings.
    let titles: Vec<String> = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
    let libresplit = LibreSplitSplitFile::from_titles_list("Test".to_string(), titles).to_json();

    // Compare to expected.
    let expected =
        read_to_string("tests/files/titles.json").expect("Failed to read expected output");
    assert_eq!(libresplit, expected);
}
