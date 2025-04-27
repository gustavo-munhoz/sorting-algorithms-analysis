from src.algorithms.base import SortAlgorithm


class MergeSort(SortAlgorithm):
    name = "MergeSort"

    def sort(self, data):
        if len(data) < 2:
            return data

        middle = len(data) // 2
        left = data[:middle]
        right = data[middle:]

        self.sort(left)
        self.sort(right)

        data[:] = self.merge(left, right)



    def merge(self, data1, data2):
        sorted_data = []
        index1, index2 = 0, 0

        while index1 < len(data1) and index2 < len(data2):
            if data1[index1] <= data2[index2]:
                sorted_data.append(data1[index1])
                index1 += 1
            else:
                sorted_data.append(data2[index2])
                index2 += 1

        if index1< len(data1):
            sorted_data.extend(data1[index1:])
        else:
            sorted_data.extend(data2[index2:])

        return sorted_data