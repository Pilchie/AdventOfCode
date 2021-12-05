fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut count = 0;
    let mut last = 0;
    let mut first = true;
    for l in &mut input.lines() {
        if first {
            first = false
        } else {
            if l.parse::<i32>().unwrap() > last {
                count = count + 1
            }
        }
        last = l.parse().unwrap()
    }


    println!("There were {} increases", count);
    Ok(())
}
