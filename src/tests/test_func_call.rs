#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer;
    use crate::tests::mytests::assert_eq;
    use crate::auto_config::*;

    #[test]
    fn test() {
        let mut cfg = do_auto_config(AutoMode::FnCall);

        //input
        let in_str = r##"
some_func(int, float);
Class::Method(char a, SomeStruct{23, 45.5, 22});//other comment
yet_another_func(int, int);//some comment"##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
some_func       (int   , float                   );               
Class::Method   (char a, SomeStruct{23, 45.5, 22});//other comment
yet_another_func(int   , int                     );//some comment "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq(&result, out_str);
    }
}