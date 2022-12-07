use nom::character::complete::{alphanumeric1, multispace0, digit1, multispace1};
use nom::combinator::{map_res, map};
use nom::error::{Error};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, combinator::value, sequence::preceded, branch::alt};
use nom::bytes::complete::tag;

#[derive(Clone, Debug, PartialEq, Eq)]
enum CDPath {
    Root,
    Parent,
    Child(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LSOut {
    File(usize, String),
    Dir(String)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LS {
    out: Vec<LSOut>
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct CD (CDPath);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command{
    LS(LS),
    CD(CD)
}

fn parse_command(s: &str) -> IResult<&str, Command>{
    /*
    1. It begins with ""
    */
    let cd = preceded(tag("cd "), alt((
        value(Command::CD(CD(CDPath::Root)), tag("/")),
        value(Command::CD(CD(CDPath::Parent)), tag("..")),
        map_res(alphanumeric1, |x: &str| {
            let r: Result<Command, Error<&str>> = Result::Ok(Command::CD(CD(CDPath::Child(x.to_string()))));
            r
        })
    )));

    let ls = map_res(preceded(tag("ls\n"), many0(
        alt((
            map(delimited(tag("dir "), alphanumeric1, multispace0), |x: &str| {LSOut::Dir(x.to_string())}),
            map(
                //LSOut::File(1, "la".to_string()),
                delimited(multispace0::<&str, Error<&str>>, tuple((digit1, multispace1, alphanumeric1)), multispace0),
                |res| {LSOut::File(res.0.parse::<usize>().unwrap(), res.2.to_string())}
            )
        ))
    )), |x| {
        let r = Command::LS(LS{out: x});
        let r: Result<Command, Error<&str>> = Result::Ok(r);
        r
    });

    /*
        alt((
            delimited(tag("dir "), alphanumeric1, multispace0),
            delimited(multispace0, tuple((digit1, preceded(multispace0, alphanumeric1))), multispace0)
        ))
    */

    let mut parser = preceded(tag("$ "), alt((cd, ls)));
    return parser(s)
}

#[cfg(test)]
mod tests {
    use super::parse_command;
    use super::{Command, CD, CDPath, LS, LSOut};

    #[test]
    fn test_parse_cd_rt() {
        assert_eq!(parse_command("$ cd /"), Ok(("", Command::CD(CD(CDPath::Root)))));
    }
    #[test]
    fn test_parse_cd_par() {
        assert_eq!(parse_command("$ cd .."), Ok(("", Command::CD(CD(CDPath::Parent)))));
    }
    #[test]
    fn test_parse_cd_chl() {
        assert_eq!(parse_command("$ cd abc"), Ok(("", Command::CD(CD(CDPath::Child("abc".to_string()))))));
    }
    #[test]
    fn test_parse_ls_1dir() {
        assert_eq!(parse_command("$ ls
dir ciao
1 la"), Ok(("", Command::LS(LS { out: vec![
            LSOut::Dir("ciao".to_string()),
            LSOut::File(1, "la".to_string())
        ] }))));

        let r = parse_command("$ ls
dir abc
123 asd
324443 per
dir f");
        println!("Debug res parsed ls {:?}", r.unwrap());
    }
}