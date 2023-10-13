fn main()
{
    let myvar = std::env::var("MYVAR").unwrap();

    if mylib::checkit(&myvar).unwrap() {
        println!("true");
    } else {
        println!("not true");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hay = "\
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
";
        let haystack = "yeah";
        assert!(mylib::checkit(&hay).unwrap());
        assert!(mylib::checkit(&haystack).unwrap() == false);
    }
}
