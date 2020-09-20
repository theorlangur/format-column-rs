mod column_tools;
mod analyzers;
mod auto_config;
mod tests;

use std::error::Error;
    
use analyzers::LineAnalyzer;

use analyzers::separators::Analyzer as SepLineAnalyzer;

use column_tools::LineDescr;
use column_tools::Printer;
use column_tools::write_lines_into;

use auto_config::auto_analyze;
use auto_config::do_auto_config;


fn main() -> Result<(), Box<dyn Error>> {
    
    use column_tools::Formatter;
    use column_tools::Align;
    use column_tools::SeparatorConfig;
    
    use analyzers::separators::Boundary;
    use analyzers::separators::BoundType;

    let mut separator_analyzer = SepLineAnalyzer::new();
    
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
    let mut type_only = false;

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
           }else if arg == "--type" {
               type_only = true;
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
    let mut auto_config_res;
    let mut printer;


    if auto_config && first_string.is_some() {
        let fs = first_string.unwrap();
        auto_config_res = do_auto_config(auto_analyze(&fs));
        line_analyzer = auto_config_res.analyzer.as_mut();
        fmtr = auto_config_res.formatter;
        printer = auto_config_res.printer;
    }else
    {
        line_analyzer = &mut separator_analyzer;
        printer = Printer::new(align, print_fill, print_fill_count, print_join, non_matched_as_is);
        printer.set_separator_configs(sep_cfgs);
    }

    if type_only {
        println!("{}", line_analyzer.type_name());
        return Ok(());
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

    printer.set_formatter(fmtr);
    
    write_lines_into(&lines, &printer, out.as_mut())
}
