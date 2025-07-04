#!/usr/bin/env python3
"""
Generate CC100 samples dataset with token counts and split into train/val/test sets.
This script orchestrates the entire data processing pipeline.
"""

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import tiktoken
from rich.console import Console


def run_command(cmd: list[str], console: Console) -> bool:
    """Run a command and return success status."""
    console.print(f"[cyan]Running: {' '.join(cmd)}[/cyan]")
    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True)
        if result.stdout:
            console.print(result.stdout)
        return True
    except subprocess.CalledProcessError as e:
        console.print(f"[red]Error: {e}[/red]")
        if e.stdout:
            console.print(f"[yellow]stdout: {e.stdout}[/yellow]")
        if e.stderr:
            console.print(f"[red]stderr: {e.stderr}[/red]")
        return False


def update_token_counts_for_file(file_path: Path, console: Console) -> bool:
    """Update token counts for a specific file."""
    console.print(f"\n[cyan]Updating token counts for {file_path}...[/cyan]")

    # Read the file
    data: list[dict[str, Any]] = []
    with open(file_path, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip():
                data.append(json.loads(line))

    encoder = tiktoken.get_encoding("o200k_base")

    updated_data: list[dict[str, Any]] = []
    for entry in data:
        text = entry.get("text", "")
        # Skip entries with empty text
        if not text.strip():
            continue
        token_len = len(encoder.encode(text))

        # Add token_len to the entry
        entry["token_len"] = token_len
        updated_data.append(entry)

    # Save the updated data back
    with open(file_path, "w", encoding="utf-8") as f:
        for entry in updated_data:
            f.write(json.dumps(entry, ensure_ascii=False) + "\n")

    console.print(f"[green]Updated {len(updated_data)} entries with token counts[/green]")
    return True


def split_dataset_custom(input_path: Path, output_dir: Path, console: Console) -> bool:
    """Split dataset and save to cc100_samples directory."""
    console.print("\n[cyan]Splitting dataset into train/val/test sets...[/cyan]")

    # Load the data
    data: list[dict[str, Any]] = []
    with open(input_path, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip():
                data.append(json.loads(line))

    # Import split function from split_dataset.py
    sys.path.insert(0, str(Path(__file__).parent))
    from split_dataset import split_dataset

    # Split the data
    train_data, val_data, test_data = split_dataset(
        data, train_ratio=0.8, val_ratio=0.1, test_ratio=0.1, seed=42
    )

    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)

    # Save the splits
    splits = [
        ("train.jsonl", train_data),
        ("val.jsonl", val_data),
        ("test.jsonl", test_data),
    ]

    for filename, split_data in splits:
        output_path = output_dir / filename
        valid_count = 0
        with open(output_path, "w", encoding="utf-8") as f:
            for entry in split_data:
                # Skip entries with empty text
                text = entry.get("text", "")
                if not text.strip():
                    continue
                # Ensure correct order: category, token_len, text
                ordered_entry = {
                    "category": entry.get("lang", "unknown"),
                    "token_len": entry.get("token_len", 0),
                    "text": text,
                }
                f.write(json.dumps(ordered_entry, ensure_ascii=False) + "\n")
                valid_count += 1
        console.print(f"[green]Saved {valid_count} samples to {output_path}[/green]")

    return True


def main() -> None:
    """Main function."""
    console = Console()
    console.print("[bold cyan]CC100 Samples Dataset Generator[/bold cyan]\n")

    # Set paths
    project_root = Path(__file__).parent.parent
    data_dir = project_root / "data"
    cc100_samples_dir = data_dir / "cc100_samples"
    raw_data_path = data_dir / "cc100_samples_raw.jsonl"

    # Step 1: Download CC100 samples
    console.print("[bold]Step 1: Downloading CC100 samples...[/bold]")
    if not run_command(["uv", "run", "scripts/download_cc100_samples.py"], console):
        console.print("[red]Failed to download CC100 samples[/red]")
        sys.exit(1)

    # Step 2: Update token counts
    console.print("\n[bold]Step 2: Updating token counts...[/bold]")
    if not update_token_counts_for_file(raw_data_path, console):
        console.print("[red]Failed to update token counts[/red]")
        sys.exit(1)

    # Step 3: Split dataset and save to cc100_samples directory
    console.print("\n[bold]Step 3: Splitting dataset...[/bold]")
    if not split_dataset_custom(raw_data_path, cc100_samples_dir, console):
        console.print("[red]Failed to split dataset[/red]")
        sys.exit(1)

    console.print("\n[green]✓ All steps completed successfully![/green]")
    console.print(f"[dim]Dataset saved to: {cc100_samples_dir}/[/dim]")
    console.print("[dim]Files created: train.jsonl, val.jsonl, test.jsonl[/dim]")

    # Step 4: Clean up raw data file
    console.print("\n[bold]Step 4: Cleaning up...[/bold]")
    if raw_data_path.exists():
        raw_data_path.unlink()
        console.print(f"[green]Deleted raw data file: {raw_data_path}[/green]")
    else:
        console.print(f"[yellow]Raw data file not found: {raw_data_path}[/yellow]")


if __name__ == "__main__":
    main()
