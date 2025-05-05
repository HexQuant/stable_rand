# Класс LCG с поддержкой нормального распределения (Box-Muller)
LCG <- setRefClass(
  "LCG",
  fields = list(
    state = "numeric",
    a = "numeric",
    c = "numeric",
    m = "numeric",
    next_gaussian = "numeric"  # Для хранения второго числа из Бокса-Мюллера
  ),
  methods = list(
    initialize = function(seed) {
      state <<- seed
      a <<- 1103515245
      c <<- 12345
      m <<- 2^31
      next_gaussian <<- NA  # Изначально нет сохранённого значения
    },
    
    next_uniform = function() {
      state <<- (a * state + c) %% m
      state / m
    },
    
    next_normal = function(mu = 0.0, sigma = 1.0) {
      # Если есть сохранённое значение — возвращаем его
      if (!is.na(next_gaussian)) {
        z <- next_gaussian
        next_gaussian <<- NA
        return(mu + sigma * z)
      }
      
      # Иначе генерируем два числа через Бокса-Мюллера
      u1 <- next_uniform()
      u2 <- next_uniform()
      
      # Избегаем log(0)
      while (u1 == 0) {
        u1 <- next_uniform()
      }
      
      z0 <- sqrt(-2 * log(u1)) * cos(2 * pi * u2)
      z1 <- sqrt(-2 * log(u1)) * sin(2 * pi * u2)
      
      # Сохраняем второе число для следующего вызова
      next_gaussian <<- z1
      mu + sigma * z0
    }
  )
)