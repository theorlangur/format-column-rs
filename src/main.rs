#[macro_use] 
extern crate lazy_static;

mod column_tools;
mod analyzers;
mod auto_config;
mod tests;

use std::error::Error;
    
use analyzers::LineAnalyzer;

use analyzers::assignment::TypeVarAnalyzer as AssignmentAnalyzer;
use analyzers::assignment::VarAnalyzer as AssignmentVarAnalyzer;
use analyzers::func_decl::Analyzer as FuncDeclAnalyzer;
use analyzers::xml_attr::Analyzer as XmlAttrAnalyzer;
use analyzers::var_decl::Analyzer as VarDeclAnalyzer;
use analyzers::bit_field::Analyzer as BitFieldAnalyzer;
use analyzers::cmnt_struct::Analyzer as CommentStructAnalyzer;
use analyzers::separators::Analyzer as SepLineAnalyzer;

use column_tools::LineDescr;
use column_tools::Printer;
use column_tools::write_lines_into;

use auto_config::auto_analyze;
use auto_config::do_auto_config;

use column_tools::Formatter;

type DynLineAnalyzer = Box<dyn LineAnalyzer>;
type ACreate = Box<dyn Fn()->DynLineAnalyzer>;
type AnalyzerFactory = std::collections::HashMap<&'static str, ACreate>;

    
fn read_input(src_file:Option<&String>)->(Vec<String>, Option<String>)
{
    let mut src : Box<dyn std::io::BufRead> = if src_file.is_some() {
            let f = std::fs::File::open(src_file.unwrap());
            if f.is_ok() {
                Box::new(std::io::BufReader::new(f.unwrap()))
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
        if src.read_line(&mut l).unwrap() <= 0 {
            break;
        } 
        if first_string.is_none() {
            first_string = Some(l.clone());
        }
        lines_str.push(l.trim_end().to_string());
    }
    
    (lines_str, first_string)
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let factory : AnalyzerFactory = {
        let mut factory : AnalyzerFactory = AnalyzerFactory::new();
        factory.insert("sep"           , Box::new(||Box::new(SepLineAnalyzer::new()      )));
        factory.insert("bit_field"     , Box::new(||Box::new(BitFieldAnalyzer{}          )));
        factory.insert("comment_struct", Box::new(||Box::new(CommentStructAnalyzer::new())));
        factory.insert("var_decl"      , Box::new(||Box::new(VarDeclAnalyzer{}           )));
        factory.insert("xml"           , Box::new(||Box::new(XmlAttrAnalyzer{}           )));
        factory.insert("func_decl"     , Box::new(||Box::new(FuncDeclAnalyzer{}          )));
        factory.insert("assign_var"    , Box::new(||Box::new(AssignmentVarAnalyzer{}     )));
        factory.insert("assign_init"   , Box::new(||Box::new(AssignmentAnalyzer{}        )));
        factory
    };

    let mut analyzer_type : String = String::from("sep");
    let args : Vec<String> = std::env::args().collect();

    let mut lines: Vec<LineDescr> = Vec::new();
    
    let mut out_file : Option<&String>  = None;
    let mut src_file : Option<&String>  = None;

    let mut auto_config = false;
    let mut type_only = false;

    let mut arg_it = args.iter();
    loop 
    {
        if let Some(arg) = arg_it.next() {
           if arg == "--file" {
               src_file = arg_it.next();
           }else if arg == "--out" {
               out_file = arg_it.next();
           }else if arg == "--analyzer" {
               if let Some(atype) = arg_it.next() {
                    let rstr : &str = &*atype;
                    if factory.contains_key(&rstr) {
                        analyzer_type = atype.clone();
                    }
               }
           }else if arg == "--auto" {
               auto_config = true;
           }else if arg == "--type" {
               type_only = true;
           } 
        }else {
            break;
        }
    };

    let (lines_str, first_string) = read_input(src_file);

    let mut line_analyzer : DynLineAnalyzer;
    let mut printer;
    let mut fmtr;


    if auto_config && first_string.is_some() {
        let fs = first_string.unwrap();
        let auto_config_res = do_auto_config(auto_analyze(&fs));
        line_analyzer = auto_config_res.analyzer;
        fmtr = auto_config_res.formatter;
        printer = auto_config_res.printer;
    }else
    {
        let entry = factory.get_key_value(analyzer_type.as_str()).unwrap();
        line_analyzer = entry.1();
        line_analyzer.parse_args(args.iter())?;

        printer = Printer::default();
        printer.parse_args(args.iter())?;
        
        fmtr = Formatter::new();
        fmtr.parse_args(args.iter())?;
    }

    if type_only {
        println!("{}", line_analyzer.type_name());
        return Ok(());
    }
    
    lines.reserve(lines_str.len());
    lines_str.iter().for_each(|l|{
       let mut line = LineDescr::new(&l);
       fmtr.analyze_line(line_analyzer.as_mut(), &mut line);
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
