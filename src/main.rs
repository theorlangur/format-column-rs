mod column_tools;
mod analyzers;
//mod test_ref;

use std::error::Error;
    
use analyzers::LineAnalyzer;

use analyzers::separators::Analyzer as SepLineAnalyzer;
use analyzers::assignment::Analyzer as AssignmentAnalyzer;
use analyzers::func_decl::Analyzer as FuncDeclAnalyzer;
use analyzers::xml_attr::Analyzer as XmlAttrAnalyzer;
use analyzers::var_decl::Analyzer as VarDeclAnalyzer;
use analyzers::bit_field::Analyzer as BitFieldAnalyzer;

enum AutoMode {
    SimpleSpace, //space separated columns
    SimpleComma, //comma separated, space as a non-new column symbol
    SimpleAssignment,
    FnDecl,
    VarDecl,
    Xml,
    BitField,
    CLike(Option<char>, Option<char>)        //ignores "", '', ignores lines starting with //, depending on what comes first {} or () tries to format inside there
}

fn auto_analyze_cpp(s :& str) -> Option<AutoMode> {
    let o;
    let c;

    if let Some(nonwhite) = s.find(|c:char|!c.is_ascii_whitespace()) {
        match s[nonwhite..].chars().next().unwrap() {
            '{' => {o = Some('{'); c = Some('}')},
            '(' => {o = Some('('); c = Some(')')},
            _ => {
               return None;
            },
        }
    }else
    {
        return None;
    }

    Some(AutoMode::CLike(o, c))
}

fn try_accept<T:LineAnalyzer>(la : T, s :&str)->Result<(),analyzers::AnalyzeErr>{
    la.can_accept(s)
}

fn auto_analyze(s :& str) -> AutoMode {
    if let Ok(_) = try_accept(XmlAttrAnalyzer{}, s) {
        AutoMode::Xml
    }else if let Ok(_) = try_accept(BitFieldAnalyzer{}, s) {
       AutoMode::BitField 
    }else if let Ok(_) = try_accept(AssignmentAnalyzer{}, s) {
       AutoMode::SimpleAssignment 
    }else if let Ok(_) = try_accept(FuncDeclAnalyzer{}, s) {
        AutoMode::FnDecl
    }else if let Ok(_) = try_accept(VarDeclAnalyzer{}, s) {
        AutoMode::VarDecl
    }else if let Some(mode) = auto_analyze_cpp(s) {
        mode
    }else if let Some(_) = s.find(',') {
        AutoMode::SimpleComma
    }else {
        AutoMode::SimpleSpace
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    
    use column_tools::LineDescr;
    use column_tools::Formatter;
    use column_tools::Printer;
    use column_tools::Align;
    use column_tools::SeparatorConfig;
    
    use analyzers::separators::Boundary;
    use analyzers::separators::BoundType;

    use std::io::Write;

    //test_ref::test_ref();
    //test_ref::test_ref2();

    let mut separator_analyzer = SepLineAnalyzer::new();
    let mut assignment_analyzer = AssignmentAnalyzer{};
    let mut func_decl_analyzer = FuncDeclAnalyzer{};
    let mut xml_attr_analyzer = XmlAttrAnalyzer{};
    let mut var_decl_analyzer = VarDeclAnalyzer{};
    let mut bit_field_analyzer = BitFieldAnalyzer{};
    
    let args : Vec<String> = std::env::args().collect();

    let mut separators : Vec<char> = vec![' '];
    let mut new_column_separators : Vec<char> = vec![];
    let mut lines: Vec<LineDescr> = Vec::new();
    let mut line_starts_to_ignore : Vec<String> = vec![];
    let mut fmtr = Formatter::new();
    
    let mut out_file : Option<&String>  = None;
    let mut src_file : Option<&String>  = None;

    let mut align = Align::Left;
    let mut print_fill : char = ' ';
    let mut print_fill_count : u8 = 0;
    let mut print_join : String = String::new();
    let mut non_matched_as_is = false;
    let mut auto_config = false;
    let mut add_pre_start = false;

    let mut sep_cfgs : Vec<SeparatorConfig> = vec![];

    let mut arg_it = args.iter();
    loop 
    {
        if let Some(arg) = arg_it.next() {
           if arg == "--file" {
               src_file = arg_it.next();
           }else if arg == "--out" {
               out_file = arg_it.next();
           }else if arg == "--align" {
               if let Some(align_str) = arg_it.next() {
                   if let Ok(al) = align_str.parse::<Align>() {
                       align = al;
                   }
               }
           }else if arg == "--fill" {
               if let Some(fill_str) = arg_it.next() {
                   print_fill = fill_str.chars().next().unwrap();
               }
           }else if arg == "--fill_count" {
               if let Some(fill_count_str) = arg_it.next() {
                   print_fill_count = fill_count_str.parse().unwrap_or(1);
               }
           }else if arg == "--join" {
               if let Some(join_str) = arg_it.next() {
                   print_join = join_str.clone();
               }
           }else if arg == "--include" || arg == "--exclude" {
               if let Some(bound_str) = arg_it.next() {
                   if let Ok(bnd) = bound_str.parse::<Boundary>() {
                        let bt = (&arg[2..]).parse::<BoundType>()?;
                        separator_analyzer.add_boundary(bnd, bt); 
                   }
               }
           }else if arg == "--seps" {
               if let Some(seps) = arg_it.next() {
                   separators = seps.chars().collect();
               }
           }else if arg == "--line_start_to_ignore" {
               if let Some(ignore) = arg_it.next() {
                    line_starts_to_ignore.push(ignore.clone());
               }
           }else if arg == "--non_matched_as_is" {
               non_matched_as_is = true;
           }else if arg == "--auto" {
               auto_config = true;
           }else if arg == "--prestart" {
               add_pre_start = true;
           }else if arg == "--sep_config" {
               if let Some(cfg_str) = arg_it.next() {
                   if let Ok(cfg) = cfg_str.parse::<SeparatorConfig>() {
                       sep_cfgs.push(cfg);
                   }
               }
           } else if arg == "--new_column_seps" {
               if let Some(seps) = arg_it.next() {
                   new_column_separators = seps.chars().collect();
               }
           } 
        }else {
            break;
        }
    };

    separator_analyzer.set_separators(separators);
    separator_analyzer.set_new_column_separators(new_column_separators);
    fmtr.set_line_starts_to_ignore(line_starts_to_ignore);
    fmtr.set_add_pre_start(add_pre_start);

    let mut src : Box<dyn std::io::BufRead> = if src_file.is_some() {
            let f = std::fs::File::open(src_file.unwrap());
            if f.is_ok() {
                Box::new(std::io::BufReader::new(f?))
            }else{
                Box::new(std::io::BufReader::new(std::io::stdin()))
            }
        }else{
            Box::new(std::io::BufReader::new(std::io::stdin()))
        };

    let mut first_string : Option<String> = None;
    let mut lines_str : Vec<String> = vec![];
    loop  
    {
        let mut l :String = String::new();
        if src.read_line(&mut l)? <= 0 {
            break;
        } 
        if first_string.is_none() {
            first_string = Some(l.clone());
        }
        lines_str.push(l.trim_end().to_string());
    }

    let line_analyzer : &mut dyn LineAnalyzer;


    if auto_config && first_string.is_some() {
        separator_analyzer.clear();
        fmtr.clear();
        sep_cfgs.clear();
        align = Align::Left;
        non_matched_as_is = false;
        print_fill = ' ';
        print_fill_count = 0;
        print_join = String::new();
        fmtr.set_add_pre_start(true);
        let fs = first_string.unwrap();

        match auto_analyze(&fs) {
            AutoMode::SimpleSpace => {
                separator_analyzer.set_separators(vec![' ']);
                line_analyzer = &mut separator_analyzer;
            },
            AutoMode::SimpleComma => {
                separator_analyzer.set_separators(vec![',']);
                separator_analyzer.set_new_column_separators(vec![',', ' ']);
                print_join = String::from(", ");
                line_analyzer = &mut separator_analyzer;
            },
            AutoMode::SimpleAssignment => {
                non_matched_as_is = true;
                sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                line_analyzer = &mut assignment_analyzer;
            },
            AutoMode::FnDecl => {
                non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                line_analyzer = &mut func_decl_analyzer;
            },
            AutoMode::VarDecl => {
                non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                line_analyzer = &mut var_decl_analyzer;
            },
            AutoMode::BitField => {
                //non_matched_as_is = true;
                sep_cfgs.push(SeparatorConfig::new(':', ' ', 2, Align::Center));
                line_analyzer = &mut bit_field_analyzer;
            },
            AutoMode::Xml => {
                non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                line_analyzer = &mut xml_attr_analyzer;
            },
            AutoMode::CLike(open, close) => {
                let mut seps : Vec<char> = Vec::with_capacity(2);
                seps.push(',');
                fmtr.set_line_starts_to_ignore(vec!["//".to_string()]);
                separator_analyzer.set_new_column_separators(vec![',', ' ']);
                separator_analyzer.add_boundary(Boundary::new_sym('"', 1), BoundType::Exclude);
                //fmtr.add_boundary(Boundary::new_asym('<', '>', 1), BoundType::Exclude);
                if let Some(o) = open {
                    if let Some(c) = close {
                        seps.push(c);
                        separator_analyzer.add_boundary(Boundary::new_asym(o, c, 1), BoundType::Include);
                    }else{
                        separator_analyzer.add_boundary(Boundary::new_sym(o, 1), BoundType::Include);
                    }
                }
                separator_analyzer.set_separators(seps);
                sep_cfgs.push(",: :1".parse::<SeparatorConfig>()?);
                non_matched_as_is = true;
                line_analyzer = &mut separator_analyzer;
            },
        }
    }else
    {
        line_analyzer = &mut separator_analyzer;
    }
    
    lines.reserve(lines_str.len());
    lines_str.iter().for_each(|l|{
       let mut line = LineDescr::new(&l);
       fmtr.analyze_line(line_analyzer, &mut line);
       lines.push(line); 
    });
    
    fmtr.finish();
    
    let mut out : Box<dyn std::io::Write> = if out_file.is_some() {
            let f = std::fs::File::create(out_file.unwrap());
            if f.is_ok() {
                Box::new(f?)
            }else{
                Box::new(std::io::stdout())
            }
        }else{
                Box::new(std::io::stdout())
        };

    let mut printer = Printer::new(&fmtr, align, print_fill, print_fill_count, print_join, non_matched_as_is);
    printer.set_separator_configs(sep_cfgs);
    
    for l in lines.iter()
    {
        if let Some(s) = printer.format_line(&l) {
            out.write(s.as_bytes())?;
        }
    }
    out.flush()?;
    Ok(())
}
