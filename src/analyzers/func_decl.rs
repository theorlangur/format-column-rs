use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer {
}


impl LineAnalyzer for Analyzer {
    
    fn clear(&mut self)
    {
    }

    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let paren_pos = l.s.sym('(')?;
        if paren_pos + 1 >= l.s.len() { return Err(AnalyzeErr{}); }
        
        let fn_end = l.s[..paren_pos].rfind_nwhite()?;
        let fn_begin = l.s[..fn_end].rfind_white()? + 1;
        let type_end = l.s[..fn_begin].rfind_nwhite()?;
        let type_begin = l.s.find_white()?;

        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(fn_begin, fn_end + 1, '\0', l);
        fmt.add_column(paren_pos, l.s.len(), '\0', l);
        Ok(())
    }
}

