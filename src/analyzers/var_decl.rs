use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer {
}

struct KeyPoints
{
    var_begin : usize,
    var_end : usize,
    type_begin : usize,
    type_end : usize,
}

impl Analyzer
{
   fn find_key_points(&self, s :&str)->Result<KeyPoints, AnalyzeErr> 
   {
        let semi_pos = s.sym(';')?;
        
        let var_end = s[..semi_pos].rfind_nwhite()?;
        let var_begin = s[..var_end].rfind_white()? + 1;
        let type_end = s[..var_begin].rfind_nwhite()?;
        let type_begin = s.find_white()?;
        
        Ok(KeyPoints{var_begin, var_end, type_begin, type_end})
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
        let KeyPoints{var_begin, var_end, type_begin, type_end} = self.find_key_points(l.s)?;

        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(var_begin, var_end + 1, ';', l);
        Ok(())
    }
}