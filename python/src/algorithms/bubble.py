from src.algorithms.base import SortAlgorithm


class BubbleSort(SortAlgorithm):
    name = "BubbleSort"

    def sort(self, data, m):
        for i in range(1, len(data)):
            for j in range(0, len(data) - i):
                m.comparisons += 1
                if data[j] > data[j + i]:
                    data[j], data[j + i] = data[j + i], data[j]
                    m.swaps += 1
