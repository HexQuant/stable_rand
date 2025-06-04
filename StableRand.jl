module StableRand

mutable struct LCG
    state::UInt32
    a::UInt32
    c::UInt32
    m::UInt32
    next_gaussian::Union{Nothing,Float64}
end

"""
    LCG(seed::UInt32)

Создаём генератор случайных чисел который инкапсулирует состоянием
"""
function LCG(seed::UInt32)
    LCG(seed, 1103515245, 12345, 2^31, nothing)
end

"""
    next_uniform(rng::LCG)::Float64

Получить равномерно распределённое случайное число на интервале [0,1)
"""
function next_uniform(rng::LCG)::Float64
    rng.state = (rng.a * rng.state + rng.c) % rng.m
    rng.state / rng.m
end

"""
    next_normal(rng::LCG; mu::Float64=0.0, sigma::Float64=1.0)::Float64

Возвращает нормально распределённое случайное число с мат ожиданием mu и дисперсией sigma
"""
function next_normal(rng::LCG; mu::Float64=0.0, sigma::Float64=1.0)::Float64
    if sigma <= 0.0
        throw(DomainError(sigma, "Дисперсия должна быть положительной"))
    end
    if rng.next_gaussian !== nothing
        z = rng.next_gaussian
        rng.next_gaussian = nothing
        return mu + sigma * z
    end

    # Бокса-Мюллера
    u1 = next_uniform(rng)
    u2 = next_uniform(rng)
    while u1 == 0  # Избегаем log(0)
        u1 = next_uniform(rng)
    end

    z0 = sqrt(-2.0 * log(u1)) * cos(2.0 * π * u2)
    z1 = sqrt(-2.0 * log(u1)) * sin(2.0 * π * u2)

    rng.next_gaussian = z1
    mu + sigma * z0
end

function test()
    # Пример использования
    seed::UInt32 = 42
    rng = LCG(seed)
    for _ in 1:5
        println(next_normal(rng, mu=0.0, sigma=1.0))
    end

end

end

