from src.algorithms.base import SortAlgorithm


class BubbleSort(SortAlgorithm):
    name = "BubbleSort"

    def sort(self, data):
        for i in range(1, len(data)):
            for j in range(0, len(data) - i):
                if data[j] > data[j + i]:
                    data[j], data[j + i] = data[j + i], data[j]
    