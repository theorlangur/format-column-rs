use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub mod separators;
pub mod assignment;
pub mod func_decl;
pub mod xml_attr;
pub mod var_decl;
pub mod bit_field;
pub mod cmnt_struct;
pub mod func_call;

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

impl std::convert::From<std::num::ParseIntError> for AnalyzeErr{
    fn from(_err : std::num::ParseIntError) -> Self
    {
        Self{}
    }
}

pub trait LineAnalyzer
{
    fn clear(&mut self){}
    fn can_accept(&self, _s :&str)->Result<(),AnalyzeErr> {Err(AnalyzeErr{})}
    fn analyze_line<'a>(&mut self, _fmt :&mut Formatter, _l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>{Err(AnalyzeErr{})}
    fn type_name(&self)->&'static str {std::any::type_name::<Self>()}
    fn parse_args(&mut self, _arg_it :std::slice::Iter<String>) -> Result<(), Box<dyn std::error::Error>> {Ok(())}
}

pub trait LineParser
{
    fn rfind_nwhite(&self)->Result<usize, AnalyzeErr>;
    fn rfind_white(&self)->Result<usize, AnalyzeErr>;
    fn find_nwhite(&self)->Result<usize, AnalyzeErr>;
    fn find_white(&self)->Result<usize, AnalyzeErr>;
    fn sym(&self, c: char)->Result<usize, AnalyzeErr>;
    fn rsym(&self, c: char)->Result<usize, AnalyzeErr>;
    fn expect_sym(&self, c : char)->Result<(), AnalyzeErr>;
    fn find_str(&self, s: &str)->Result<usize, AnalyzeErr>;
    fn rfind_str(&self, s: &str)->Result<usize, AnalyzeErr>;
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
    fn rsym(&self, c: char)->Result<usize, AnalyzeErr> {
        self.rfind(c).ok_or(AnalyzeErr{})
    }

    fn expect_sym(&self, c : char)->Result<(), AnalyzeErr>
    {
        if let Some(ch) = self.chars().next() {
            if ch == c {
                return Ok(());
            }
        }
        return Err(AnalyzeErr{});
    }

    fn find_str(&self, s: &str)->Result<usize, AnalyzeErr>
    {
        self.find(s).ok_or(AnalyzeErr{})
    }

    fn rfind_str(&self, s: &str)->Result<usize, AnalyzeErr>
    {
        self.rfind(s).ok_or(AnalyzeErr{})
    }
}