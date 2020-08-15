struct LineSlice
{
    beg : usize,
    end : usize
}

pub struct LineDescr
{
    pub s : String,
    columns : Vec<LineSlice>
}

impl LineDescr{
    pub fn new() -> LineDescr
    {
        LineDescr{s : String::new(), columns : Vec::new()}
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
            self.lim = self.lim - 1;
            (true, self.lim <= 0)
        }else if let Some(cl) = self.close {
            if cl == c {
                self.lim = self.lim + 1;
                (true, self.lim <= 0)
            }else {
                (false, self.lim <= 0)
            }
        }else {
            (false, self.lim <= 0)
        }
    }
}

pub struct ColumnTracker
{
    seps : Vec<char>,
    seps_new_column : Vec<char>,
    bounds : Vec<Boundary>
}

impl ColumnTracker
{
    pub fn new() -> ColumnTracker
    {
        ColumnTracker{seps: Vec::new(), seps_new_column: Vec::new(), bounds : Vec::new()}
    }

    pub fn reset(&mut self)
    {
        self.bounds.iter_mut().for_each(|x|x.reset());
    }

    pub fn is_column_end(&mut self, c : char) -> bool
    {
        let mut res = false;
        if let Result::Ok(_) = self.seps.binary_search(&c) {
            res = true;
        }

        for s in self.bounds.iter_mut() {
            let (_, allowed) = s.check(c);
            if !allowed {
                res = false;
            }
        }
        res
    }

    pub fn is_column_begin(&self, c : char) -> bool
    {
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

    pub fn add_boundary_symmetrical(&mut self, b_sym : char, lim : i16)
    {
        self.tracker.bounds.push(Boundary{open : b_sym, close : None, lim, lim_orig: lim});
    }

    pub fn add_boundary(&mut self, b_sym_open : char, b_sym_close : char, lim : i16)
    {
        self.tracker.bounds.push(Boundary{open : b_sym_open, close : Some(b_sym_close), lim, lim_orig: lim});
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

    fn add_column(&mut self, begin:usize, end:usize, cols :&mut Vec<LineSlice>)
    {
        let cnt = end - begin;
        cols.push(LineSlice{beg : begin, end});
        self.check_biggest_column(cols.len() - 1, cnt);
    }

    pub fn finish(&mut self)
    {
        self.total_size = self.columns.iter().sum();
        
    }

    pub fn analyze_line(&mut self, l :&mut LineDescr)
    {
        enum State{
            BeforeColumnBegin,
            InsideColumn
        }
        self.tracker.reset();
        let mut column_begin = 0;
        let mut s = State::BeforeColumnBegin;
        for (off,v) in l.s.chars().enumerate(){
            s = match s {
                State::BeforeColumnBegin => if self.tracker.is_column_begin(v) {column_begin = off; State::InsideColumn} else {State::BeforeColumnBegin},
                State::InsideColumn => if self.tracker.is_column_end(v) {
                        self.add_column(column_begin, off, &mut l.columns);
                        State::BeforeColumnBegin
                    }else{
                        State::InsideColumn
                    }
            }
        }

        match s {
            State::InsideColumn => self.add_column(column_begin, l.s.len(), &mut l.columns),
            _ => ()
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
    fmt : &'a Formatter
}

impl<'a> Printer<'a>{
    pub fn new(fmt : &'a Formatter, align:Align, fill : char, fill_count : u8, join : String) -> Printer
    {
        Printer{fill, align, fmt, fill_count, join}
    }

    pub fn format_line(&self, l : &LineDescr) -> String
    {
        let mut res = String::with_capacity(self.fmt.total_size + self.fmt.columns.len() * (self.join.len() + self.fill_count as usize));
        let fill_str = self.fill.to_string();
        
        for (c,sub_range) in l.columns.iter().enumerate(){
            if c > 0 {
                res.push_str(&self.join);
            }
            
            let w = self.fmt.columns[c];
            let subs = &l.s[sub_range.beg..sub_range.end];
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
        }

        res.push('\n');
        res
    }
}