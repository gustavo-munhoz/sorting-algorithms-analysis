from typing import Iterable
from .algorithms.base import SortAlgorithm
from .dataset import random_list
from .metrics import Metrics
import time


def run(
        sorters: Iterable[type[SortAlgorithm]],
        sizes=(1_000, 100_000, 1_000_000)
):
    results = []

    for n in sizes:
        data = random_list(n)

        for sorter_cls in sorters:
            sorter = sorter_cls()
            data_copy = data[:]
            m = Metrics()
            start = time.perf_counter_ns()
            sorter.sort(data_copy)
            m.elapsed_ns = time.perf_counter_ns() - start
            results.append(
                {
                    "algorithm": sorter.name,
                    "N": n,
                    "time_ns": m.elapsed_ns
                }
            )

    return results


def run_one(sorter: SortAlgorithm, n: int) -> dict:
    data = random_list(n)
    start = time.perf_counter()
    m = Metrics()
    sorter.sort(data, m)
    elapsed = time.perf_counter() - start


    return {
        "algorithm": sorter.name,
        "n": n,
        "time_s": elapsed,
    }
