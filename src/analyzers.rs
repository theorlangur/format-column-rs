use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub mod separators;

pub trait LineAnalyzer
{
    fn clear(&mut self){}
    fn analyze_line<'a>(&mut self, _fmt :&mut Formatter, _l: &mut LineDescr<'a>){}
}
