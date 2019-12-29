use n_body_problem::*;

fn assert_moons(actual: &[Moon], expected: &[Moon]) {
    assert_eq!(expected.len(), actual.len());
    for i in 0..expected.len() {
        assert_eq!(expected[i], actual[i]);
    }
}

fn step10(moons: &mut [Moon]) {
    let mut px: Vec<i32> = moons.iter().map(|m| m.position.x).collect();
    let mut py: Vec<i32> = moons.iter().map(|m| m.position.y).collect();
    let mut pz: Vec<i32> = moons.iter().map(|m| m.position.z).collect();
    let mut vx: Vec<i32> = moons.iter().map(|m| m.velocity.x).collect();
    let mut vy: Vec<i32> = moons.iter().map(|m| m.velocity.y).collect();
    let mut vz: Vec<i32> = moons.iter().map(|m| m.velocity.z).collect();

    for _ in 0..10 {
        step_one(&mut px, &mut vx);
        step_one(&mut py, &mut vy);
        step_one(&mut pz, &mut vz);
    }

    for i in 0..moons.len() {
        moons[i] = Moon { position: Vector3D { x: px[i], y: py[i], z: pz[i]}, velocity: Vector3D { x: vx[i], y: vy[i], z: vz[i] } }
    }
}

#[test]
fn first_100_steps() {
    let mut moons = vec![ Moon::new(-8, -10, 0), Moon::new(5, 5, 10), Moon::new(2, -7, 3), Moon::new(9, -8, -3), ];

    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  -8, y:-10, z:  0 }, velocity: Vector3D { x:  0, y:  0, z:  0 }, },
            Moon { position: Vector3D { x:   5, y:  5, z: 10 }, velocity: Vector3D { x:  0, y:  0, z:  0 }, },
            Moon { position: Vector3D { x:   2, y: -7, z:  3 }, velocity: Vector3D { x:  0, y:  0, z:  0 }, },
            Moon { position: Vector3D { x:   9, y: -8, z: -3 }, velocity: Vector3D { x:  0, y:  0, z:  0 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  -9, y:-10, z:  1 }, velocity: Vector3D { x: -2, y: -2, z: -1 }, },
            Moon { position: Vector3D { x:   4, y: 10, z:  9 }, velocity: Vector3D { x: -3, y:  7, z: -2 }, },
            Moon { position: Vector3D { x:   8, y:-10, z: -3 }, velocity: Vector3D { x:  5, y: -1, z: -2 }, },
            Moon { position: Vector3D { x:   5, y:-10, z:  3 }, velocity: Vector3D { x:  0, y: -4, z:  5 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -10, y:  3, z: -4 }, velocity: Vector3D { x: -5, y:  2, z:  0 }, },
            Moon { position: Vector3D { x:   5, y:-25, z:  6 }, velocity: Vector3D { x:  1, y:  1, z: -4 }, },
            Moon { position: Vector3D { x:  13, y:  1, z:  1 }, velocity: Vector3D { x:  5, y: -2, z:  2 }, },
            Moon { position: Vector3D { x:   0, y:  1, z:  7 }, velocity: Vector3D { x: -1, y: -1, z:  2 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  15, y: -6, z: -9 }, velocity: Vector3D { x: -5, y:  4, z:  0 }, },
            Moon { position: Vector3D { x:  -4, y:-11, z:  3 }, velocity: Vector3D { x: -3, y:-10, z:  0 }, },
            Moon { position: Vector3D { x:   0, y: -1, z: 11 }, velocity: Vector3D { x:  7, y:  4, z:  3 }, },
            Moon { position: Vector3D { x:  -3, y: -2, z:  5 }, velocity: Vector3D { x:  1, y:  2, z: -3 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  14, y:-12, z: -4 }, velocity: Vector3D { x: 11, y:  3, z:  0 }, },
            Moon { position: Vector3D { x:  -1, y: 18, z:  8 }, velocity: Vector3D { x: -5, y:  2, z:  3 }, },
            Moon { position: Vector3D { x:  -5, y:-14, z:  8 }, velocity: Vector3D { x:  1, y: -2, z:  0 }, },
            Moon { position: Vector3D { x:   0, y:-12, z: -2 }, velocity: Vector3D { x: -7, y: -3, z: -3 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -23, y:  4, z:  1 }, velocity: Vector3D { x: -7, y: -1, z:  2 }, },
            Moon { position: Vector3D { x:  20, y:-31, z: 13 }, velocity: Vector3D { x:  5, y:  3, z:  4 }, },
            Moon { position: Vector3D { x:  -4, y:  6, z:  1 }, velocity: Vector3D { x: -1, y:  1, z: -3 }, },
            Moon { position: Vector3D { x:  15, y:  1, z: -5 }, velocity: Vector3D { x:  3, y: -3, z: -3 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  36, y:-10, z:  6 }, velocity: Vector3D { x:  5, y:  0, z:  3 }, },
            Moon { position: Vector3D { x: -18, y: 10, z:  9 }, velocity: Vector3D { x: -3, y: -7, z:  5 }, },
            Moon { position: Vector3D { x:   8, y:-12, z: -3 }, velocity: Vector3D { x: -2, y:  1, z: -7 }, },
            Moon { position: Vector3D { x: -18, y: -8, z: -2 }, velocity: Vector3D { x:  0, y:  6, z: -1 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -33, y: -6, z:  5 }, velocity: Vector3D { x: -5, y: -4, z:  7 }, },
            Moon { position: Vector3D { x:  13, y: -9, z:  2 }, velocity: Vector3D { x: -2, y: 11, z:  3 }, },
            Moon { position: Vector3D { x:  11, y: -8, z:  2 }, velocity: Vector3D { x:  8, y: -6, z: -7 }, },
            Moon { position: Vector3D { x:  17, y:  3, z:  1 }, velocity: Vector3D { x: -1, y: -1, z: -3 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:  30, y: -8, z:  3 }, velocity: Vector3D { x:  3, y:  3, z:  0 }, },
            Moon { position: Vector3D { x:  -2, y: -4, z:  0 }, velocity: Vector3D { x:  4, y:-13, z:  2 }, },
            Moon { position: Vector3D { x: -18, y: -7, z: 15 }, velocity: Vector3D { x: -8, y:  2, z: -2 }, },
            Moon { position: Vector3D { x:  -2, y: -1, z: -8 }, velocity: Vector3D { x:  1, y:  8, z:  0 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x: -25, y: -1, z:  4 }, velocity: Vector3D { x:  1, y: -3, z:  4 }, },
            Moon { position: Vector3D { x:   2, y: -9, z:  0 }, velocity: Vector3D { x: -3, y: 13, z: -1 }, },
            Moon { position: Vector3D { x:  32, y: -8, z: 14 }, velocity: Vector3D { x:  5, y: -4, z:  6 }, },
            Moon { position: Vector3D { x:  -1, y: -2, z: -8 }, velocity: Vector3D { x: -3, y: -6, z: -9 }, },
        ],
    );

    step10(&mut moons);
    assert_moons(
        &moons,
        &vec![
            Moon { position: Vector3D { x:   8, y:-12, z: -9 }, velocity: Vector3D { x: -7, y:  3, z:  0 }, },
            Moon { position: Vector3D { x:  13, y: 16, z: -3 }, velocity: Vector3D { x:  3, y:-11, z: -5 }, },
            Moon { position: Vector3D { x: -29, y:-11, z: -1 }, velocity: Vector3D { x: -3, y:  7, z:  4 }, },
            Moon { position: Vector3D { x:  16, y:-13, z: 23 }, velocity: Vector3D { x:  7, y:  1, z:  1 }, },
        ],
    );

    assert_eq!(29, moons[0].potential_energy());
    assert_eq!(10, moons[0].kinetic_energy());
    assert_eq!(290, moons[0].total_energy());

    assert_eq!(32, moons[1].potential_energy());
    assert_eq!(19, moons[1].kinetic_energy());
    assert_eq!(608, moons[1].total_energy());
    
    assert_eq!(41, moons[2].potential_energy());
    assert_eq!(14, moons[2].kinetic_energy());
    assert_eq!(574, moons[2].total_energy());

    assert_eq!(52, moons[3].potential_energy());
    assert_eq!(9, moons[3].kinetic_energy());
    assert_eq!(468, moons[3].total_energy());

    assert_eq!(1940, moons.iter().map(|m| m.total_energy()).sum());
}
