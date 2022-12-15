#![allow(dead_code)]

use nom::InputTakeAtPosition;
use nom::character::complete::{alphanumeric1, multispace0, digit1, multispace1};
use nom::combinator::{map_res, map};
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, combinator::value, sequence::preceded, branch::alt};
use nom::bytes::complete::tag;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CDPath<'a> {
    Root,
    Parent,
    Child(&'a str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LSOut<'a> {
    File(usize, &'a str),
    Dir(&'a str)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LS<'a> {
    pub out: Vec<LSOut<'a>>
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CD<'a> (pub CDPath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command<'a>{
    LS(LS<'a>),
    CD(CD<'a>)
}

fn non_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(char::is_whitespace)
}

pub fn parse_commands<'a>(s: &'a str) -> IResult<&str, Vec<Command<'a>>>{
    /*
    1. It begins with ""
    */
    let cd = preceded(tag("cd "), alt((
        value(Command::CD(CD(CDPath::Root)), tag("/")),
        value(Command::CD(CD(CDPath::Parent)), tag("..")),
        map_res(alphanumeric1, |x: &str| {
            let r: Result<Command, Error<&str>> = Result::Ok(Command::CD(CD(CDPath::Child(x))));
            r
        })
    )));

    let ls = map_res(preceded(tag("ls\n"), many0(
        alt((
            map(delimited(tag("dir "), non_space, multispace0), |x: &str| {LSOut::Dir(x)}),
            map(
                //LSOut::File(1, "la".to_string()),
                delimited(multispace0::<&str, Error<&str>>, tuple((digit1, multispace1, non_space)), multispace0),
                |res| {LSOut::File(res.0.parse::<usize>().unwrap(), res.2)}
            )
        ))
    )), |x| {
        let r = Command::LS(LS{out: x});
        let r: Result<Command, Error<&str>> = Result::Ok(r);
        r
    });

    let mut parser = many0(delimited(tag("$ "), alt((cd, ls)), multispace0));
    return parser(s)
}


#[cfg(test)]
mod tests {
    use super::parse_commands;
    use super::{Command, CD, CDPath, LS, LSOut};

    #[test]
    fn test_parse_cd_rt() {
        assert_eq!(parse_commands("$ cd /"), Ok(("", vec![Command::CD(CD(CDPath::Root))])));
    }
    #[test]
    fn test_parse_cd_par() {
        assert_eq!(parse_commands("$ cd .."), Ok(("", vec![Command::CD(CD(CDPath::Parent))])));
    }
    #[test]
    fn test_parse_cd_chl() {
        assert_eq!(parse_commands("$ cd abc"), Ok(("", vec![Command::CD(CD(CDPath::Child("abc")))])));
    }
    #[test]
    fn test_parse_ls_1dir() {
        assert_eq!(parse_commands("$ ls
dir ciao
1 la.txt"), Ok(("", vec![Command::LS(LS { out: vec![
            LSOut::Dir("ciao"),
            LSOut::File(1, "la.txt")
        ] })])));

        let r = parse_commands("$ ls
dir abc
123 asd
324443 per.txt
dir f");
        println!("Debug res parsed ls {:?}", r.unwrap());
    }

    #[test]
    fn test_input_aoc() {
        let t = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let r = parse_commands(t);
        println!("{:?}", r);
        assert!(r.is_ok(), "Didn't match a list of commands");
        let (rem, ur) = r.unwrap();
        println!("{:?}, {}", ur, ur.len());
        assert_eq!(ur.len(),10);
        assert_eq!(rem,"");
    }
}