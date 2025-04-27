from src.algorithms.base import SortAlgorithm


def quick_sort(data: [int], low: int, high: int) -> None:
    if low < high:
        pivot_location = partition(data, low, high)
        quick_sort(data, low, pivot_location)
        quick_sort(data, pivot_location + 1, high)


def partition(data: list, low: int, high: int) -> int:
    pivot = data[low]
    left_wall = low

    for i in range(low + 1, high + 1):
        if data[i] < pivot:
            left_wall += 1
            data[i], data[left_wall] = data[left_wall], data[i]

    data[low], data[left_wall] = data[left_wall], data[low]
    return left_wall


class QuickSort(SortAlgorithm):
    name = "QuickSort"

    def sort(self, data):
        quick_sort(data, 0, len(data) - 1)
