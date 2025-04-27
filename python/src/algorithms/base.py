from __future__ import annotations
from abc import ABC, abstractmethod
from typing import MutableSequence

from src.metrics import Metrics


class SortAlgorithm(ABC):
    """Interface for sorting algorithms."""

    name: str = "AbstractSort"

    @abstractmethod
    def sort(self, data: MutableSequence[int], m: Metrics) -> None:
        """Sorts the given data using this algorithm."""
        ...

    def __call__(self, data: MutableSequence[int]) -> None:
        self.sort(data)
