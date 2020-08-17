mod column_tools;
use std::error::Error;

fn parse_first_num(s :&str) -> (Option<i16>, &str)
{
    let mut numres : Option<i16> = None;
    let mut rest_str : &str = s;
    
    for (i,c) in s.chars().enumerate() {
        if c < '0' || c > '9' {
           if i > 0 {
               numres = Some(s[..i].parse::<i16>().unwrap());
               rest_str = &s[i..];
           } 
        }
    }
    
    (numres, rest_str)
}

fn main() -> Result<(), Box<dyn Error>> {
    
    use column_tools::LineDescr;
    use column_tools::Formatter;
    use column_tools::Printer;
    use column_tools::Align;
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
    let mut print_join : String = String::from(" ");
    let mut non_matched_as_is = false;

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
                   let lc_align_str = align_str.to_lowercase();
                   if lc_align_str == "left" {
                       align = Align::Left;
                   }else if lc_align_str == "center" {
                       align = Align::Center;
                   }else if lc_align_str == "right" {
                       align = Align::Right;
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
                  let res = parse_first_num(&bound_str);
                  if res.0.is_some() && res.1.len() > 0 {
                     let mut chrs = res.1.chars();
                     let open_c = chrs.next().unwrap(); 
                     let bt_opt : Option<column_tools::BoundType> = match arg.as_str() {
                        "--include" => Some(column_tools::BoundType::Include), 
                        "--exclude" => Some(column_tools::BoundType::Exclude), 
                        &_ => None
                     };
                     if let Some(bt) = bt_opt {
                         if let Some(close_c) = chrs.next() {
                            fmtr.add_boundary(open_c, close_c, res.0.unwrap(), bt); 
                         }else {
                            fmtr.add_boundary_symmetrical(open_c, res.0.unwrap(), bt); 
                         }
                     }
                  }
               }
           }else if arg == "--seps" {
               if let Some(seps) = arg_it.next() {
                   separators = seps.chars().collect();
               }
           }else if arg == "--non_matched_as_is" {
               non_matched_as_is = true;
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

    let printer = Printer::new(&fmtr, align, print_fill, print_fill_count, print_join, non_matched_as_is);
    for l in lines.iter()
    {
        if let Some(s) = printer.format_line(&l) {
            out.write(s.as_bytes())?;
        }
    }
    out.flush()?;
    Ok(())
}
