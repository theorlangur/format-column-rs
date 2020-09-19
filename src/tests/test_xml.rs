#[cfg(test)]
mod mytests {
    use crate::tests::mytests::run_analyzer as run_analyzer;
    use crate::*;

    #[test]
    fn test() {
        let mut cfg = do_auto_config(AutoMode::Xml);

        //input
        let in_str = r##"
<sometag prop1="val1" someotherprop = "some other name" and_some="thing completely different">
</sometag>
<someothertag gggprop1="ddval1" some= "some name" and=" different"/>
<someortag gddddggprop1="ddval1" some= "dd some name" and=" different"/>
<sosdmeothertag gggprop1="ddvaeekdljkwl1" some= "skjldksome name" andeee=" different"/>
    "##;
        
        //expected: (currently there are spaces at the end of each line except the longest one)
        let out_str = r##"
<sometag        prop1       ="val1"           someotherprop="some other name" and_some="thing completely different" > 
</sometag>
<someothertag   gggprop1    ="ddval1"         some         ="some name"       and     =" different"                 />
<someortag      gddddggprop1="ddval1"         some         ="dd some name"    and     =" different"                 />
<sosdmeothertag gggprop1    ="ddvaeekdljkwl1" some         ="skjldksome name" andeee  =" different"                 />
    "##;

        let result = run_analyzer(in_str, cfg.analyzer.as_mut(), cfg.formatter, cfg.printer);

        assert_eq!(result, out_str);
    }
}