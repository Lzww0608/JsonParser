use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::{separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;


#[derive(Debug)] 
enum JsonValue {
    Null,
    Num(f64),
    Bool(bool),
    Str(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn parse_null(s: &str) -> IResult<&str, JsonValue> {
    map(tag("null"), |_| JsonValue::Null).parse(s)
}

fn parse_bool(s: &str) -> IResult<&str, JsonValue> {
    alt((
        map(tag("true"), |_| JsonValue::Bool(true)),
        map(tag("false"), |_| JsonValue::Bool(false)),
    )).parse(s)
}

fn parse_num(s: &str) -> IResult<&str, JsonValue> {
    map(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        JsonValue::Num(s.parse().unwrap())
    }).parse(s)
}

fn parse_str(s: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('"'),
            take_while(|c| c != '"'),
            char('"'),
        ),
        |s: &str| JsonValue::Str(s.to_string()),
    ).parse(s)
}

fn parse_array(s: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('['),
            separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, parse_value),
            ),
            char(']'),
        ),
        |vec| JsonValue::Array(vec),
    ).parse(s)
}

fn parse_value(s: &str) -> IResult<&str, JsonValue> {
    preceded(
        multispace0,
        alt((
            parse_str,
            parse_num,
            parse_bool,
            parse_null,
            parse_array,
            parse_object,
        )),
    ).parse(s)
}

fn parse_pair(s: &str) -> IResult<&str, (JsonValue, JsonValue)> {
    separated_pair(
        preceded(multispace0, parse_str),
        preceded(multispace0, char(':')),
        preceded(multispace0, parse_value),
    ).parse(s)
}

fn parse_object(s: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('{'),
            separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, parse_pair),
            ),
            preceded(multispace0, char('}')),
        ),
        |pairs| JsonValue::Object(pairs.into_iter().map(|(k, v)| {
            if let JsonValue::Str(k) = k {
                return (k, v)
            }
            panic!("key must be a string")
        }).collect()),
    ).parse(s)
}

fn parse_json(s: &str) -> IResult<&str, JsonValue> {
    terminated(parse_value, multispace0).parse(s)
}

fn main() {

    println!("{:?}", parse_null("null"));
    println!("{:?}", parse_bool("true"));
    println!("{:?}", parse_num("123"));
    println!("{:?}", parse_str("hello"));
    println!("{:?}", parse_array("[1,2,3]"));
    println!("{:?}", parse_object("{\"name\": \"John\", \"age\": 30}"));
    let json_str = r##"
        {
            "nickname": "张三",
            "age": 30,
            "is_teacher": false,
            "scores": [90, 85, 95],
            "address": {
                "city": "北京",
                "street": "中关村大街",
                "code": [200, 2000]
            }
        }
    "##;

    match parse_json(json_str) {
        Ok((_, json)) => println!("{:#?}", json),
        Err(e) => println!("Error: {:?}", e),
    }
}
