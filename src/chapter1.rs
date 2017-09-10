use chrono::prelude::NaiveDate;
use regex::Regex;


pub fn s2_3_1()
{
    let tests = vec![("FF",16,10,"255"), ("4b",16,10,"75"), ("4c",16,10,"76")];
    for x in tests
    {
        let (input, base_x, base_y, output) = x;
        println!("{:?}",x);
        assert!(s2_3_1_solution(input,base_x,base_y) == output);
    }
    println!("Okay!");
}

fn s2_3_1_solution(number : &str, base_x : u32, base_y : u32) -> String
{
    let mut num_as_u32 : u32 = u32::from_str_radix(number, base_x).unwrap();

    let mut output : String = String::from("");

    while num_as_u32 > 0
    {
            let dividend : u32 = num_as_u32 / base_y;
            let moduland : u32 = num_as_u32 % base_y;

            output.insert(
                0,
                ::std::char::from_digit(moduland,base_y).unwrap()
            );

            if dividend != 0
            {
                num_as_u32 = dividend;
            }
            else
            {
                break;
            }
    }

    output
}


pub fn s2_3_2()
{
    let tests = vec![([1,2,3],10,false), ([1,2,3],2,true), ([1,2,3],-1,false)];
    for x in tests
    {
        let (l,v,output) = x;
        println!("{:?}",x);
        assert!(s2_3_2_solution(&l,&v) == output);
    }
    println!("Okay!");
}

fn s2_3_2_solution(l : &[i32], v : &i32) -> bool
{
    l.contains(v)
}

pub fn s2_3_3()
{
    let tests = vec![("20170908","Friday"), ("20170906","Wednesday"), ("20170911","Monday")];
    for x in tests
    {
        let (d,output) = x;
        println!("{:?}",x);
        assert!(s2_3_3_solution(d) == output);
    }
    println!("Okay!");
}

fn s2_3_3_solution(date : &str) -> String
{
    NaiveDate::parse_from_str(date,"%Y%m%d").unwrap().format("%A").to_string()
}

pub fn s2_3_4()
{
    let tests = vec![("a70 and z72 will be replaced, but aa24 and a872 will not","*** and *** will be replaced, but aa24 and a872 will not")];
    for x in tests
    {
        let (input,output) = x;
        println!("{:?}",x);
        assert!(s2_3_4_solution(input) == output);
    }
    println!("Okay!");
}

fn s2_3_4_solution(input : &str) -> String
{
    let re = Regex::new(r"(?P<sb>\b)\w{1}\d{2}(?P<eb>\b)").unwrap();
    let output = re.replace_all(input, "$sb***$eb").to_string();
    output
}

pub fn s3_1()
{
    let tests = vec![
        (
            vec![(1,10),(100,200),(201,210),(900,1000)]
            , vec![(1,10,20),(100,200,125),(201,210,89),(900,1000,174)]
        )
        ,(
            vec![(1,10000)]
            , vec![(1,10000,262)]
        )
        ,(
            vec![(10000,1)]
            , vec![(10000,1,262)]
        )
    ];
    for x in tests
    {
        println!("{:?}",x);
        let (input,output) = x;
        let actual_output = s3_1_solution(&input);
        for (k,result) in actual_output.iter().enumerate()
        {
            let (i_actual,j_actual,max_actual) = *result;
            let (i,j,max) = output[k];

            assert!(i == i_actual);
            assert!(j == j_actual);
            assert!(max == max_actual);
        }
    }
    println!("Okay!");
}

fn s3_1_solution(input : & Vec<(u32,u32)> ) -> Vec<(u32,u32,u32)>
{
    let mut ret : Vec<(u32,u32,u32)> = vec![];

    let size = input.len();
    for z in 0..size
    {
        let (i,j) = input[z];
        let mut iter = i..j;
        if i > j
        {
            iter = j..i;
        }
        let mut max_cycle = 0;

        for n in iter
        {
            let curr_cycle = s3_1_cycle_length(n);
            if curr_cycle > max_cycle
            {
                max_cycle = curr_cycle;
            }
        }

        ret.push( (i,j,max_cycle) );
    }
    ret
}

fn s3_1_cycle_length(mut n : u32) -> u32
{
    let mut cnt : u32 = 1;
    while n > 1
    {
        if n % 2 == 0
        {
            n = n/2;
        }
        else
        {
            n = (3*n)+1;
        }
        cnt += 1;
    }
    cnt
}

pub fn s3_2()
{
    let tests = vec![
        (
            r#""To be or not to be," quoth the bard, "that is the question.""#
            , r#"``To be or not to be,'' quoth the bard, ``that is the question.''"#
        )
    ];
    for x in tests
    {
        println!("{:?}",x);
        let (input,output) = x;
        println!("{}",s3_2_solution(&input));
        assert!(s3_2_solution(&input) == output);

    }
    println!("Okay!");
}

fn s3_2_solution(text : &str) -> String
{
    let re = Regex::new(r#""(?P<t>[^"])""#).unwrap();
    println!("{}",r#""(?P<t>[^"]*)""#);
    re.replace_all(text,r#"``$t''"#).to_string()
}
