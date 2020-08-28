
struct TestRef<'a>
{
    s : String,
    refs : Vec<&'a str>
}

impl<'a> TestRef<'a>
{
    fn new(s : String) -> TestRef<'a>
    {
        TestRef{s, refs: vec![]}
    }
}

struct TestRef2<'a>
{
    arr: [u8; 3],
    refs : Vec<&'a [u8]>
}

impl<'a> TestRef2<'a>
{
    fn new(arr : [u8; 3]) -> TestRef2<'a>
    {
        TestRef2{arr, refs: vec![]}
    }
}

pub fn test_ref2()
{
    let mut tests : Vec<TestRef2> = vec![];
    
    loop  
    {
        let mut l :String = String::new();
        let r = std::io::stdin().read_line(&mut l);
        if r.unwrap() <= 0 {
            break;
        } 
        let base =l.parse::<u8>().unwrap();
        tests.push(TestRef2::new([base, base + 1, base + 2]));
    }
    
    let mut rest : &mut [TestRef2] = &mut tests;
    loop
    {
        if let Some((first, r)) = rest.split_first_mut() {
            let sptr = &(first.arr) as *const [u8; 3];
            unsafe{
                first.refs.push(&((*sptr)[1..2]));
                first.refs.push(&((*sptr)[0..1]));
            }
            rest = r; 
        }else
        {
            break;
        }
    }

    tests.push(TestRef2::new([1,2,3]));

    use std::io::Write;
    let mut dest = std::io::stdout();

    for item in tests.iter() {
        dest.write(&item.arr).unwrap();
    }
}

pub fn test_ref()
{
    let mut tests : Vec<TestRef> = vec![];
    
    loop  
    {
        let mut l :String = String::new();
        let r = std::io::stdin().read_line(&mut l);
        if r.unwrap() <= 0 {
            break;
        } 
        tests.push(TestRef::new(l));
    }
    
    let mut rest : &mut [TestRef] = &mut tests;
    loop
    {
        if let Some((first, r)) = rest.split_first_mut() {
            let sptr = &first.s as *const String;
            unsafe{
                first.refs.push(&(*sptr)[1..3]);
                first.refs.push(&(*sptr)[2..7]);
            }
            rest = r; 
        }else
        {
            break;
        }
    }

    tests.push(TestRef::new("l".to_string()));

    use std::io::Write;
    let mut dest = std::io::stdout();

    for item in tests.iter() {
        dest.write(item.s.as_bytes()).unwrap();
    }
}
