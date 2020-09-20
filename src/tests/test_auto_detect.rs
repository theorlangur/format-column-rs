#[cfg(test)]
mod mytests {
    use crate::auto_config::*;

    #[test]
    fn test() {
        //input
        let mode = auto_analyze(" int  some_func(int, float);");
        assert_eq!(mode, AutoMode::FnDecl);

        let mode = auto_analyze("MyClass x = 12;");
        assert_eq!(mode, AutoMode::SimpleAssignment);

        let mode = auto_analyze("SomwOtherl = SomwOtherl(x);");
        assert_eq!(mode, AutoMode::SimpleVarAssignment);

        let mode = auto_analyze("uint64_t verylongbi : 4; //and here's a comment");
        assert_eq!(mode, AutoMode::BitField);

        let mode = auto_analyze("SomeTemplate<bla> and_another;");
        assert_eq!(mode, AutoMode::VarDecl);

        let mode = auto_analyze(r##"<someothertag gggprop1="ddval1" some= "some name" and=" different"/>"##);
        assert_eq!(mode, AutoMode::Xml);

        let mode = auto_analyze(r##"/* empty */ {"OneMore::WhoNeedsThis", &OneMore::WhoNeedsThis, "nothing", ""},"##);
        assert_eq!(mode, AutoMode::CommentWithStruct);

        let mode = auto_analyze("try to, format it, correctly");
        assert_eq!(mode, AutoMode::SimpleComma);

        let mode = auto_analyze("try to format it correctly");
        assert_eq!(mode, AutoMode::SimpleSpace);

        let mode = auto_analyze(r##"{"SomeOtherApi::CoolMethod", &SomeOtherApi::CoolMyMethod, "string : nu, int : bla[, {x : all}]", "nothing"},"##);
        assert_eq!(mode, AutoMode::CLike(Some('{'), Some('}')));
    }
}