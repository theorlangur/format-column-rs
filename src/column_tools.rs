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
    s : &'a str,
    columns : Vec<Column<'a>>
}

impl<'a> LineDescr<'a>{
    pub fn new(s : &'a str) -> LineDescr<'a>
    {
        LineDescr{s, columns : Vec::new()}
    }
}

pub struct Boundary
{
    open : char,//char that 'opens' the block, also for simple separators
    close : Option<char>, //char that 'closes' the block
    lim : i16,
    lim_orig : i16
}

impl Boundary
{
    pub fn reset(&mut self)
    {
        self.lim = self.lim_orig;
    }
    
    pub fn check(&mut self, c : char) -> (bool, bool)
    {
        if c == self.open {
            if self.close.is_none() && self.lim != self.lim_orig {
                self.lim += 1;
            }else{
                self.lim -= 1;
            }
                
            (true, self.lim <= 0)
        }else if let Some(cl) = self.close {
            if cl == c {
                self.lim += 1;
                (true, self.lim <= 0)
            }else {
                (false, self.lim <= 0)
            }
        }else {
            (false, self.lim <= 0)
        }
    }
}

impl std::str::FromStr for Boundary {
    type Err = ParseErr;

    fn from_str(s :&str) -> Result<Self, Self::Err>
    {
        if let Some(last_num_idx) = s.find(|c|c < '0' || c > '9') {
           let lim_orig = s[..last_num_idx].parse::<i16>()?;
           let rest = &s[last_num_idx..];
           if rest.len() > 0 {
            let mut chrs = rest.chars();
            let open_c = chrs.next().unwrap(); 
            let res = Boundary{open : open_c, close : chrs.next(), lim_orig, lim : lim_orig};
            return Ok(res);
           }
        }
        
        Err(Self::Err{})
    }
}


pub enum BoundType
{
    Include,
    Exclude
}

impl std::str::FromStr for BoundType {
    type Err = ParseErr;

    fn from_str(s :&str) -> Result<Self, Self::Err>
    {
       match s {
           "include" => Ok(BoundType::Include),
           "exclude" => Ok(BoundType::Exclude),
           &_ => Err(Self::Err{}),
       }
    }
}

pub struct ColumnTracker
{
    seps : Vec<char>,
    seps_new_column : Vec<char>,
    include : Vec<Boundary>,
    exclude : Vec<Boundary>
}

impl ColumnTracker
{
    pub fn new() -> ColumnTracker
    {
        ColumnTracker{seps: Vec::new(), seps_new_column: Vec::new(), include : Vec::new(), exclude : Vec::new()}
    }

    pub fn reset(&mut self)
    {
        self.include.iter_mut().for_each(|x|x.reset());
        self.exclude.iter_mut().for_each(|x|x.reset());
    }

    fn check_bounds(&mut self, c : char) -> bool
    {
        let mut res = true;
        for s in self.include.iter_mut() {
            let (_, allowed) = s.check(c);
            if !allowed {
                res = false;
            }
        }

        for s in self.exclude.iter_mut() {
            let (_, ignore) = s.check(c);
            if ignore {
                res = false;
            }
        }

        res
    }

    pub fn is_column_end(&mut self, c : char) -> bool
    {
        let mut res = false;
        if let Result::Ok(_) = self.seps.binary_search(&c) {
            res = true;
        }

        if self.check_bounds(c) {
            res
        }else {
            false
        }
    }

    pub fn is_column_begin(&mut self, c : char) -> bool
    {
        self.check_bounds(c);
        let seps = if self.seps_new_column.is_empty() { &self.seps }else{ &self.seps_new_column };
        if let Result::Ok(_) = seps.binary_search(&c) {
            false //among separators? - no the column begin
        }else{
            true //some other symbol - yes, can be a column begin
        }
    }
}

pub struct Formatter
{
    columns : Vec<usize>,
    tracker : ColumnTracker,
    total_size : usize,
    line_starts_to_ignore : Vec<String>,
}

impl Formatter
{
    pub fn new()->Formatter
    {
        Formatter{columns:Vec::new(), tracker : ColumnTracker::new(), total_size: 0, line_starts_to_ignore : Vec::new()}
    }

    pub fn set_separators(&mut self, seps :&[char])
    {
       self.tracker.seps.extend_from_slice(seps); 
       self.tracker.seps.sort();
    }

    pub fn set_new_column_separators(&mut self, seps :&[char])
    {
       self.tracker.seps_new_column.extend_from_slice(seps); 
       self.tracker.seps_new_column.sort();
    }

    pub fn set_line_starts_to_ignore(&mut self, vals : Vec<String>) {
        self.line_starts_to_ignore = vals;
    }

    pub fn add_boundary(&mut self, bnd : Boundary, bt : BoundType)
    {
        match bt {
            BoundType::Include => &mut self.tracker.include,
            BoundType::Exclude => &mut self.tracker.exclude,
        }.push(bnd);
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

    fn add_column<'a>(&mut self, begin:usize, end:usize, ch : char, s : &'a str, cols :&mut Vec<Column<'a>>)
    {
        let cnt = end - begin;
        cols.push(Column{col : &s[begin..end], sep : ch});
        self.check_biggest_column(cols.len() - 1, cnt);
    }

    pub fn finish(&mut self)
    {
        self.total_size = self.columns.iter().sum();
        
    }

    pub fn analyze_line<'a>(&mut self, l: &mut LineDescr<'a>)
    {
        enum State{
            BeforeColumnBegin,
            InsideColumn
        }
        self.tracker.reset();
        let mut ignore = false;
        let mut first_non_white = true;

        let mut column_begin = 0;
        let mut past_column_end = 0;
        let mut s = State::BeforeColumnBegin;
        for (off,v) in l.s.char_indices(){
            if first_non_white && !v.is_ascii_whitespace() {
                first_non_white = false;
                let first = &(l.s[off..]);
                if self.line_starts_to_ignore.iter().any(|s|first.starts_with(s)) {
                    ignore = true;
                    break;
                }
            }
            s = match s {
                State::BeforeColumnBegin => if self.tracker.is_column_begin(v) {column_begin = off; State::InsideColumn} else {State::BeforeColumnBegin},
                State::InsideColumn => if self.tracker.is_column_end(v) {
                        self.add_column(column_begin, off, v, l.s, &mut l.columns);
                        past_column_end = off + 1;
                        State::BeforeColumnBegin
                    }else{
                        State::InsideColumn
                    }
            }
        }


        if !ignore {
            match s {
                State::InsideColumn => self.add_column(column_begin, l.s.len(), '\0', l.s, &mut l.columns),
                State::BeforeColumnBegin => if past_column_end < l.s.len() { self.add_column(past_column_end, l.s.len(), '\0', l.s, &mut l.columns); },
            }
        }
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

pub struct Printer<'a>
{
    fill : char,
    fill_count : u8,
    join : String,
    align : Align,
    fmt : &'a Formatter,
    non_matched_as_is : bool,//lines with not exactly amount of columns will be written as is
    sep_joins : Vec<SeparatorConfig>,
}

impl<'a> Printer<'a>{
    pub fn new(fmt : &'a Formatter, align:Align, fill : char, fill_count : u8, join : String, non_matched_as_is : bool) -> Printer
    {
        Printer{fill, align, fmt, fill_count, join, non_matched_as_is, sep_joins : Vec::new()}
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
        if (self.non_matched_as_is && l.columns.len() != self.fmt.columns.len()) || l.columns.is_empty() {
            return Some(l.s.to_string());
        }
        
        let mut res = String::with_capacity(self.fmt.total_size + self.fmt.columns.len() * (self.join.len() + self.fill_count as usize));
        let fill_str = self.fill.to_string();
        let explicit_join = !self.join.is_empty();
        
        for (c,s) in l.columns.iter().enumerate(){
            if explicit_join && c > 0 {
                res.push_str(&self.join);
            }
            
            let subs : &str = s.col;
            let w = self.fmt.columns[c];
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

        res.push('\n');
        Some(res)
    }
}