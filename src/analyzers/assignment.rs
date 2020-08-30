use super::LineAnalyzer;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer
{
}

impl Analyzer
{
    
}

impl LineAnalyzer for Analyzer
{
    fn clear(&mut self)
    {
    }

    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)
    {
        let opt = l.s.find('=');
        if opt.is_none() { return; }
        let assign_pos = opt.unwrap();

        if assign_pos + 1 >= l.s.len() { return; }
        
        let opt = l.s[..assign_pos - 1].rfind(|c:char|!c.is_ascii_whitespace());
        if opt.is_none() { return; }
        let var_end = opt.unwrap();
        
        let opt = l.s[..var_end].rfind(|c:char|c.is_ascii_whitespace());
        if opt.is_none() { return; }
        let var_begin = opt.unwrap() + 1;
       
        let opt = l.s[..var_begin - 1].rfind(|c:char|!c.is_ascii_whitespace());
        if opt.is_none() { return; }
        let type_end = opt.unwrap();
        
        let opt = l.s.find(|c:char|!c.is_ascii_whitespace());
        let type_begin = opt.unwrap();

        let opt = l.s[assign_pos + 1..].find(|c:char|!c.is_ascii_whitespace());
        let expr_begin = opt.unwrap() + assign_pos + 1;

        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(var_begin, var_end + 1, '=', l);
        fmt.add_column(expr_begin, l.s.len(), '\0', l);
    }
}