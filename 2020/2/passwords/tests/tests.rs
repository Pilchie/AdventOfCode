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

#[test]
fn first_part2() {
    assert_eq!(true, is_valid_part2(&"1-3 a: abcde"));
}

#[test]
fn second_part2() {
    assert_eq!(false, is_valid_part2(&"1-3 b: cdefg"));
} 

#[test]
fn third_part2() {
    assert_eq!(false, is_valid_part2(&"2-9 c: ccccccccc"));

}
fn is_valid_line(line: &str) -> bool {
    let password_line = PasswordLine::parse(line);
    password_line.is_valid_part1()
}

fn is_valid_part2(line: &str) -> bool {
    let password_line = PasswordLine::parse(line);
    password_line.is_valid_part2()
}
