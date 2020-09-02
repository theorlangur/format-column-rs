use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub mod separators;
pub mod assignment;

#[derive(Debug)]
pub struct AnalyzeErr{
    
}

impl std::fmt::Display for AnalyzeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "analyzer error")
    }
}

impl std::error::Error for AnalyzeErr {
}

pub trait LineAnalyzer
{
    fn clear(&mut self){}
    fn analyze_line<'a>(&mut self, _fmt :&mut Formatter, _l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>{Err(AnalyzeErr{})}
}
