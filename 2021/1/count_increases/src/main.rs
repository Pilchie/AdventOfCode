fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut count = 0;

    let lines: Vec<_> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut last = lines[0] + lines[1] + lines[2];
    for i in 3..lines.len() {
        let cur = lines[i] + lines[i-1] + lines[i-2];
        if cur > last {
            count += 1;
        }
        last = cur
    }

    println!("There were {} increases", count);
    Ok(())
}
