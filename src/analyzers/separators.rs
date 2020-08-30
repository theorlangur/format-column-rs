use crate::column_tools::*;
use super::LineAnalyzer;

pub struct Boundary
{
    open : char,//char that 'opens' the block, also for simple separators
    close : Option<char>, //char that 'closes' the block
    lim : i16,
    lim_orig : i16
}

impl Boundary
{
    pub fn new_sym(c :char, lim : i16) -> Boundary {
        Boundary{open : c, close : None, lim, lim_orig : lim}
    }

    pub fn new_asym(o :char, c :char, lim : i16) -> Boundary {
        Boundary{open : o, close : Some(c), lim, lim_orig : lim}
    }

    pub fn reset(&mut self)
    {
        self.lim = self.lim_orig;
    }
    
    pub fn check(&mut self, c : char) -> (bool, bool)
    {
        let mut begin_or_end = false;
        let mut in_bound = self.lim <= 0;

        if c == self.open {
            begin_or_end = true;
            if self.close.is_none() && self.lim != self.lim_orig {
                self.lim += 1;
            }else{
                self.lim -= 1;
                in_bound = self.lim <= 0;
            }
        }else if let Some(cl) = self.close {
            if cl == c {
                self.lim += 1;
                begin_or_end = true;
            }
        }
        (begin_or_end, in_bound)
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

pub struct SepLineAnalyzer
{
    seps : Vec<char>,
    seps_new_column : Vec<char>,
    include : Vec<Boundary>,
    exclude : Vec<Boundary>
}

impl SepLineAnalyzer {
    pub fn new() -> SepLineAnalyzer
    {
        SepLineAnalyzer{seps: Vec::new(), seps_new_column: Vec::new(), include : Vec::new(), exclude : Vec::new()}
    }
    
    pub fn reset(&mut self)
    {
        self.include.iter_mut().for_each(|x|x.reset());
        self.exclude.iter_mut().for_each(|x|x.reset());
    }
    
    pub fn set_separators(&mut self, seps :Vec<char>)
    {
       self.seps = seps; 
       self.seps.sort();
    }

    pub fn set_new_column_separators(&mut self, seps :Vec<char>)
    {
       self.seps_new_column = seps; 
       self.seps_new_column.sort();
    }

    fn boundary(&mut self, bt : BoundType) -> &mut Vec<Boundary>{
        match bt {
            BoundType::Include => &mut self.include,
            BoundType::Exclude => &mut self.exclude,
        }
    }

    pub fn clear_boundaries(&mut self, bt : BoundType)
    {
        self.boundary(bt).clear();
    }

    pub fn add_boundary(&mut self, bnd : Boundary, bt : BoundType)
    {
        self.boundary(bt).push(bnd);
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

    fn is_column_end(&mut self, c : char) -> bool
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

    fn is_column_begin(&mut self, c : char) -> bool
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

impl LineAnalyzer for SepLineAnalyzer
{
    fn clear(&mut self)
    {
        self.set_separators(vec![]);
        self.set_new_column_separators(vec![]);
        self.clear_boundaries(BoundType::Include);
        self.clear_boundaries(BoundType::Exclude);
    }

    
    fn analyze_line<'a>(&mut self, fmt :&mut Formatter, l: &mut LineDescr<'a>)
    {
        enum State{
            BeforeColumnBegin,
            InsideColumn
        }
        self.reset();
        let mut ignore = false;
        let mut first_non_white = true;

        let mut column_begin = 0;
        let mut past_column_end = 0;
        let mut s = State::BeforeColumnBegin;
        for (off,v) in l.s.char_indices(){
            if first_non_white && !v.is_ascii_whitespace() {
                first_non_white = false;
                let first = &(l.s[off..]);
                if fmt.check_line_start_to_ignore(first) {
                    ignore = true;
                    break;
                }
            }
            s = match s {
                State::BeforeColumnBegin => if self.is_column_begin(v) {
                    column_begin = off; 
                    if fmt.keep_indent() && !l.any_columns() {
                        fmt.add_column(0, off, '\0', l);
                    }
                    State::InsideColumn
                } else {State::BeforeColumnBegin},
                State::InsideColumn => if self.is_column_end(v) {
                        fmt.add_column(column_begin, off, v, l);
                        past_column_end = off + 1;
                        State::BeforeColumnBegin
                    }else{
                        State::InsideColumn
                    }
            }
        }


        if !ignore {
            match s {
                State::InsideColumn => fmt.add_column(column_begin, l.s.len(), '\0', l),
                State::BeforeColumnBegin => if past_column_end < l.s.len() { fmt.add_column(past_column_end, l.s.len(), '\0', l); },
            }
        }
    }
}