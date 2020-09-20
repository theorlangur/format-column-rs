use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer {
}

struct KeyPoints
{
    fn_begin : usize,
    fn_end : usize,
    type_begin : usize,
    type_end : usize,
    paren_pos : usize,
}

impl Analyzer
{
   fn find_key_points(&self, s :&str)->Result<KeyPoints, AnalyzeErr> 
   {
        let paren_pos = s.sym('(')?;
        if paren_pos + 1 >= s.len() { return Err(AnalyzeErr{}); }
        
        if let Some(_) = s[..paren_pos].find('=') {
            return Err(AnalyzeErr{});
        }
        
        let fn_end = s[..paren_pos].rfind_nwhite()?;
        let fn_begin = s[..fn_end].rfind_white()? + 1;
        let type_end = s[..fn_begin].rfind_nwhite()?;
        let type_begin = s.find_white()?;

        Ok(KeyPoints{fn_begin, fn_end, type_begin, type_end, paren_pos})
   }
}


impl LineAnalyzer for Analyzer {
    
    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        self.find_key_points(s)?;
        Ok(())
    }
    
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let KeyPoints{paren_pos, fn_begin, fn_end, type_begin, type_end} = self.find_key_points(l.s)?;
        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(fn_begin, fn_end + 1, '\0', l);
        fmt.add_column(paren_pos, l.s.len(), '\0', l);
        Ok(())
    }
}

