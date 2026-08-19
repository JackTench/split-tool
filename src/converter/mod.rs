use crate::libresplit::lssplitfile::LibreSplitSplitFile;

pub trait ConvertableSplitFile {
    fn convert(&self) -> LibreSplitSplitFile;
}
