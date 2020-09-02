use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer {
}


impl LineAnalyzer for Analyzer {
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        //TODO: implement
        Ok(())
    }
}