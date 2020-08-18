mod column_tools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    use column_tools::LineDescr;
    use column_tools::Formatter;
    use column_tools::Printer;
    use column_tools::Align;
    use column_tools::Boundary;
    use column_tools::SeparatorConfig;

    use std::io::Write;
    
    let args : Vec<String> = std::env::args().collect();

    let mut separators : Vec<char> = vec![' '];
    let mut new_column_separators : Vec<char> = vec![];
    let mut lines: Vec<LineDescr> = Vec::new();
    let mut fmtr = Formatter::new();
    
    let mut out_file : Option<&String>  = None;
    let mut src_file : Option<&String>  = None;

    let mut align = Align::Left;
    let mut print_fill : char = ' ';
    let mut print_fill_count : u8 = 0;
    let mut print_join : String = String::new();
    let mut non_matched_as_is = false;

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
                        let bt = (&arg[2..]).parse::<column_tools::BoundType>()?;
                        fmtr.add_boundary(bnd, bt); 
                   }
               }
           }else if arg == "--seps" {
               if let Some(seps) = arg_it.next() {
                   separators = seps.chars().collect();
               }
           }else if arg == "--non_matched_as_is" {
               non_matched_as_is = true;
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

    fmtr.set_separators(&separators);
    fmtr.set_new_column_separators(&new_column_separators);

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
    
    let mut lines_str : Vec<String> = vec![];
    loop  
    {
        let mut l :String = String::new();
        if src.read_line(&mut l)? <= 0 {
            break;
        } 
        lines_str.push(l.trim_end().to_string());
    }
    
    lines.reserve(lines_str.len());
    lines_str.iter().for_each(|l|{
       let mut line = LineDescr::new(&l);
       fmtr.analyze_line(&mut line);
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
