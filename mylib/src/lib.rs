pub fn checkit(haystack: &str) -> anyhow::Result<bool>
{
    let re = regex::Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(haystack).map(|c| c.extract()) {
        results.push((path, lineno.parse::<u64>()?, line));
    }
    let expected = vec![
        ("path/to/foo", 54, "Blue Harvest"),
        ("path/to/bar", 90, "Something, Something, Something, Dark Side"),
        ("path/to/baz", 3, "It's a Trap!"),
    ];
    return Ok(results == expected);
}
