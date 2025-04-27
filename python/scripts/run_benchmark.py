#!/usr/bin/env python
from __future__ import annotations
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
import json
import argparse

from rich.progress import (
    Progress, SpinnerColumn, BarColumn, TextColumn,
    TimeElapsedColumn, TaskID
)

from src.algorithms.base import SortAlgorithm
from src.runner import run_one
from src.algorithms import *


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Benchmark of sorting algorithms")
    p.add_argument(
        "-s", "--sizes",
        type=int, nargs="+", default=[1_000, 100_000, 1_000_000],
        help="list of sizes n"
    )
    p.add_argument(
        "-o", "--output",
        default="py_results.json",
        help="JSON file for output"
    )
    return p.parse_args()


if __name__ == "__main__":
    args = parse_args()

    algorithms: list[SortAlgorithm] = [
        InsertionSort(),
        SelectionSort(),
        BubbleSort(),
        MergeSort(),
        QuickSort()
    ]
    results: list[dict] = []

    progress = Progress(
        SpinnerColumn(style="yellow"),
        TextColumn("{task.description:<20}"),
        BarColumn(bar_width=30),
        TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
        TextColumn("{task.completed}/{task.total}"),
        TimeElapsedColumn(),
    )

    task_id: dict[str, TaskID] = {
        algorithm.name: progress.add_task(f"{algorithm.name}", total=len(args.sizes))
        for algorithm in algorithms
    }

    with progress:
        with ThreadPoolExecutor(max_workers=len(algorithms)) as pool:
            futures = []
            for algorithm in algorithms:
                for n in args.sizes:
                    futures.append(
                        pool.submit(run_one, algorithm, n)
                    )

            for future in as_completed(futures):
                rec = future.result()
                results.append(rec)
                progress.advance(task_id[rec["algorithm"]])

    Path(args.output).write_text(json.dumps(results, indent=4))
    print(f"Saved {len(results)} results to {args.output}")
