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

#[derive(Clone)]
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

pub enum BoundType
{
    Include,
    Exclude
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
    total_size : usize
}

impl Formatter
{
    pub fn new()->Formatter
    {
        Formatter{columns:Vec::new(), tracker : ColumnTracker::new(), total_size: 0}
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

    pub fn add_boundary_symmetrical(&mut self, b_sym : char, lim : i16, bt : BoundType)
    {
        let bounds = match bt {
            BoundType::Include => &mut self.tracker.include,
            BoundType::Exclude => &mut self.tracker.exclude,
        };
        bounds.push(Boundary{open : b_sym, close : None, lim, lim_orig: lim});
    }

    pub fn add_boundary(&mut self, b_sym_open : char, b_sym_close : char, lim : i16, bt : BoundType)
    {
        let bounds = match bt {
            BoundType::Include => &mut self.tracker.include,
            BoundType::Exclude => &mut self.tracker.exclude,
        };
        bounds.push(Boundary{open : b_sym_open, close : Some(b_sym_close), lim, lim_orig: lim});
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
        let mut column_begin = 0;
        let mut s = State::BeforeColumnBegin;
        for (off,v) in l.s.char_indices(){
            s = match s {
                State::BeforeColumnBegin => if self.tracker.is_column_begin(v) {column_begin = off; State::InsideColumn} else {State::BeforeColumnBegin},
                State::InsideColumn => if self.tracker.is_column_end(v) {
                        self.add_column(column_begin, off, v, l.s, &mut l.columns);
                        State::BeforeColumnBegin
                    }else{
                        State::InsideColumn
                    }
            }
        }

        match s {
            State::InsideColumn => self.add_column(column_begin, l.s.len(), '\0', l.s, &mut l.columns),
            _ => ()
        }
    }
}

pub enum Align {
    Left,
    Center,
    Right
}

pub struct Printer<'a>
{
    fill : char,
    fill_count : u8,
    join : String,
    align : Align,
    fmt : &'a Formatter,
    non_matched_as_is : bool,//lines with not exactly amount of columns will be written as is
}

impl<'a> Printer<'a>{
    pub fn new(fmt : &'a Formatter, align:Align, fill : char, fill_count : u8, join : String, non_matched_as_is : bool) -> Printer
    {
        Printer{fill, align, fmt, fill_count, join, non_matched_as_is}
    }

    pub fn format_line(&self, l : &LineDescr) -> Option<String>
    {
        if self.non_matched_as_is && l.columns.len() != self.fmt.columns.len() {
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
            
            if let Align::Right = self.align {
                res.push_str(&fill_str.repeat(delta));
            }else if let Align::Center = self.align {
                res.push_str(&fill_str.repeat(delta / 2));
            }
            
            res.push_str(&subs);
            
            if let Align::Left = self.align {
                res.push_str(&fill_str.repeat(delta));
            }else if let Align::Center = self.align {
                res.push_str(&fill_str.repeat(delta / 2 + delta % 2));
            }

            if !explicit_join && s.sep != '\0' {
                res.push(s.sep);
            }
        }

        res.push('\n');
        Some(res)
    }
}