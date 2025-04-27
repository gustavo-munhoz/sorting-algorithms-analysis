from .base import SortAlgorithm


class InsertionSort(SortAlgorithm):
    name = "InsertionSort"

    def sort(self, data, m):
        for i in range(1, len(data)):
            key = data[i]
            j = i - 1
            m.comparisons += 1

            while j >= 0 and data[j] > key:
                data[j + 1] = data[j]
                j -= 1
                m.swaps += 1

            data[j + 1] = key
