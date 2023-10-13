use regex::Regex;

fn main() -> anyhow::Result<()>
{
    let myvar = std::env::var("MYVAR")?;

    let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(&myvar).map(|c| c.extract()) {
        results.push((path, lineno.parse::<u64>()?, line));
    }
    assert_eq!(results, vec![
        ("path/to/foo", 54, "Blue Harvest"),
        ("path/to/bar", 90, "Something, Something, Something, Dark Side"),
        ("path/to/baz", 3, "It's a Trap!"),
    ]);
    return Ok(());
}
