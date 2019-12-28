use n_body_problem::*;

fn assert_moons(actual: &[Moon], expected: &[Moon]) {
    assert_eq!(expected.len(), actual.len());
    for i in 0..expected.len() {
        assert_eq!(expected[i], actual[i]);
    }
}

#[test]
fn first_ten_steps() {
    let mut moons = vec![ Moon::new(-1, 0, 2), Moon::new(2, -10, -7), Moon::new(4, -8, 8), Moon::new(3, 5, -1), ];

    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -1, y:   0, z:  2 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
            Moon { position: Vector3D { x:  2, y: -10, z: -7 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
            Moon { position: Vector3D { x:  4, y:  -8, z:  8 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
            Moon { position: Vector3D { x:  3, y:   5, z: -1 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 2, y: -1, z:  1 }, velocity: Vector3D { x:  3, y: -1, z: -1 }, },
            Moon { position: Vector3D { x: 3, y: -7, z: -4 }, velocity: Vector3D { x:  1, y:  3, z:  3 }, },
            Moon { position: Vector3D { x: 1, y: -7, z:  5 }, velocity: Vector3D { x: -3, y:  1, z: -3 }, },
            Moon { position: Vector3D { x: 2, y:  2, z:  0 }, velocity: Vector3D { x: -1, y: -3, z:  1 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 5, y: -3, z: -1 }, velocity: Vector3D { x:  3, y: -2, z: -2 }, },
            Moon { position: Vector3D { x: 1, y: -2, z:  2 }, velocity: Vector3D { x: -2, y:  5, z:  6 }, },
            Moon { position: Vector3D { x: 1, y: -4, z: -1 }, velocity: Vector3D { x:  0, y:  3, z: -6 }, },
            Moon { position: Vector3D { x: 1, y: -4, z:  2 }, velocity: Vector3D { x: -1, y: -6, z:  2 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 5, y: -6, z: -1 }, velocity: Vector3D { x:  0, y: -3, z:  0 }, },
            Moon { position: Vector3D { x: 0, y:  0, z:  6 }, velocity: Vector3D { x: -1, y:  2, z:  4 }, },
            Moon { position: Vector3D { x: 2, y:  1, z: -5 }, velocity: Vector3D { x:  1, y:  5, z: -4 }, },
            Moon { position: Vector3D { x: 1, y: -8, z:  2 }, velocity: Vector3D { x:  0, y: -4, z:  0 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 2, y: -8, z:  0 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
            Moon { position: Vector3D { x: 2, y:  1, z:  7 }, velocity: Vector3D { x:  2, y:  1, z:  1 }, },
            Moon { position: Vector3D { x: 2, y:  3, z: -6 }, velocity: Vector3D { x:  0, y:  2, z: -1 }, },
            Moon { position: Vector3D { x: 2, y: -9, z:  1 }, velocity: Vector3D { x:  1, y: -1, z: -1 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -1, y: -9, z:  2 }, velocity: Vector3D { x: -3, y: -1, z:  2 }, },
            Moon { position: Vector3D { x:  4, y:  1, z:  5 }, velocity: Vector3D { x:  2, y:  0, z: -2 }, },
            Moon { position: Vector3D { x:  2, y:  2, z: -4 }, velocity: Vector3D { x:  0, y: -1, z:  2 }, },
            Moon { position: Vector3D { x:  3, y: -7, z: -1 }, velocity: Vector3D { x:  1, y:  2, z: -2 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -1, y: -7, z:  3 }, velocity: Vector3D { x:  0, y:  2, z:  1 }, },
            Moon { position: Vector3D { x:  3, y:  0, z:  0 }, velocity: Vector3D { x: -1, y: -1, z: -5 }, },
            Moon { position: Vector3D { x:  3, y: -2, z:  1 }, velocity: Vector3D { x:  1, y: -4, z:  5 }, },
            Moon { position: Vector3D { x:  3, y: -4, z: -2 }, velocity: Vector3D { x:  0, y:  3, z: -1 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 2, y: -2, z:  1 }, velocity: Vector3D { x:  3, y:  5, z: -2 }, },
            Moon { position: Vector3D { x: 1, y: -4, z: -4 }, velocity: Vector3D { x: -2, y: -4, z: -4 }, },
            Moon { position: Vector3D { x: 3, y: -7, z:  5 }, velocity: Vector3D { x:  0, y: -5, z:  4 }, },
            Moon { position: Vector3D { x: 2, y:  0, z:  0 }, velocity: Vector3D { x: -1, y:  4, z:  2 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 5, y:  2, z: -2 }, velocity: Vector3D { x:  3, y:  4, z: -3 }, },
            Moon { position: Vector3D { x: 2, y: -7, z: -5 }, velocity: Vector3D { x:  1, y: -3, z: -1 }, },
            Moon { position: Vector3D { x: 0, y: -9, z:  6 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
            Moon { position: Vector3D { x: 1, y:  1, z:  3 }, velocity: Vector3D { x: -1, y:  1, z:  3 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 5, y:  3, z: -4 }, velocity: Vector3D { x: 0, y:  1, z: -2 }, },
            Moon { position: Vector3D { x: 2, y: -9, z: -3 }, velocity: Vector3D { x: 0, y: -2, z:  2 }, },
            Moon { position: Vector3D { x: 0, y: -8, z:  4 }, velocity: Vector3D { x: 0, y:  1, z: -2 }, },
            Moon { position: Vector3D { x: 1, y:  1, z:  5 }, velocity: Vector3D { x: 0, y:  0, z:  2 }, },
        ],
    );

    step(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: 2, y:  1, z: -3 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
            Moon { position: Vector3D { x: 1, y: -8, z:  0 }, velocity: Vector3D { x: -1, y:  1, z:  3 }, },
            Moon { position: Vector3D { x: 3, y: -6, z:  1 }, velocity: Vector3D { x:  3, y:  2, z: -3 }, },
            Moon { position: Vector3D { x: 2, y:  0, z:  4 }, velocity: Vector3D { x:  1, y: -1, z: -1 }, },
        ],
    );

    assert_eq!(6, moons[0].potential_energy());
    assert_eq!(6, moons[0].kinetic_energy());
    assert_eq!(36, moons[0].total_energy());

    assert_eq!(9, moons[1].potential_energy());
    assert_eq!(5, moons[1].kinetic_energy());
    assert_eq!(45, moons[1].total_energy());

    assert_eq!(10, moons[2].potential_energy());
    assert_eq!(8, moons[2].kinetic_energy());
    assert_eq!(80, moons[2].total_energy());

    assert_eq!(6, moons[3].potential_energy());
    assert_eq!(3, moons[3].kinetic_energy());
    assert_eq!(18, moons[3].total_energy());

    assert_eq!(179, moons.iter().map(|m| m.total_energy()).sum());
}
