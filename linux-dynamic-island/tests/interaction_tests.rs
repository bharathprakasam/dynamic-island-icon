#[test]
fn test_spring_step_convergence() {
    let mut current = 100.0;
    let target = 200.0;
    let mut velocity = 0.0;
    let dt = 0.016; // ~60fps
    let tension = 180.0;
    let friction = 18.0;
    let mass = 1.0;

    for _ in 0..300 {
        let displacement = current - target;
        let spring_force = -tension * displacement;
        let damping_force = -friction * velocity;
        let acceleration = (spring_force + damping_force) / mass;

        velocity += acceleration * dt;
        current += velocity * dt;
    }

    // Should converge close to target (200.0)
    assert!((current - target).abs() < 0.1);
}
