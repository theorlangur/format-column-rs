#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer as run_analyzer;
    use crate::*;

    #[test]
    fn test() {
        let mut cfg = do_auto_config(AutoMode::CommentWithStruct);

        //input
        let in_str = r##"
/* some hint */    {"SomeApi::Func1", &SomeApi::Func1, "{int b[, int : a]}", "result: boolean"},
/* bla */   {"SomeOtherApi::CoolMethod", &SomeOtherApi::CoolMyMethod, "string : nu, int : bla[, {x : all}]", "nothing"},
/* some other longer */  {"JustApi::Boring", &JustApi::Boring, "", "nothing"},
/* empty */ {"OneMore::WhoNeedsThis", &OneMore::WhoNeedsThis, "nothing", ""},
    "##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
/* some hint         */ {"SomeApi::Func1"          , &SomeApi::Func1            , "{int b[, int : a]}"                 , "result: boolean"},
/* bla               */ {"SomeOtherApi::CoolMethod", &SomeOtherApi::CoolMyMethod, "string : nu, int : bla[, {x : all}]", "nothing"        },
/* some other longer */ {"JustApi::Boring"         , &JustApi::Boring           , ""                                   , "nothing"        },
/* empty             */ {"OneMore::WhoNeedsThis"   , &OneMore::WhoNeedsThis     , "nothing"                            , ""               },
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
}
