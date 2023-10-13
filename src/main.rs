fn main()
{
    let myvar = std::env::var("MYVAR").unwrap();

    if mylib::checkit(&myvar).unwrap() {
        println!("true");
    } else {
        println!("not true");
    }
}
