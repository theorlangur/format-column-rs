#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer as run_analyzer;
    use crate::auto_config::*;

    #[test]
    fn test_space() {
        let mut cfg = do_auto_config(AutoMode::SimpleSpace);

        //input
        let in_str = r##"
some weird data
in columns
try to format it correctly
    "##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
some weird   data  
in   columns
try  to      format it correctly
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }

    #[test]
    fn test_comma() {
        let mut cfg = do_auto_config(AutoMode::SimpleComma);

        //input
        let in_str = r##"
some weird, data
in, columns
try to, format it, correctly"##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
some weird, data     
in        , columns  
try to    , format it, correctly"##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
    
    #[test]
    fn test_struct() {
        let mut cfg = do_auto_config(AutoMode::CLike(Some('{'), Some('}')));

        //input
        let in_str = r##"
    {"SomeApi::Func1", &SomeApi::Func1, "{int b[, int : a]}", "result: boolean"},
   {"SomeOtherApi::CoolMethod", &SomeOtherApi::CoolMyMethod, "string : nu, int : bla[, {x : all}]", "nothing"},
  {"JustApi::Boring", &JustApi::Boring, "", "nothing"},
 {"OneMore::WhoNeedsThis", &OneMore::WhoNeedsThis, "nothing", ""},
"##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
    {"SomeApi::Func1"          , &SomeApi::Func1            , "{int b[, int : a]}"                 , "result: boolean"},
    {"SomeOtherApi::CoolMethod", &SomeOtherApi::CoolMyMethod, "string : nu, int : bla[, {x : all}]", "nothing"        },
    {"JustApi::Boring"         , &JustApi::Boring           , ""                                   , "nothing"        },
    {"OneMore::WhoNeedsThis"   , &OneMore::WhoNeedsThis     , "nothing"                            , ""               },"##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
}
