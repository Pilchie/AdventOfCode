use n_body_problem::*;

fn part2() {
    let mut moons = [ Moon::new(19, -10, 7), Moon::new(1, 2, -3), Moon::new(14, -4, 1), Moon::new(8, 7, -6), ];
    let count = find_repeat(&mut moons);
    println!("Repeated after {} steps", count);
}

fn part1() {
    let mut moons = [ Moon::new(19, -10, 7), Moon::new(1, 2, -3), Moon::new(14, -4, 1), Moon::new(8, 7, -6), ];

    for _ in 0..1000 {
        step(&mut moons);
    }

   let total_energy: i32 = moons.iter().map(|m| m.total_energy()).sum();
   println!("{}", total_energy);
}

fn main() {
    part1();
    part2();
}
