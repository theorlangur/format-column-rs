use crate::analyzers::LineAnalyzer;
/****************************************************
 * AddToString trait
 * 
 ***************************************************/
trait AddToString
{
    fn add_to_string(&self, dest :&mut String);
    fn own_size(&self)->usize;
}

impl AddToString for String
{
    fn add_to_string(&self, dest :&mut String)
    {
        dest.push_str(self);
    }
    
    fn own_size(&self)->usize {
        self.len()
    }
}

impl AddToString for &str
{
    fn add_to_string(&self, dest :&mut String)
    {
        dest.push_str(self);
    }
    
    fn own_size(&self)->usize {
        self.len()
    }
}

impl AddToString for char
{
    fn add_to_string(&self, dest :&mut String)
    {
        dest.push(*self);
    }
    
    fn own_size(&self)->usize {
        std::mem::size_of::<char>()
    }
}


pub fn write_lines_into(lines :&Vec<LineDescr>, printer :&Printer, out :&mut dyn std::io::Write)->Result<(), Box<dyn std::error::Error>> {
    let mut first_line = true;
    for l in lines.iter()
    {
        if let Some(s) = printer.format_line(&l) {
            if !first_line {
                out.write("\n".as_bytes())?;
            }else{
                first_line = false;
            }
            out.write(s.as_bytes())?;
        }
    }
    out.flush()?;
    Ok(())
}

#[derive(Debug)]
pub struct ParseErr{
    
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl std::error::Error for ParseErr {
}

impl std::convert::From<std::num::ParseIntError> for ParseErr{
    fn from(_err : std::num::ParseIntError) -> Self
    {
        Self{}
    }
}

pub enum Align {
    Left,
    Center,
    Right
}

impl std::str::FromStr for Align {
    type Err = ParseErr;

    fn from_str(s :&str) -> Result<Align, Self::Err>
    {
       match s.to_lowercase().as_str() {
           "left" => Ok(Align::Left),
           "right" => Ok(Align::Right),
           "center" => Ok(Align::Center),
           &_ => Err(Self::Err{}),
       }
    }
}

fn align_string<T>(target :T, fill_str :&str, delta :usize, a : &Align)->String
where T:AddToString
{
   let mut res = String::with_capacity(fill_str.len() * delta + target.own_size());

   if let Align::Right = a {
       res.push_str(&fill_str.repeat(delta));
   }else if let Align::Center = a {
       res.push_str(&fill_str.repeat(delta / 2));
   }
   
   target.add_to_string(&mut res);
   
   if let Align::Left = a {
       res.push_str(&fill_str.repeat(delta));
   }else if let Align::Center = a {
       res.push_str(&fill_str.repeat(delta / 2 + delta % 2));
   }
   
   res
}


struct Column<'a>
{
    col : &'a str,
    sep : char
}

pub struct LineDescr<'a>
{
    pub s : &'a str,
    columns : Vec<Column<'a>>
}

impl<'a> LineDescr<'a>{
    pub fn new(s : &'a str) -> LineDescr<'a>
    {
        LineDescr{s, columns : Vec::new()}
    }
}

pub struct Formatter
{
    columns : Vec<usize>,
    total_size : usize,
    line_starts_to_ignore : Vec<String>,
    add_pre_start : bool,
}

impl Formatter
{
    pub fn new()->Self
    {
        Self{columns:Vec::new(), total_size: 0, line_starts_to_ignore : Vec::new(), add_pre_start : false}
    }

    pub fn clear(&mut self)
    {
        self.set_line_starts_to_ignore(vec![]);
        self.add_pre_start = false;
    }

    pub fn set_add_pre_start(&mut self, val : bool)
    {
        self.add_pre_start = val;
    }

    pub fn set_line_starts_to_ignore(&mut self, vals : Vec<String>) {
        self.line_starts_to_ignore = vals;
    }

    fn check_biggest_column(&mut self, idx : usize, sz :usize)
    {
        if self.columns.len() <= idx {
            self.columns.resize(idx + 1, 0);
        }
        if self.columns[idx] < sz {
            self.columns[idx] = sz;
        }
    }

    pub fn add_column<'b>(&mut self, begin:usize, end:usize, ch : char, l : &mut LineDescr<'b>)
    {
        if self.add_pre_start && l.columns.is_empty() {
            l.columns.push(Column{col : &l.s[..], sep : '\0'});
            self.check_biggest_column(0, 0);
        }
        
        let cnt = end - begin;
        l.columns.push(Column{col : &l.s[begin..end], sep : ch});
        self.check_biggest_column(l.columns.len() - 1, cnt);
    }

    pub fn finish(&mut self)
    {
        self.total_size = self.columns.iter().sum();
        
    }

    pub fn check_line_start_to_ignore(&self, l: &str) -> bool{
        self.line_starts_to_ignore.iter().any(|s|l.starts_with(s))
    }

    pub fn analyze_line<'a>(&mut self, analyzer :&mut dyn LineAnalyzer, l: &mut LineDescr<'a>)
    {
        if let Ok(_) = analyzer.analyze_line(self, l){
            if !l.columns.is_empty() && self.add_pre_start {
                let ps = l.s.as_ptr();
                let pf = l.columns[1].col.as_ptr();
                let first = pf as usize - ps as usize;
                
                l.columns[0] = Column{col : &l.s[..first], sep : '\0'};
                if self.columns[0] < first {
                    self.columns[0] = first;
                }
            }
        }else
        {
            l.columns.clear();
        }
    }

    pub fn parse_args(&mut self, mut arg_it :std::slice::Iter<String>) -> Result<(), Box<dyn std::error::Error>> {
        loop 
        {
            if let Some(arg) = arg_it.next() {
               if arg == "--line_start_to_ignore" {
                   if let Some(ignore) = arg_it.next() {
                        self.line_starts_to_ignore.push(ignore.clone());
                   }
               }else if arg == "--prestart" {
                   self.add_pre_start = true;
               }
            }else {
                break;
            }
        }
        Ok(())
    }
}

pub struct SeparatorConfig
{
    sep : char,
    fill : char,
    count : u8,
    align : Align,
    sep_str : String,
}

impl SeparatorConfig {
   pub fn new(sep: char, fill : char, count :u8, align : Align)->Self 
   {
       let sep_str = align_string(sep, &fill.to_string(), count as usize, &align);
       Self{sep, fill, count, align, sep_str}
   }
}

impl std::str::FromStr for SeparatorConfig {
    type Err = ParseErr;

    fn from_str(s :&str) -> Result<Self, Self::Err>
    {
        let mut it = s.split(':');
        if let Some(sep) = it.next() {
            
            if sep.len() == 1 {
               let mut res = SeparatorConfig{sep : sep.chars().next().unwrap(), fill : ' ', count : 1, align : Align::Left, sep_str: String::new() }; 
               if let Some(f) = it.next() {
                   res.fill = f.chars().next().unwrap();
               }
               if let Some(c) = it.next() {
                   res.count = c.parse::<u8>()?;
               }
               if let Some(a) = it.next() {
                   res.align = a.parse::<Align>()?;
               }
               res.sep_str = align_string(res.sep, &res.fill.to_string(), res.count as usize, &res.align);
               return Ok(res);
            }
        }
        
        Err(Self::Err{})
    }
}

pub struct Printer
{
    fill : char,
    fill_count : u8,
    join : String,
    align : Align,
    fmt : Option<Formatter>,
    non_matched_as_is : bool,//lines with not exactly amount of columns will be written as is
    sep_joins : Vec<SeparatorConfig>,
}

impl Printer{
    pub fn new(align:Align, fill : char, fill_count : u8, join : String, non_matched_as_is : bool) -> Self
    {
        Self{fill, align, fmt:None, fill_count, join, non_matched_as_is, sep_joins : Vec::new()}
    }

    pub fn default() -> Self
    {
        Self{fill : ' ', align : Align::Center, fmt:None, fill_count : 0, join : String::new(), non_matched_as_is : false, sep_joins : Vec::new()}
    }
    
    pub fn set_formatter(&mut self, fmt :Formatter) {
        self.fmt = Some(fmt);
    }

    pub fn set_separator_configs(&mut self, cfgs : Vec<SeparatorConfig>) {
       self.sep_joins = cfgs; 
    }

    fn find_sep_config(&self, sep : char) -> Option<&SeparatorConfig>
    {
        return self.sep_joins.iter().find(|i| i.sep == sep);
    }

    pub fn format_line(&self, l : &LineDescr) -> Option<String>
    {
        let fmt = self.fmt.as_ref().unwrap();
        if (self.non_matched_as_is && l.columns.len() != fmt.columns.len()) || l.columns.is_empty() {
            return Some(l.s.to_string());
        }
        
        let mut res = String::with_capacity(fmt.total_size + fmt.columns.len() * (self.join.len() + self.fill_count as usize));
        let fill_str = self.fill.to_string();
        let explicit_join = !self.join.is_empty();
        let mut skip_join = true;
        
        for (c,s) in l.columns.iter().enumerate(){
            if explicit_join && !skip_join {
                res.push_str(&self.join);
            }
            
            if skip_join && fmt.columns[c] > 0 {
                skip_join = false;
            }
            
            let subs : &str = s.col;
            let w = fmt.columns[c];
            let delta = w - subs.len() + self.fill_count as usize;
            
            res.push_str(&align_string(subs, &fill_str, delta, &self.align));

            if !explicit_join && s.sep != '\0' {
                if let Some(sep_cfg) = self.find_sep_config(s.sep) {
                    res.push_str(&sep_cfg.sep_str);
                }else{
                    res.push(s.sep);
                }
            }
        }

        Some(res)
    }

    pub fn parse_args(&mut self, mut arg_it :std::slice::Iter<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut sep_cfgs : Vec<SeparatorConfig> = vec![];
        loop 
        {
            if let Some(arg) = arg_it.next() {
               if arg == "--align" {
                   if let Some(align_str) = arg_it.next() {
                       if let Ok(al) = align_str.parse::<Align>() {
                           self.align = al;
                       }
                   }
               }else if arg == "--fill" {
                   if let Some(fill_str) = arg_it.next() {
                       self.fill = fill_str.chars().next().unwrap();
                   }
               }else if arg == "--fill_count" {
                   if let Some(fill_count_str) = arg_it.next() {
                       self.fill_count = fill_count_str.parse().unwrap_or(1);
                   }
               }else if arg == "--join" {
                   if let Some(join_str) = arg_it.next() {
                       self.join = join_str.clone();
                   }
               }else if arg == "--non_matched_as_is" {
                   self.non_matched_as_is = true;
               }else if arg == "--sep_config" {
                   if let Some(cfg_str) = arg_it.next() {
                       if let Ok(cfg) = cfg_str.parse::<SeparatorConfig>() {
                           sep_cfgs.push(cfg);
                       }
                   }
               }
            }else {
                break;
            }
        };
        self.set_separator_configs(sep_cfgs);
        Ok(())
    }
}