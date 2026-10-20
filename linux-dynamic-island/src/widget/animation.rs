#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutCubic,
    Spring { tension: f64, friction: f64, mass: f64 },
}

impl Default for Easing {
    fn default() -> Self {
        Easing::Spring {
            tension: 180.0,
            friction: 18.0,
            mass: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpringState {
    pub current: f64,
    pub target: f64,
    pub velocity: f64,
}

impl SpringState {
    pub fn new(initial: f64) -> Self {
        Self {
            current: initial,
            target: initial,
            velocity: 0.0,
        }
    }

    /// Step the spring simulation forward by `dt` seconds
    pub fn step(&mut self, dt: f64, tension: f64, friction: f64, mass: f64) -> bool {
        let displacement = self.current - self.target;
        let spring_force = -tension * displacement;
        let damping_force = -friction * self.velocity;
        let acceleration = (spring_force + damping_force) / mass;

        self.velocity += acceleration * dt;
        self.current += self.velocity * dt;

        // Check if settled
        let is_at_target = (self.current - self.target).abs() < 0.001;
        let is_still = self.velocity.abs() < 0.001;

        if is_at_target && is_still {
            self.current = self.target;
            self.velocity = 0.0;
            true // Settled
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionInterpolator {
    pub width: SpringState,
    pub height: SpringState,
    pub corner_radius: SpringState,
    pub opacity: SpringState,
    pub tension: f64,
    pub friction: f64,
    pub mass: f64,
}

impl DimensionInterpolator {
    pub fn new(w: f64, h: f64, r: f64, o: f64) -> Self {
        Self {
            width: SpringState::new(w),
            height: SpringState::new(h),
            corner_radius: SpringState::new(r),
            opacity: SpringState::new(o),
            tension: 200.0,
            friction: 20.0,
            mass: 1.0,
        }
    }

    pub fn set_target(&mut self, w: f64, h: f64, r: f64, o: f64) {
        self.width.target = w;
        self.height.target = h;
        self.corner_radius.target = r;
        self.opacity.target = o;
    }

    /// Advances the animations. Returns true when all dimensions have reached their targets.
    pub fn update(&mut self, dt: f64) -> bool {
        let w_done = self.width.step(dt, self.tension, self.friction, self.mass);
        let h_done = self.height.step(dt, self.tension, self.friction, self.mass);
        let r_done = self.corner_radius.step(dt, self.tension, self.friction, self.mass);
        let o_done = self.opacity.step(dt, self.tension, self.friction, self.mass);

        w_done && h_done && r_done && o_done
    }
}
