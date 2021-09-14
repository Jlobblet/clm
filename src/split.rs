use crate::args::ProcessedArgs;

pub fn process_line<'a, 'b>(args: &'b ProcessedArgs, s: &'a str) -> Option<&'a str> {
    if args.raw.filter.as_ref().map(|r| r.is_match(s)).unwrap_or(false) {
        args.raw.delimiter.split(s).nth(args.field - 1).or(Some(""))
    } else {
        None
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
            (r"\s+", "1 2 3 4", "2"),
            (r"\s+", "1    2    3    4   ", "2"),
            (r",\s+", "1, 2, 3, 4", "2"),
            (r",", "1,2,3,4", "2"),
            (r"\s+", "1234", ""),
        ];
        let mut buf: String;
        for (d, input, expected) in cases {
            let args = ProcessedArgs {
                raw: Args {
                    delimiter: Regex::new(d)?,
                    filter: None,
                    pcre: false,
                    field: Some(2),
                    col_name: None,
                },
                field: 2,
            };
            buf = String::from(input);
            let actual = process_line(&args, &mut buf).as_bytes();
            assert_eq!(actual, expected.as_bytes());
        }
        Ok(())
    }
}
