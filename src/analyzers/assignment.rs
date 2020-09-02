use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
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

    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let assign_pos = l.s.sym('=')?;
        if assign_pos + 1 >= l.s.len() { return Err(AnalyzeErr{}); }
        
        let var_end = l.s[..assign_pos].rfind_nwhite()?;
        let var_begin = l.s[..var_end].rfind_white()? + 1;
        let type_end = l.s[..var_begin].rfind_nwhite()?;
        let type_begin = l.s.find_nwhite()?;

        let expr_begin = l.s[assign_pos + 1..].find_nwhite()? + assign_pos + 1;

        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(var_begin, var_end + 1, '=', l);
        fmt.add_column(expr_begin, l.s.len(), '\0', l);
        Ok(())
    }
}