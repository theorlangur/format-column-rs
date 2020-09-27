mod test_bit_field;
mod test_var_decl;
mod test_func_decl;
mod test_assignment;
mod test_xml;
mod test_cmnt_structs;
mod test_separators;
mod test_auto_detect;
mod test_func_call;

#[cfg(test)]
pub mod mytests {
    use crate::analyzers::LineAnalyzer;
    use crate::column_tools::LineDescr;
    use crate::column_tools::Formatter;
    use crate::column_tools::Printer;
    use crate::column_tools::write_lines_into;


    pub fn run_analyzer(in_s :&str, la :&mut dyn LineAnalyzer, mut fmtr :Formatter, mut printer :Printer)->String{
       let lines_str : Vec<String> = in_s.lines().map(|x|x.to_string()).collect();

       let mut lines: Vec<LineDescr> = Vec::new();

        lines.reserve(lines_str.len());
        lines_str.iter().for_each(|l|{
           let mut line = LineDescr::new(&l);
           fmtr.analyze_line(la, &mut line);
           lines.push(line); 
        });

        fmtr.finish();

        printer.set_formatter(fmtr);

        let mut v = Vec::new();
        let out : &mut dyn std::io::Write = &mut v;
        
        write_lines_into(&lines, &printer, out).unwrap();
        
        std::str::from_utf8(&v).unwrap().to_string()
    }
    
    fn cmp_str(result :&str, expected :&str)->isize {
        let mut res_chars = result.chars();
        let mut exp_chars = expected.chars();
        
        let mut idx : isize = 0;
        
        loop {
            let rc = res_chars.next();
            let ec = exp_chars.next();
            
            if rc.is_some() != ec.is_some() {
                break;
            }
            
            if rc.is_none() {
                //both are actually None at this point. so they are same
                idx = -1;
                break;
            }
            
            if rc.unwrap() != ec.unwrap() {
                //different
                break;
            }
            idx += 1;
        };
        
        idx
    }
    
    pub fn assert_eq(result :&str, expected :&str) {
        assert_eq!(result.lines().count(), expected.lines().count());

        let res_lines = result.lines();
        let exp_lines = expected.lines();
        
        let mut line_idx = 0;
        let cmp_lines = res_lines.into_iter().zip(exp_lines.into_iter());
        cmp_lines.for_each(|(res, exp)|{
            let d = cmp_str(res, exp);
            assert!(d == -1, "Diff at line {0}.\nResult:\n{1}|\n{cur:>pad$}\nExpected:\n{2}|\n{cur:>pad$}", line_idx, res, exp, pad = (d + 1) as usize, cur = '^');
            line_idx += 1;
        });
    }
}