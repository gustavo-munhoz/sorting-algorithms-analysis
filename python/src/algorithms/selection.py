from .base import SortAlgorithm


class SelectionSort(SortAlgorithm):
    name = "SelectionSort"

    def sort(self, data, m):
        for i in range(len(data)):
            min_index = i
            for j in range(i + 1, len(data)):
                m.comparisons += 1
                if data[j] < data[min_index]:
                    min_index = j

            data[i], data[min_index] = data[min_index], data[i]
            m.swaps += 1
        pass
