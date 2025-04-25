from typing import List, Callable
import random

Generator = Callable[[int], List[int]]


def random_list(n: int) -> List[int]:
    return [random.randint(0, n) for _ in range(n)]


def nearly_sorted(n: int, swaps: int = 10) -> List[int]:
    data = list(range(n))

    for _ in range(swaps):
        i, j = random.sample(range(n), 2)
        data[i], data[j] = data[j], data[i]

    return data
