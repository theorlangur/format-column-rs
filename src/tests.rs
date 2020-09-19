#[cfg(test)]
mod tests{
    use crate::analyzers::LineAnalyzer;
    use crate::column_tools::LineDescr;
    use crate::column_tools::Formatter;
    use crate::column_tools::Printer;

    pub fn run_analyzer(in_s :&str, la :&mut dyn LineAnalyzer, fmtr :&mut Formatter, printer :&Printer)->String{
       let lines_str : Vec<String> = in_s.lines().map(|x|x.to_string()).collect();
       let mut lines: Vec<LineDescr> = Vec::new();

        lines.reserve(lines_str.len());
        lines_str.iter().for_each(|l|{
           let mut line = LineDescr::new(&l);
           fmtr.analyze_line(la, &mut line);
           lines.push(line); 
        });

        fmtr.finish();

        let mut v = Vec::new();
        let out : &mut dyn std::io::Write = &mut v;
        for l in lines.iter()
        {
            if let Some(s) = printer.format_line(&l) {
                out.write(s.as_bytes()).unwrap();
            }
        }
        
       std::str::from_utf8(&v).unwrap().to_string()
    }
}