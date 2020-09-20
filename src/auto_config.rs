use crate::analyzers::assignment::TypeVarAnalyzer as AssignmentAnalyzer;
use crate::analyzers::assignment::VarAnalyzer as AssignmentVarAnalyzer;
use crate::analyzers::func_decl::Analyzer as FuncDeclAnalyzer;
use crate::analyzers::xml_attr::Analyzer as XmlAttrAnalyzer;
use crate::analyzers::var_decl::Analyzer as VarDeclAnalyzer;
use crate::analyzers::bit_field::Analyzer as BitFieldAnalyzer;
use crate::analyzers::cmnt_struct::Analyzer as CommentStructAnalyzer;
use crate::analyzers::separators::Analyzer as SepLineAnalyzer;

use crate::analyzers::LineAnalyzer;
use crate::analyzers::AnalyzeErr;
use crate::analyzers::separators::Boundary;
use crate::analyzers::separators::BoundType;

use crate::column_tools::Printer;
use crate::column_tools::Formatter;
use crate::column_tools::Align;
use crate::column_tools::SeparatorConfig;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum AutoMode {
    SimpleSpace, //space separated columns
    SimpleComma, //comma separated, space as a non-new column symbol
    SimpleAssignment,
    SimpleVarAssignment,
    FnDecl,
    VarDecl,
    Xml,
    BitField,
    CommentWithStruct, // /* xxxx */ {.....}
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

fn try_accept<T:LineAnalyzer>(la : T, s :&str)->Result<(),AnalyzeErr>{
    la.can_accept(s)
}

pub fn auto_analyze(s :& str) -> AutoMode {
    if let Ok(_) = try_accept(XmlAttrAnalyzer{}, s) {
        AutoMode::Xml
    }else if let Ok(_) = try_accept(BitFieldAnalyzer{}, s) {
       AutoMode::BitField 
    }else if let Ok(_) = try_accept(AssignmentAnalyzer{}, s) {
       AutoMode::SimpleAssignment 
    }else if let Ok(_) = try_accept(FuncDeclAnalyzer{}, s) {
        AutoMode::FnDecl
    }else if let Ok(_) = try_accept(AssignmentVarAnalyzer{}, s) {
       AutoMode::SimpleVarAssignment 
    }else if let Ok(_) = try_accept(CommentStructAnalyzer::new(), s) {
        AutoMode::CommentWithStruct 
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

pub struct AutoConfigResult
{
    pub printer : Printer,
    pub formatter : Formatter,
    pub analyzer : Box<dyn LineAnalyzer>
}

pub fn do_auto_config(m:AutoMode)->AutoConfigResult {

    let mut print_join = String::new();
    let analyzer : Box<dyn LineAnalyzer>;
    let mut non_matched_as_is = false;
    let mut sep_cfgs : Vec<SeparatorConfig> = vec![];
    let mut fmtr : Formatter = Formatter::new();
    let align = Align::Left;
    let print_fill = ' ';
    let print_fill_count = 0;

    fmtr.clear();
    fmtr.set_add_pre_start(true);

    match m {
            AutoMode::SimpleSpace => {
                let mut sa = SepLineAnalyzer::new();
                sa.clear();
                sa.set_separators(vec![' ']);
                analyzer = Box::new(sa);
            },
            AutoMode::SimpleComma => {
                let mut sa = SepLineAnalyzer::new();
                sa.set_separators(vec![',']);
                sa.set_new_column_separators(vec![',', ' ']);
                print_join = String::from(", ");
                analyzer = Box::new(sa);
            },
            AutoMode::SimpleAssignment => {
                non_matched_as_is = true;
                sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>().unwrap());
                analyzer = Box::new(AssignmentAnalyzer{});
            },
            AutoMode::SimpleVarAssignment => {
                non_matched_as_is = true;
                sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>().unwrap());
                analyzer = Box::new(AssignmentVarAnalyzer{});
            },
            AutoMode::FnDecl => {
                non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                analyzer = Box::new(FuncDeclAnalyzer{});
            },
            AutoMode::VarDecl => {
                non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                analyzer = Box::new(VarDeclAnalyzer{});
            },
            AutoMode::BitField => {
                //non_matched_as_is = true;
                sep_cfgs.push(SeparatorConfig::new(':', ' ', 2, Align::Center));
                analyzer = Box::new(BitFieldAnalyzer{});
            },
            AutoMode::Xml => {
                //non_matched_as_is = true;
                //sep_cfgs.push("=: :2:center".parse::<SeparatorConfig>()?);
                analyzer = Box::new(XmlAttrAnalyzer{});
            },
            AutoMode::CommentWithStruct => {
                fmtr.set_line_starts_to_ignore(vec!["//".to_string()]);
                sep_cfgs.push(",: :1".parse::<SeparatorConfig>().unwrap());
                non_matched_as_is = true;
                let mut a = CommentStructAnalyzer::new();
                a.clear();
                analyzer = Box::new(a);
            },
            AutoMode::CLike(open, close) => {
                let mut seps : Vec<char> = Vec::with_capacity(2);
                seps.push(',');
                fmtr.set_line_starts_to_ignore(vec!["//".to_string()]);
                let mut sa = SepLineAnalyzer::new();
                sa.set_new_column_separators(vec![',', ' ']);
                sa.add_boundary(Boundary::new_sym('"', 1), BoundType::Exclude);
                //fmtr.add_boundary(Boundary::new_asym('<', '>', 1), BoundType::Exclude);
                if let Some(o) = open {
                    if let Some(c) = close {
                        seps.push(c);
                        sa.add_boundary(Boundary::new_asym(o, c, 1), BoundType::Include);
                    }else{
                        sa.add_boundary(Boundary::new_sym(o, 1), BoundType::Include);
                    }
                }
                sa.set_separators(seps);
                sep_cfgs.push(",: :1".parse::<SeparatorConfig>().unwrap());
                non_matched_as_is = true;
                analyzer = Box::new(sa);
            },
    };

    let mut printer = Printer::new(align, print_fill, print_fill_count, print_join, non_matched_as_is);
    printer.set_separator_configs(sep_cfgs);

    AutoConfigResult{printer, formatter:fmtr, analyzer}
}
