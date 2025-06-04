use std::f64::consts::PI;

/// Псевдослучайный генератор на основе LCG
struct LCG {
    state: u32,
    a: u32,
    c: u32,
    m: u32,
    next_gaussian: Option<f64>,
}

impl LCG {
    /// Создает новый экземпляр LCG с заданным начальным состоянием.
    pub fn new(seed: u32) -> Self {
        LCG {
            state: seed,
            a: 1103515245,
            c: 12345,
            m: 2_u32.pow(31),
            next_gaussian: None,
        }
    }

    /// Генерирует следующее псевдослучайное число в диапазоне [0, 1).
    pub fn next_uniform(&mut self) -> f64 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        f64::from(self.state) / f64::from(self.m)
    }

    /// Генерирует следующее псевдослучайное число из нормального распределения с заданным математическим ожиданием и стандартным отклонением.
    pub fn next_normal(&mut self, mu: f64, sigma: f64) -> f64 {
        if sigma <= 0.0 {
            panic!("Стандартное отклонение должно быть больше 0");
        }
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

/// Пример использования
fn main() {
    let mut rng = LCG::new(42);
    for _ in 0..5 {
        println!("{}", rng.next_normal(0.0, 1.0));
    }
}

// rustc -C target-cpu=native -C opt-level=3  rand.rs && ./rand

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_lcg_new() {
        let seed = 42;
        let lcg = LCG::new(seed);
        assert_eq!(lcg.state, seed);
        assert_eq!(lcg.a, 1103515245);
        assert_eq!(lcg.c, 12345);
        assert_eq!(lcg.m, 2_u32.pow(31));
        assert!(lcg.next_gaussian.is_none());
    }

    #[test]
    fn test_next_uniform() {
        let mut lcg = LCG::new(42);
        let mut results = Vec::new();
        for _ in 0..1000 {
            let num = lcg.next_uniform();
            assert!(num >= 0.0 && num < 1.0);
            results.push(num);
        }

        // Проверяем, что числа не повторяются подряд (базовый тест на случайность)
        let unique_count = results.iter().collect::<HashSet<_>>().len();
        assert!(unique_count > 500, "Слишком много повторяющихся значений");
    }

    #[test]
    fn test_next_normal() {
        let mut lcg = LCG::new(42);
        let mut results = Vec::new();
        let mu = 5.0;
        let sigma = 2.0;

        for _ in 0..1000 {
            let num = lcg.next_normal(mu, sigma);
            results.push(num);
        }

        let mean = results.iter().sum::<f64>() / results.len() as f64;
        let variance =
            results.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / results.len() as f64;
        let std_dev = variance.sqrt();

        // Проверяем, что среднее и стандартное отклонение близки к ожидаемым
        assert!(
            (mean - mu).abs() < 0.2,
            "Среднее значение отличается от ожидаемого"
        );
        assert!(
            (std_dev - sigma).abs() < 0.2,
            "Стандартное отклонение отличается от ожидаемого"
        );
    }

    #[test]
    fn test_next_normal_gaussian_pair() {
        let mut lcg = LCG::new(42);
        let mut results = Vec::new();

        // Генерируем нечётное количество чисел, чтобы проверить сохранение второго значения из пары
        for _ in 0..1001 {
            results.push(lcg.next_normal(0.0, 1.0));
        }

        // Проверяем, что next_gaussian используется корректно
        let mut gaussian_values = Vec::new();
        lcg.next_gaussian = Some(1.0);
        gaussian_values.push(lcg.next_normal(0.0, 1.0));
        assert_eq!(gaussian_values[0], 1.0);
        assert!(lcg.next_gaussian.is_none());
    }

    #[test]
    fn test_next_uniform_distribution() {
        let mut lcg = LCG::new(42);
        let mut buckets = [0; 10];
        let total_numbers = 10000;

        for _ in 0..total_numbers {
            let num = lcg.next_uniform();
            let index = (num * 10.0) as usize;
            buckets[index] += 1;
        }

        // Проверяем равномерность распределения
        let expected = total_numbers / 10;
        for &count in &buckets {
            assert!(
                (count - expected).abs() <= 200,
                "Распределение не равномерное"
            );
        }
    }
}
