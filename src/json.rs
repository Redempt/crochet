use crate::*;

type Result<'a, T> = ParserResult<'a, T, ParserError>;

fn parse_num(input: &str) -> Result<f64> {
    literal("-", input)
        .optional(input)
        .and(cur!(take_while <= "digit", |c| c.is_ascii_digit()))
        .and(|s| {
            literal(".", s)
                .and(cur!(take_while <= "digit", |c| c.is_ascii_digit()))
                .optional(s)
        })
        .map_slice(input, |s| s.parse().unwrap())
}

fn parse_str(s: &str) -> Result<String> {
    let (mut s, _) = literal("\"", s)?;
    let string: String = iter(
        |s| parse_esc(s).or(s, |s| matching_char("char", |c| c != '"', s)),
        &mut s,
    )
    .ok()
    .collect();
    let (s, _) = literal("\"", s)?;
    ParserResult::from_val(s, string)
}

fn parse_esc(s: &str) -> Result<char> {
    let (s, _) = literal("\\", s)?;
    let (s, c) = advance(s)?;
    ParserResult::from_val(
        s,
        match c {
            'n' => '\n',
            't' => '\t',
            _ => c,
        },
    )
}
