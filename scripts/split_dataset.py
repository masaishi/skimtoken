#!/usr/bin/env python3
"""
Split CC100 samples dataset into train/val/test sets using random sampling.
Each run will produce different random splits unless a seed is specified.
"""

import json
import random
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.progress import BarColumn, Progress, TextColumn, TimeRemainingColumn


def load_raw_dataset(path: Path) -> list[dict[str, Any]]:
    """Load raw dataset from JSONL file."""
    data: list[dict[str, Any]] = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip():
                data.append(json.loads(line))
    return data


def split_dataset(
    data: list[dict[str, Any]],
    train_ratio: float = 0.8,
    val_ratio: float = 0.1,
    test_ratio: float = 0.1,
    seed: int | None = None,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    """Split dataset into train/val/test sets using random sampling."""
    if abs(train_ratio + val_ratio + test_ratio - 1.0) > 0.001:
        raise ValueError("Ratios must sum to 1.0")

    # Set random seed for reproducibility if provided
    if seed is not None:
        random.seed(seed)

    # Create indices for all data points
    indices = list(range(len(data)))
    random.shuffle(indices)

    # Calculate split sizes
    n = len(data)
    train_size = int(n * train_ratio)
    val_size = int(n * val_ratio)

    # Randomly sample indices for each split
    train_indices = set(indices[:train_size])
    val_indices = set(indices[train_size : train_size + val_size])
    test_indices = set(indices[train_size + val_size :])

    # Create splits by randomly selecting data points
    train_data = [data[i] for i in sorted(train_indices)]
    val_data = [data[i] for i in sorted(val_indices)]
    test_data = [data[i] for i in sorted(test_indices)]

    return train_data, val_data, test_data


def save_dataset(data: list[dict[str, Any]], path: Path, name: str) -> None:
    """Save dataset to JSONL file."""
    console = Console()

    with open(path, "w", encoding="utf-8") as f:
        with Progress(
            TextColumn(f"[cyan]Saving {name} dataset...[/cyan]"),
            BarColumn(),
            TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            TimeRemainingColumn(),
            console=console,
        ) as progress:
            task = progress.add_task("Processing", total=len(data))

            for sample in data:
                # Keep the original format from raw data
                f.write(json.dumps(sample, ensure_ascii=False) + "\n")
                progress.update(task, advance=1)

    console.print(f"[green]Saved {len(data):,} samples to {path}[/green]")


def main() -> None:
    """Main function."""
    console = Console()

    # Paths
    project_root = Path(__file__).parent.parent
    data_dir = project_root / "data"
    raw_data_path = data_dir / "cc100_samples_raw.jsonl"

    console.print("[bold cyan]Dataset Splitter (Random Split)[/bold cyan]\n")

    # Check if raw data exists
    if not raw_data_path.exists():
        console.print(f"[red]Raw dataset not found at {raw_data_path}[/red]")
        console.print("[yellow]Please run download_cc100_samples.py first[/yellow]")
        return

    # Load raw dataset
    console.print(f"[cyan]Loading raw dataset from {raw_data_path}...[/cyan]")
    raw_data = load_raw_dataset(raw_data_path)
    console.print(f"[green]Loaded {len(raw_data):,} samples[/green]\n")

    # Split dataset
    console.print("[cyan]Splitting dataset using random sampling...[/cyan]")
    train_data, val_data, test_data = split_dataset(
        raw_data, train_ratio=0.8, val_ratio=0.1, test_ratio=0.1, seed=None
    )

    console.print(f"[dim]Train: {len(train_data):,} samples (80%)[/dim]")
    console.print(f"[dim]Val: {len(val_data):,} samples (10%)[/dim]")
    console.print(f"[dim]Test: {len(test_data):,} samples (10%)[/dim]\n")

    # Save datasets
    console.print("[cyan]Saving split datasets...[/cyan]\n")

    save_dataset(train_data, data_dir / "cc100_train.jsonl", "train")
    save_dataset(val_data, data_dir / "cc100_val.jsonl", "val")
    save_dataset(test_data, data_dir / "cc100_test.jsonl", "test")

    # Print language statistics for each split
    console.print("\n[bold cyan]Language Distribution:[/bold cyan]")

    for split_name, split_data in [("Train", train_data), ("Val", val_data), ("Test", test_data)]:
        lang_counts: dict[str, int] = {}
        for sample in split_data:
            lang = sample.get("lang", "unknown")
            lang_counts[lang] = lang_counts.get(lang, 0) + 1

        console.print(f"\n[cyan]{split_name} set:[/cyan]")
        # Show top 5 languages
        sorted_langs = sorted(lang_counts.items(), key=lambda x: x[1], reverse=True)[:5]
        for lang, count in sorted_langs:
            percentage = (count / len(split_data)) * 100
            console.print(f"  {lang}: {count:,} samples ({percentage:.1f}%)")
        if len(lang_counts) > 5:
            console.print(f"  ... and {len(lang_counts) - 5} more languages")

    console.print("\n[green]Dataset splitting complete![/green]")
    console.print("[dim]Using truly random sampling (no fixed seed) for each run.[/dim]")


if __name__ == "__main__":
    main()
