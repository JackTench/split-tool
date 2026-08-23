use std::fs::{read_to_string, write};

use quick_xml::de::from_str;

use crate::converter::{ConvertableSplitFile, livesplit::livesplitsplitfile::LiveSplitRun};

mod converter;
mod libresplit;

fn main() {
    // Convert test LiveSplit file.
    let file = read_to_string("test.lss").unwrap();
    let livesplit: LiveSplitRun = from_str(&file).unwrap();
    let libresplit = livesplit.convert().to_json();
    write("test.json", libresplit);
}
