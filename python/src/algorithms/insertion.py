from .base import SortAlgorithm


class InsertionSort(SortAlgorithm):
    name = "InsertionSort"

    def sort(self, data):
        for i in range(1, len(data)):
            key = data[i]
            j = i
            while j >= 0 and key < data[j - 1]:
                data[j] = data[j - 1]
                j -= 1
            data[j] = key
        pass