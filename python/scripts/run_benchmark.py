#!/usr/bin/env python
from src.runner import run
from src.algorithms import *
import json
import argparse


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run sorting benchmarks")
    parser.add_argument("-o", "--output", type=str, default="python_results.json")
    args = parser.parse_args()

    results = run([InsertionSort, SelectionSort, BubbleSort, MergeSort, QuickSort])

    with open(args.output, "w") as f:
        # noinspection PyTypeChecker
        json.dump(results, f, indent=4)

    print(f"Saved {len(results)} results to {args.output}")
