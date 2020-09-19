mod test_bit_field;
mod test_var_decl;
mod test_func_decl;
mod test_assignment;
mod test_xml;

#[cfg(test)]
pub mod mytests {
    use crate::analyzers::LineAnalyzer;
    use crate::column_tools::LineDescr;
    use crate::column_tools::Formatter;
    use crate::column_tools::Printer;
    use crate::write_lines_into;


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
}