use std::{collections::HashMap, num::ParseFloatError};

#[derive(Clone)]
enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

#[derive(Debug)]
enum RispErr {
    Reason(String),
}

#[derive(Clone)]
struct RispEnv {
    data: HashMap<String, RispExp>,
}

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let mut res: Vec<RispExp> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(RispErr::Reason("could not find closing ')'".to_string()))?;

        if next_token == ")" {
            return Ok((RispExp::List(res), rest));
        }

        let (exp, next_xs) = parse(&xs)?;
        res.push(exp);
        xs = next_xs;
    }
}

fn parse_atom(token: &str) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();

    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token.to_string().clone()),
    }
}

fn default_env() -> RispEnv {
    let mut data: HashMap<String, RispExp> = HashMap::new();
    data.insert(
        "+".to_string(),
        RispExp::Func(|args| -> Result<RispExp, RispErr> {
            let sum = parse_list_of_float(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);

            Ok(RispExp::Number(sum))
        }),
    );
    data.insert(
        "-".to_string(),
        RispExp::Func(|args| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_float(args)?;
            let first = *floats
                .first()
                .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
            let sum = floats[1..].iter().fold(0.0, |sum, a| sum + a);
            Ok(RispExp::Number(first - sum))
        }),
    );
}

fn parse_list_of_float(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}

fn main() {
    println!("Hello, world!");
}
