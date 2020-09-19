use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;
use crate::analyzers::separators::Analyzer as SepAnalyzer;
use crate::analyzers::separators::Boundary;
use crate::analyzers::separators::BoundType;

/*
 * support for the following format
 * / * some comment herer * / {***, ***, ****},
*/

pub struct Analyzer {
    sepa : SepAnalyzer,
}

struct KeyPoints
{
    c_begin : usize,
    c_end : usize,
    block_begin : usize,
}

impl Analyzer {
    pub fn new()->Self
    {
        Self{sepa:SepAnalyzer::new()}
    }

   fn find_key_points(&self, s :&str)->Result<KeyPoints, AnalyzeErr> 
   {
        let c_begin = s.find_str("/*")?;
        let c_end = s.find_str("*/")?;
        let block_begin = s.sym('{')?;
        let block_end = s.rsym('}')?;
        
        if c_begin > c_end || block_begin < c_end || block_end < block_begin {
            Err(AnalyzeErr{})
        }else {
            Ok(KeyPoints{c_begin, c_end, block_begin})
        }
   }
}


impl LineAnalyzer for Analyzer {

    fn clear(&mut self)
    {
        self.sepa.clear();
        self.sepa.set_new_column_separators(vec![',', ' ']);
        self.sepa.set_separators(vec![',', '}']);
        self.sepa.add_boundary(Boundary::new_sym('"', 1), BoundType::Exclude);
        self.sepa.add_boundary(Boundary::new_asym('{', '}', 1), BoundType::Include);
    }
    
    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        self.find_key_points(s)?;
        Ok(())
    }
    
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let KeyPoints{c_begin, c_end, block_begin} = self.find_key_points(l.s)?;
        fmt.add_column(c_begin, c_begin + 2, '\0',  l);
        fmt.add_column(c_begin + 1, c_end, '\0',  l);
        fmt.add_column(c_end, c_end + 2, ' ',  l);
        //parse {} block here
        self.sepa.analyze_substr(fmt, &l.s[block_begin..], block_begin, l)
    }
}