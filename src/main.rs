use std::{fs::read_to_string, io::Cursor};

use spex::parsing::XmlReader;

use crate::converter::{ConvertableSplitFile, livesplit::livesplitsplitfile::LiveSplitSplitFile};

mod converter;
mod libresplit;

fn main() {
    // Convert test LiveSplit file.
    let file = read_to_string("test.lss").unwrap();
    let cursor = Cursor::new(file);
    let xml = XmlReader::parse_auto(cursor)
        .map_err(|e| e.to_string())
        .unwrap();
    let livesplit = LiveSplitSplitFile::new(xml);
    let libresplit = livesplit.convert();

    println!("{:?}", libresplit);
}
