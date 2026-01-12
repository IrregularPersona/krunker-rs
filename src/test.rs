fn test() -> i32 {
    let mut m = 20;
    let n = 5;
    let _ = foo(&m);

    m = n + 2;

    m
}

fn foo(num: &i32) -> i32 {
    num + 10
}
