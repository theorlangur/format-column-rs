#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer as run_analyzer;
    use crate::auto_config::*;

    #[test]
    fn test() {
        let mut cfg = do_auto_config(AutoMode::FnDecl);

        //input
        let in_str = r##"
int  some_func(int, float);
char Class::Method(char a, SomeStruct B);

ComplexReturn Class2::AnotherMethod();
int yet_another_func(int, int, int, int, int);
    "##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
int           some_func            (int, float);             
char          Class::Method        (char a, SomeStruct B);   

ComplexReturn Class2::AnotherMethod();                       
int           yet_another_func     (int, int, int, int, int);
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
}