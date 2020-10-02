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
    bit_begin : usize,
    bit_end : usize,
    rest_begin : usize,
    rest_end : usize,
}

impl Analyzer
{
   fn find_key_points(&self, s :&str)->Result<KeyPoints, AnalyzeErr> 
   {
        let colon_pos = s.sym(':')?;
        
        let var_end = s[..colon_pos].rfind_nwhite()?;
        let var_begin = s[..var_end].rfind_white()? + 1;
        let type_end = s[..var_begin].rfind_nwhite()?;
        let type_begin = s.find_nwhite()?;
        let bit_begin = s[colon_pos + 1..].find_nwhite()? + colon_pos + 1;
        let bit_end = s[bit_begin..].sym(';')? + bit_begin;
        s[bit_begin..bit_end].parse::<u32>()?;
        let t = if bit_end + 1 < s.len() {
            s[bit_end + 1..].find_nwhite()
        }else {
           Err(AnalyzeErr{}) 
        };

        let rest_begin;
        let rest_end;
        
        if let Ok(rb) = t {
            rest_begin = rb + bit_end + 1;
            rest_end = s.rfind_nwhite()?;
        }else {
            rest_begin = s.len();
            rest_end = s.len();
        }
        
        Ok(KeyPoints{var_begin, var_end, type_begin, type_end, bit_begin, bit_end, rest_begin, rest_end})
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
        let KeyPoints{var_begin, var_end, type_begin, type_end, bit_begin, bit_end, rest_begin, rest_end} = self.find_key_points(l.s)?;

        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(var_begin, var_end + 1, ':', l);
        let rest_exist = rest_begin < l.s.len();
        let sep = if rest_exist {' '} else {'\0'};
        fmt.add_column(bit_begin, bit_end + 1, sep, l);
        if rest_exist {
            fmt.add_column(rest_begin, rest_end + 1, '\0', l);
        }
        Ok(())
    }
}