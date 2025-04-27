from .base import SortAlgorithm


class InsertionSort(SortAlgorithm):
    name = "InsertionSort"

    def sort(self, data, m):
        for i in range(1, len(data)):
            key = data[i]
            j = i
            while j >= 0:
                m.comparisons += 1
                if key < data[j - 1]:
                    break

                data[j] = data[j - 1]
                j -= 1
                m.swaps += 1
            data[j] = key
