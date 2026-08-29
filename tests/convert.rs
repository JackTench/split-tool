use std::fs::read_to_string;

use quick_xml::de::from_str;
use split_tool::converter::{ConvertableSplitFile, livesplit::livesplitsplitfile::LiveSplitRun};

#[test]
fn test_convert_lss_libre() {
    // Read input LSS file.
    let input = read_to_string("tests/files/test_input.lss")
        .expect("Failed to read test_input.lss");

    // Convert to LibreSplit file.
    let livesplit: LiveSplitRun = from_str(&input).expect("Failed to read LSS from file");
    let libresplit = livesplit.convert().to_json();

    // Compare to expected.
    let expected = read_to_string("tests/files/test_cont_output.json")
        .expect("Failed to read expected output");
    assert_eq!(libresplit, expected);
}
