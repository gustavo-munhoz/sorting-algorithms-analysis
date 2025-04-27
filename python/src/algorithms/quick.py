from src.algorithms.base import SortAlgorithm


def quick_sort(data: [int], low: int, high: int, m) -> None:
    if low < high:
        pivot_location = partition(data, low, high, m)
        if pivot_location > 0:
            quick_sort(data, low, pivot_location, m)
        quick_sort(data, pivot_location + 1, high, m)


def partition(data: list, low: int, high: int, m) -> int:
    pivot = data[low]
    left_wall = low

    for i in range(low + 1, high + 1):
        m.comparisons += 1
        if data[i] < pivot:
            left_wall += 1
            data[i], data[left_wall] = data[left_wall], data[i]
            m.swaps += 1

    data[low], data[left_wall] = data[left_wall], data[low]
    m.swaps += 1
    return left_wall


class QuickSort(SortAlgorithm):
    name = "QuickSort"

    def sort(self, data, m):
        quick_sort(data, 0, len(data) - 1, m)
