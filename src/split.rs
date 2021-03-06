use crate::args::ProcessedArgs;

pub fn process_line<'a, 'b>(args: &'b ProcessedArgs, s: &'a str) -> Option<&'a str> {
    match args
        .raw
        .filter
        .as_ref()
        .map(|r| r.is_match(s))
        .unwrap_or(true)
    {
        true => args.raw.delimiter.split(s).nth(args.field - 1).or(Some("")),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::args::{Args, ProcessedArgs};
    use crate::split::process_line;
    use anyhow::Result;
    use regex::Regex;

    #[test]
    fn test_delimiters() -> Result<()> {
        let cases = vec![
            (r"\s+", "1 2 3 4", Some("2")),
            (r"\s+", "1    2    3    4   ", Some("2")),
            (r",\s+", "1, 2, 3, 4", Some("2")),
            (r",", "1,2,3,4", Some("2")),
            (r"\s+", "1234", Some("")),
        ];
        let mut buf: String;
        for (d, input, expected) in cases {
            let args = ProcessedArgs {
                raw: Args {
                    delimiter: Regex::new(d)?,
                    filter: None,
                    field: Some(2),
                    col_name: None,
                },
                field: 2,
            };
            buf = String::from(input);
            let actual = process_line(&args, &mut buf);
            assert_eq!(actual, expected);
        }
        Ok(())
    }
}
