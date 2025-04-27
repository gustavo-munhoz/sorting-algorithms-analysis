from src.algorithms.base import SortAlgorithm


class BubbleSort(SortAlgorithm):
    name = "BubbleSort"

    def sort(self, data, m):
        for i in range(len(data)):
            for j in range(0, len(data) - i - 1):
                m.comparisons += 1
                if data[j] > data[j + 1]:
                    data[j], data[j + 1] = data[j + 1], data[j]
                    m.swaps += 1
