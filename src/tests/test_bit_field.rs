#[cfg(test)]
pub mod mytests {
    use crate::tests::mytests::run_analyzer as run_analyzer;
    use crate::*;

    #[test]
    fn test_bit_field() {
        let mut cfg = do_auto_config(AutoMode::BitField);

        //input
        let in_str = r##"
    uint64_t somebit : 1;
    uint64_t verylongbi : 4; //and here's a comment
    uint64_t sho : 15;
    uint64_t b : 2;//some other comments
    uint64_t and_another : 10;"##;
        
        //expected:
        let out_str = r##"
    uint64_t somebit     : 1; 
    uint64_t verylongbi  : 4;  //and here's a comment
    uint64_t sho         : 15;
    uint64_t b           : 2;  //some other comments 
    uint64_t and_another : 10;"##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
}