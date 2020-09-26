#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer;
    use crate::tests::mytests::assert_eq;
    use crate::auto_config::*;

    #[test]
    fn test_var_decl_assign() {
        let mut cfg = do_auto_config(AutoMode::SimpleAssignment);

        //input
        let in_str = r##"
   MyClass x = 12;
    int x = 155;
   SomwOtherl vay = SomwOtherl(x);
      const char * x = "xxxxx";
   std::span<int,3> s = 2;
    "##;
        
        //expected:
        let out_str = r##"
      MyClass          x   = 12;           
      int              x   = 155;          
      SomwOtherl       vay = SomwOtherl(x);
      const char *     x   = "xxxxx";      
      std::span<int,3> s   = 2;            
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq(&result, out_str);
    }

    #[test]
    fn test_var_assign() {
        let mut cfg = do_auto_config(AutoMode::SimpleVarAssignment);

        //input
        let in_str = r##"
MyClass = 12;
int = 155;
SomwOtherl = SomwOtherl(x);
char = "xxxxx";
s = 2;
    "##;
        
        //expected:
        let out_str = r##"
MyClass    = 12;           
int        = 155;          
SomwOtherl = SomwOtherl(x);
char       = "xxxxx";      
s          = 2;            
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq(&result, out_str);
    }
}