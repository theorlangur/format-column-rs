use super::LineAnalyzer;
use super::AnalyzeErr;
use super::LineParser;
use crate::column_tools::Formatter;
use crate::column_tools::LineDescr;

pub struct TypeVarAnalyzer
{
}

struct TypeVarKeyPoints
{
    var_begin : usize,
    var_end : usize,
    type_begin : usize,
    type_end : usize,
    expr_begin : usize,
}

impl TypeVarAnalyzer
{
   fn find_key_points(&self, s :&str)->Result<TypeVarKeyPoints, AnalyzeErr> 
   {
        let assign_pos = s.sym('=')?;
        if assign_pos + 1 >= s.len() { return Err(AnalyzeErr{}); }
        
        let var_end = s[..assign_pos].rfind_nwhite()?;
        let var_begin = s[..var_end].rfind_white()? + 1;
        let type_end = s[..var_begin].rfind_nwhite()?;
        let type_begin = s.find_nwhite()?;

        let expr_begin = s[assign_pos + 1..].find_nwhite()? + assign_pos + 1;

        Ok(TypeVarKeyPoints{var_begin, var_end, type_begin, type_end, expr_begin})
   }
}

impl LineAnalyzer for TypeVarAnalyzer
{
    fn clear(&mut self)
    {
    }
   
    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        self.find_key_points(s)?;
        Ok(())
    }

    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let TypeVarKeyPoints{var_begin, var_end, type_begin, type_end, expr_begin} = self.find_key_points(l.s)?;
        fmt.add_column(type_begin, type_end + 1, ' ', l);
        fmt.add_column(var_begin, var_end + 1, '=', l);
        fmt.add_column(expr_begin, l.s.len(), '\0', l);
        Ok(())
    }
}

pub struct VarAnalyzer
{
}

struct VarKeyPoints
{
    before_begin : usize,
    before_end : usize,
    after_begin : usize,
    after_end : usize,
}

impl VarAnalyzer
{
   fn find_key_points(&self, s :&str)->Result<VarKeyPoints, AnalyzeErr> 
   {
        let assign_pos = s.sym('=')?;
        if assign_pos + 1 >= s.len() { return Err(AnalyzeErr{}); }
        
        let before_end = s[..assign_pos].rfind_nwhite()?;
        let before_begin = s[..assign_pos].find_white()?;
        let after_begin = s[assign_pos + 1..].find_nwhite()? + assign_pos + 1;
        let after_end = s[assign_pos + 1..].rfind_nwhite()? + assign_pos + 1;

        Ok(VarKeyPoints{before_begin, before_end, after_begin, after_end})
   }
}


impl LineAnalyzer for VarAnalyzer
{
    fn clear(&mut self)
    {
    }
   
    fn can_accept(&self, s :&str)->Result<(),AnalyzeErr> 
    {
        self.find_key_points(s)?;
        Ok(())
    }

    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)->Result<(),AnalyzeErr>
    {
        let VarKeyPoints{before_begin, before_end, after_begin, after_end} = self.find_key_points(l.s)?;
        fmt.add_column(before_begin, before_end + 1, '=', l);
        fmt.add_column(after_begin, after_end + 1, '\0', l);
        Ok(())
    }
}