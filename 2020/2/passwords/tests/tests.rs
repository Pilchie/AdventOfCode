use passwords::*;

#[test]
fn first() {
    assert_eq!(true, is_valid_line(&"1-3 a: abcde"));
}

#[test]
fn second() {
    assert_eq!(false, is_valid_line(&"1-3 b: cdefg"));
}

#[test]
fn third() {
    assert_eq!(true, is_valid_line(&"2-9 c: ccccccccc"));
}

fn is_valid_line(line: &str) -> bool {
    let password_line = PasswordLine::parse(line);
    password_line.is_valid()
}
