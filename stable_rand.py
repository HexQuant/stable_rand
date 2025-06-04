"""Стабильный генератор случайных чисел"""

import math
import sys

from numba import float64, optional, uint32
from numba.experimental import jitclass

spec = [
    ("state", uint32),  # a simple scalar field
    ("a", uint32),  # an array field
    ("c", uint32),  # an array field
    ("m", uint32),  # an array field
    ("next_gaussian", optional(float64)),  # an array field
]


@jitclass(spec)  # type: ignore
class LCG:
    """
    Класс для генерации равномерно распределённых случайных чисел
    """

    def __init__(self, seed: int) -> None:
        if seed < 0:
            raise ValueError("Семя должно быть положительным числом")
        self.state = seed
        self.a = 1103515245
        self.c = 12345
        self.m = 2**31
        self.next_gaussian = None  # Для хранения второго числа из Бокса-Мюллера

    def next_uniform(self) -> float:
        """
        Получить равномерно распределённое случайное число на интервале [0,1)

        Returns:
            float: _description_
        """
        self.state = (self.a * self.state + self.c) % self.m
        return self.state / self.m

    def next_normal(self, mu: float = 0.0, sigma: float = 1.0) -> float:
        """
        Возвращает нормально распределённую случайное число

        Args:
            mu (float, optional): мат. ожидание. Defaults to 0.0.
            sigma (float, optional): дисперсия. Defaults to 1.0.

        Returns:
            float: _description_
        """
        if sigma <= 0:
            raise ValueError("Дисперсия должна быть положительным числом")

        if self.next_gaussian is not None:
            z = self.next_gaussian
            self.next_gaussian = None
            return mu + sigma * z

        # Бокса-Мюллера
        u1 = self.next_uniform()
        u2 = self.next_uniform()
        while u1 == 0:  # Избегаем log(0)
            u1 = self.next_uniform()

        z0 = math.sqrt(-2.0 * math.log(u1)) * math.cos(2.0 * math.pi * u2)
        z1 = math.sqrt(-2.0 * math.log(u1)) * math.sin(2.0 * math.pi * u2)

        self.next_gaussian = z1
        return mu + sigma * z0


def main() -> None:
    """
    Пример использования
    """
    seed = 42
    lcg = LCG(seed)

    for _ in range(5):
        print(lcg.next_normal(0, 1))


if __name__ == "__main__":
    sys.exit(main())
