use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct Analyzer {
}


impl LineAnalyzer for Analyzer {
    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        let start = s.find_nwhite()?;
        s[start..].expect_sym('<')?;
        let end = s.rfind_nwhite()?;
        s[end..].expect_sym('>')?;
        Ok(())
    }
    
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let start = l.s.find_nwhite()?;
        l.s[start..].expect_sym('<')?;
        let end = l.s.rfind_nwhite()?;
        l.s[end..].expect_sym('>')?;
        let tag_end = l.s[start + 1..end].find_white()? + 1 + start;

        let close_beg;
        if let Some(lc) = l.s[..end].chars().last() {
           if lc == '/' {
             close_beg = end - 1;  
           }else {
               close_beg = end;
           }
        }else {
           close_beg = end;
        }
        
        //adding '<'
        fmt.add_column(start, tag_end, ' ', l);
        let ins = &l.s[tag_end .. close_beg];

        let mut search = 0;

        while let Ok(lprop_beg) = ins[search..].find_nwhite() {
            let help_prop = search + lprop_beg;
            let assign = ins[help_prop..].sym('=')?;
            let help_assign = help_prop + assign;
            let lprop_end = ins[help_prop..help_assign].rfind_nwhite()?;
            let lq_beg = ins[help_assign + 1..].sym('"')?;
            let lq_end = ins[help_assign + 1 + lq_beg + 1..].sym('"')?;

            let p_beg = help_prop + tag_end;
            let p_end = p_beg + lprop_end + 1;

            let v_beg = help_assign + lq_beg + 1 + tag_end;
            let v_end = v_beg + 1 + lq_end + 1;
            
            //property name
            fmt.add_column(p_beg, p_end, '=', l);
            //property value
            fmt.add_column(v_beg, v_end, ' ', l);

            search = v_end - tag_end;
        }
        
        //adding '>'
        fmt.add_column(close_beg, end + 1, '\0', l);
        
        Ok(())
    }
}