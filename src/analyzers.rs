use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub mod separators;
pub mod assignment;
pub mod func_decl;

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

pub trait LineParser
{
    fn rfind_nwhite(&self)->Result<usize, AnalyzeErr>;
    fn rfind_white(&self)->Result<usize, AnalyzeErr>;
    fn find_nwhite(&self)->Result<usize, AnalyzeErr>;
    fn find_white(&self)->Result<usize, AnalyzeErr>;
    fn sym(&self, c: char)->Result<usize, AnalyzeErr>;
}

impl LineParser for str {
    fn rfind_nwhite(&self)->Result<usize, AnalyzeErr> {
        self.rfind(|c:char|!c.is_ascii_whitespace()).ok_or(AnalyzeErr{})
    }
    fn rfind_white(&self)->Result<usize, AnalyzeErr> {
        self.rfind(|c:char|c.is_ascii_whitespace()).ok_or(AnalyzeErr{})
    }
    fn find_nwhite(&self)->Result<usize, AnalyzeErr>{
        self.find(|c:char|!c.is_ascii_whitespace()).ok_or(AnalyzeErr{})
    }
    fn find_white(&self)->Result<usize, AnalyzeErr> {
        self.find(|c:char|c.is_ascii_whitespace()).ok_or(AnalyzeErr{})
    }
    fn sym(&self, c: char)->Result<usize, AnalyzeErr> {
        self.find(c).ok_or(AnalyzeErr{})
    }
}