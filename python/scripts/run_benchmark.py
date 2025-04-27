#!/usr/bin/env python
from __future__ import annotations
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
import json
import argparse
import time

from rich.progress import (
    Progress, ProgressColumn, SpinnerColumn, BarColumn, TextColumn,
    TimeElapsedColumn, TaskID
)
from rich.text import Text

from src.algorithms.base import SortAlgorithm
from src.runner import run_one
from src.algorithms import *


class NColumn(ProgressColumn):
    def render(self, task) -> Text:
        n = task.fields.get("n")
        if task.finished or n in (None, ""):
            return Text("")                 # linha final → nada
        return Text(f"(n = {n})")


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


def worker(
    algorithm: SortAlgorithm,
    sizes: list[int],
    tid: TaskID,
    progress: Progress,
    results: list[dict],
) -> None:
    for n in sizes:
        progress.update(tid, n=n)

        start = time.perf_counter()
        rec = run_one(algorithm, n)
        rec["time_s"] = time.perf_counter() - start
        results.append(rec)

        progress.advance(tid)

    progress.update(tid, n=None)
    progress.update(tid, description=f"{algorithm.name} ✔ finished")


if __name__ == "__main__":
    args = parse_args()

    algorithms: list[SortAlgorithm] = [
        InsertionSort(),
        SelectionSort(),
        BubbleSort(),
        MergeSort(),
        QuickSort()
    ]

    progress = Progress(
        SpinnerColumn(style="yellow"),
        TextColumn("{task.description:<20}"),
        BarColumn(bar_width=30),
        TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
        TextColumn("{task.completed}/{task.total}"),
        NColumn(),
        TimeElapsedColumn(),
    )

    task_id: dict[str, TaskID] = {
        algorithm.name: progress.add_task(
            algorithm.name, total=len(args.sizes), n="-"
        )
        for algorithm in algorithms
    }

    results: list[dict] = []

    with progress, ThreadPoolExecutor(max_workers=len(algorithms)) as pool:
        for algorithm in algorithms:
            pool.submit(
                worker,
                algorithm,
                args.sizes,
                task_id[algorithm.name],
                progress,
                results
            )

    Path(args.output).write_text(json.dumps(results, indent=4))
    print(f"Saved {len(results)} results to {args.output}")
