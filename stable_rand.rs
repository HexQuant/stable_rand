use std::f64::consts::PI;

struct LCG {
    state: u32,
    a: u32,
    c: u32,
    m: u32,
    next_gaussian: Option<f64>,
}

impl LCG {
    pub fn new(seed: u32) -> Self {
        LCG {
            state: seed,
            a: 1103515245,
            c: 12345,
            m: 2_u32.pow(31),
            next_gaussian: None,
        }
    }

    pub fn next_uniform(&mut self) -> f64 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        f64::from(self.state) / f64::from(self.m)
    }

    pub fn next_normal(&mut self, mu: f64, sigma: f64) -> f64 {
        if let Some(z) = self.next_gaussian.take() {
            return mu + sigma * z;
        }

        // Бокса-Мюллера
        let u1 = self.next_uniform();
        let u2 = self.next_uniform();
        let u1 = if u1 == 0.0 { self.next_uniform() } else { u1 }; // Избегаем log(0)

        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).sin();

        self.next_gaussian = Some(z1);
        mu + sigma * z0
    }
}

fn main() {
    let mut rng = LCG::new(42);
    for _ in 0..5 {
        println!("{}", rng.next_normal(0.0, 1.0));
    }
}

// rustc -C target-cpu=native -C opt-level=3  rand.rs && ./rand
