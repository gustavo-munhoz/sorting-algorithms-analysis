import unittest
from typing import Type

from src.algorithms import BubbleSort, InsertionSort, MergeSort, QuickSort, SelectionSort
from src.algorithms.base import SortAlgorithm
from src.metrics import Metrics

class TestSortAlgorithms(unittest.TestCase):
    algorithms: list[Type[SortAlgorithm]] = [
        BubbleSort,
        InsertionSort,
        MergeSort,
        QuickSort,
        SelectionSort
    ]

    passed_algorithms: set[str] = set()
    failed_algorithms: set[str] = set()

    def setUp(self):
        self.test_cases = [
            [],
            [1],
            [2, 1],
            [1, 2, 3, 4, 5],
            [5, 4, 3, 2, 1],
            [3, 1, 2, 3, 1, 2],
            list(range(100, 0, -1)),
        ]

    def test_sort_correctness(self):
        for algorithm_cls in self.algorithms:
            algorithm_name = algorithm_cls.__name__
            all_passed = True

            for case in self.test_cases:
                with self.subTest(algorithm=algorithm_name, case=case):
                    alg = algorithm_cls()
                    data = case.copy()
                    metrics = Metrics()

                    try:
                        alg.sort(data, metrics)
                        self.assertEqual(
                            data,
                            sorted(case),
                            msg=(
                                f"\n\nAlgorithm: {algorithm_name}"
                                f"\nInput: {case}"
                                f"\nExpected: {sorted(case)}"
                                f"\nGot: {data}\n"
                            )
                        )
                    except AssertionError:
                        all_passed = False
                        raise

            if all_passed:
                TestSortAlgorithms.passed_algorithms.add(algorithm_name)
            else:
                TestSortAlgorithms.failed_algorithms.add(algorithm_name)

    @classmethod
    def tearDownClass(cls):
        print("\n\n--- Summary ---")
        for alg in sorted(cls.passed_algorithms):
            print(f"✅ {alg} PASSED")
        for alg in sorted(cls.failed_algorithms):
            print(f"❌ {alg} FAILED")
        print("----------------\n")

if __name__ == "__main__":
    unittest.main(verbosity=2)