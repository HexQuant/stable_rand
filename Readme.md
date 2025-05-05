# Стабильные генераторы случайных чисел для отладки

При инициализации одним и тем же *seed* генераторы будут выдавать одинаковую последовательность случайных чисел вне зависимости от платформы и ЯП.

При *seed=42* последовательность: *[-1.0319055269422748, -0.12916623354070006, 0.20893216991782163, -1.2180261988090866, 1.28328531420276, ...]*

Пример использования:

Python:

```python
from stable_rand import LCG

seed = 42
lcg = LCG(seed)
for _ in range(5):
    print(lcg.next_normal(0, 1))
```

Julia:

```julia
using .StableRand

seed::UInt32 = 42
rng = StableRand::LCG(seed)
for _ in 1:5
    println(StableRand::next_normal(rng, mu=0.0, sigma=1.0))
end
```

Rust:

```rust
use stable_rand::LCG 

let mut rng = LCG::new(42);
for _ in 0..5 {
    println!("{}", rng.next_normal(0.0, 1.0));
}
```

R:

```r
lcg <- LCG$new(42)
for (i in 1:5) {
  print(lcg$next_normal(mu = 5, sigma = 2))
}
```
