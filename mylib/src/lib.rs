pub fn checkit(haystack: &str) -> Result<bool, std::num::ParseIntError>
{
    let re = regex::Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();

    anyhow::pwn();

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(haystack).map(|c| c.extract()) {
        let r = lineno.parse::<u64>();
        match r {
            Ok(v) => results.push((path, v, line)),
            Err(e) => return Err(e),
        }
    }
    let expected = vec![
        ("path/to/foo", 54, "Blue Harvest"),
        ("path/to/bar", 90, "Something, Something, Something, Dark Side"),
        ("path/to/baz", 3, "It's a Trap!"),
    ];


    return Ok(results == expected);
}
