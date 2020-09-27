use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::analyzers::separators::Analyzer as SepAnalyzer;
use crate::analyzers::separators::Boundary;
use crate::analyzers::separators::BoundType;
use crate::column_tools::LineDescr;

pub struct Analyzer {
    sepa : SepAnalyzer,
}

struct KeyPoints
{
    fn_begin : usize,
    fn_end : usize,
    paren_pos : usize,
}

impl Analyzer
{
    pub fn new()->Self
    {
        Self{sepa:SepAnalyzer::new()}
    }

   fn find_key_points(&self, s :&str)->Result<KeyPoints, AnalyzeErr> 
   {
        let paren_pos = s.sym('(')?;
        if paren_pos + 1 >= s.len() { return Err(AnalyzeErr{}); }
        
        if let Some(_) = s[..paren_pos].find('=') {
            return Err(AnalyzeErr{});
        }
        
        let fn_end = s[..paren_pos].rfind_nwhite()?;
        let fn_begin = s[..fn_end].find_nwhite()?;
        
        Ok(KeyPoints{fn_begin, fn_end, paren_pos})
   }
}


impl LineAnalyzer for Analyzer {
    fn clear(&mut self)
    {
        self.sepa.clear();
        self.sepa.set_new_column_separators(vec![',', ' ']);
        self.sepa.set_separators(vec![',', ')']);
        self.sepa.add_boundary(Boundary::new_sym('"', 1), BoundType::Exclude);
        self.sepa.add_boundary(Boundary::new_asym('{', '}', 1), BoundType::Exclude);
        self.sepa.add_boundary(Boundary::new_asym('[', ']', 1), BoundType::Exclude);

        self.sepa.add_boundary(Boundary::new_asym('(', ')', 1), BoundType::Include);
    }

    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        self.find_key_points(s)?;
        Ok(())
    }
    
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let KeyPoints{paren_pos, fn_begin, fn_end} = self.find_key_points(l.s)?;
        fmt.add_column(fn_begin, fn_end + 1, '\0', l);
        self.sepa.analyze_substr(fmt, &l.s[paren_pos..], paren_pos, l)
    }
}

