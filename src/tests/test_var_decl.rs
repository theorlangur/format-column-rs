#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer;
    use crate::tests::mytests::assert_eq;
    use crate::auto_config::*;

    #[test]
    fn test_var_decl() {
        let mut cfg = do_auto_config(AutoMode::VarDecl);

        //input
        let in_str = r##"
    uint64_t somebit;
    int verylongbi; //and here's a comment
    char sho;
    double b;//some other comments
    SomeTemplate<bla> and_another;
    "##;
        
        //expected:
        let out_str = r##"
    uint64_t          somebit    ;
    int               verylongbi ;
    char              sho        ;
    double            b          ;
    SomeTemplate<bla> and_another;
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq(&result, out_str);
    }
}