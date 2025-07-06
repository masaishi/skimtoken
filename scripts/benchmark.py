import argparse
import json
import math
import random
import time
import tracemalloc
from collections import defaultdict
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.table import Table

# ==================== Helper Functions ====================


def parse_args() -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Benchmark skimtoken vs tiktoken performance",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "-n",
        "--num-items",
        type=int,
        default=None,
        help="Number of items to randomly sample from the dataset. If not specified, all items will be processed.",
    )
    parser.add_argument(
        "-d",
        "--dataset-path",
        type=Path,
        default=None,
        help="Path to the dataset JSONL file. If not specified, uses default test.jsonl",
    )
    parser.add_argument(
        "-m",
        "--method",
        type=str,
        choices=["simple", "basic", "multilingual", "multilingual_simple"],
        default="multilingual_simple",
        help="Type of skimtoken import to use",
    )
    return parser.parse_args()


def load_dataset(path: Path) -> list[dict[str, Any]]:
    """Load test dataset from JSONL file."""
    data: list[dict[str, Any]] = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip():
                data.append(json.loads(line))
    return data


def get_memory_usage() -> float:
    """Get current memory usage in MB using tracemalloc."""
    current, _ = tracemalloc.get_traced_memory()
    return current / 1024 / 1024


def format_ratio(ratio: float) -> str:
    """Format ratio with color based on value."""
    if ratio < 1.0:
        return f"[green]{ratio:.3f}x[/green]"
    else:
        return f"[red]{ratio:.3f}x[/red]"


def format_error_rate(error_rate: float) -> str:
    """Format error rate with color based on value."""
    if error_rate < 5.0:
        return f"[green]{error_rate:.2f}[/green]"
    elif error_rate < 10.0:
        return f"[yellow]{error_rate:.2f}[/yellow]"
    else:
        return f"[red]{error_rate:.2f}[/red]"


# ==================== Benchmarking Functions ====================


def benchmark_tiktoken_init() -> dict[str, Any]:
    """Benchmark tiktoken import and encoding load."""
    tracemalloc.start()
    start_time = time.perf_counter()

    # Import tiktoken
    import tiktoken

    # Load encoding (gpt-4o, gpt-4o-mini)
    enc = tiktoken.get_encoding("o200k_base")

    end_time = time.perf_counter()
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    return {
        "init_time": end_time - start_time,
        "init_memory": peak / 1024 / 1024,  # Use peak memory for init
        "encoder": enc,
    }


def benchmark_skimtoken_init(method: str) -> dict[str, Any]:
    """Benchmark skimtoken import."""
    tracemalloc.start()
    start_time = time.perf_counter()

    # Import skimtoken based on method
    if method == "simple":
        from skimtoken.simple import estimate_tokens
    elif method == "basic":
        from skimtoken.basic import estimate_tokens
    elif method == "multilingual":
        from skimtoken.multilingual import estimate_tokens
    elif method == "multilingual_simple":
        from skimtoken.multilingual_simple import estimate_tokens
    else:
        raise ValueError(f"Unknown skimtoken method: {method}")

    end_time = time.perf_counter()
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    return {
        "init_time": end_time - start_time,
        "init_memory": peak / 1024 / 1024,  # Use peak memory for init
        "estimate_func": estimate_tokens,
    }


def benchmark_tiktoken_execution(texts: list[str], enc: Any) -> dict[str, Any]:
    """Benchmark tiktoken execution."""
    tracemalloc.start()
    start_time = time.perf_counter()

    token_counts: list[int] = []
    for text in texts:
        tokens = len(enc.encode(text))
        token_counts.append(tokens)

    end_time = time.perf_counter()
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    return {
        "exec_time": end_time - start_time,
        "exec_memory": peak / 1024 / 1024,  # Use peak memory for execution
        "token_counts": token_counts,
    }


def benchmark_skimtoken_execution(texts: list[str], estimate_func: Any) -> dict[str, Any]:
    """Benchmark skimtoken execution."""
    tracemalloc.start()
    start_time = time.perf_counter()

    token_counts: list[int] = []
    for text in texts:
        tokens = estimate_func(text)
        token_counts.append(tokens)

    end_time = time.perf_counter()
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    return {
        "exec_time": end_time - start_time,
        "exec_memory": peak / 1024 / 1024,  # Use peak memory for execution
        "token_counts": token_counts,
    }


# ==================== Metrics Calculation ====================


def calculate_rmse(true_counts: list[int], estimated_counts: list[int]) -> float:
    """Calculate Root Mean Square Error between true and estimated token counts."""
    if len(true_counts) != len(estimated_counts):
        raise ValueError("Lists must have the same length")

    if not true_counts:
        return 0.0

    squared_errors = [(true - est) ** 2 for true, est in zip(true_counts, estimated_counts)]
    mean_squared_error = sum(squared_errors) / len(squared_errors)
    return math.sqrt(mean_squared_error)


def calculate_error_rate(true_counts: list[int], estimated_counts: list[int]) -> float:
    """Calculate mean absolute percentage error between true and estimated token counts."""
    if len(true_counts) != len(estimated_counts):
        raise ValueError("Lists must have the same length")

    percentage_errors = [
        abs(true - est) / true * 100 for true, est in zip(true_counts, estimated_counts) if true > 0
    ]
    return sum(percentage_errors) / len(percentage_errors) if percentage_errors else 0.0


def calculate_category_metrics(
    valid_entries: list[dict[str, Any]], tiktoken_counts: list[int], skimtoken_counts: list[int]
) -> dict[str, dict[str, float]]:
    """Calculate RMSE and error rate for each category."""
    category_data: dict[str, dict[str, list[int]]] = defaultdict(
        lambda: {"tiktoken": [], "skimtoken": []}
    )

    for i, entry in enumerate(valid_entries):
        category = entry.get("category", "unknown")
        category_data[category]["tiktoken"].append(tiktoken_counts[i])
        category_data[category]["skimtoken"].append(skimtoken_counts[i])

    category_metrics = {}
    for category, counts in category_data.items():
        if counts["tiktoken"] and counts["skimtoken"]:
            rmse = calculate_rmse(counts["tiktoken"], counts["skimtoken"])
            error_rate = calculate_error_rate(counts["tiktoken"], counts["skimtoken"])
            category_metrics[category] = {
                "rmse": rmse,
                "error_rate": error_rate,
                "count": len(counts["tiktoken"]),
            }

    return category_metrics


# ==================== Main Function ====================


def main() -> None:
    """Run benchmark comparing skimtoken and tiktoken."""
    args = parse_args()
    console = Console()

    # Load dataset
    if args.dataset_path:
        dataset_path = args.dataset_path
    else:
        dataset_path = Path(__file__).parent.parent / "data" / "cc100_samples" / "test.jsonl"

    console.print(f"[cyan]Loading dataset from {dataset_path}...[/cyan]")
    dataset = load_dataset(dataset_path)

    # Filter entries with valid text
    valid_entries = [
        entry for entry in dataset if isinstance(entry.get("text"), str) and entry["text"].strip()
    ]

    # Apply item limit if specified
    if args.num_items is not None:
        if args.num_items < len(valid_entries):
            # Randomly sample n items from the dataset
            valid_entries = random.sample(valid_entries, args.num_items)
            console.print(f"[yellow]Randomly sampled {args.num_items} items from dataset[/yellow]")
        else:
            console.print(
                f"[yellow]Using all {len(valid_entries)} items (requested {args.num_items})[/yellow]"
            )

    texts = [entry["text"] for entry in valid_entries]
    console.print(f"[green]Loaded {len(texts)} valid entries[/green]\n")

    if not texts:
        console.print("[red]Error: No valid entries found in the dataset.[/red]")
        console.print(
            "[yellow]Please ensure the dataset file contains entries with 'text' field.[/yellow]"
        )
        return

    # Benchmark initialization
    console.print("[bold cyan]Initialization Benchmark[/bold cyan]")
    console.print("[cyan]Benchmarking tiktoken initialization...[/cyan]")
    tiktoken_init = benchmark_tiktoken_init()

    console.print(f"[cyan]Benchmarking skimtoken.{args.method} initialization...[/cyan]")
    skimtoken_init = benchmark_skimtoken_init(args.method)

    # Benchmark execution
    console.print("\n[bold cyan]Execution Benchmark[/bold cyan]")
    console.print("[cyan]Benchmarking tiktoken execution...[/cyan]")
    tiktoken_exec = benchmark_tiktoken_execution(texts, tiktoken_init["encoder"])

    console.print(f"[cyan]Benchmarking skimtoken.{args.method} execution...[/cyan]")
    skimtoken_exec = benchmark_skimtoken_execution(texts, skimtoken_init["estimate_func"])

    # Display results
    console.print("\n[bold]Results:[/bold]")

    # Display dataset metadata
    total_chars = sum(len(text) for text in texts)
    console.print(f"[dim]Total Samples: {len(texts):,}[/dim]")
    console.print(f"[dim]Total Characters: {total_chars:,}[/dim]")

    # Calculate and display RMSE and error rate
    rmse = calculate_rmse(tiktoken_exec["token_counts"], skimtoken_exec["token_counts"])
    error_rate = calculate_error_rate(tiktoken_exec["token_counts"], skimtoken_exec["token_counts"])
    console.print(f"[dim]Mean RMSE: {rmse:.4f} tokens[/dim]")
    console.print(f"[dim]Mean Error Rate: {error_rate:.2f}%[/dim]\n")

    table = Table(show_header=True, header_style="bold cyan", show_lines=True)
    table.add_column("Metric", style="bold")
    table.add_column("tiktoken", justify="right")
    table.add_column("skimtoken", justify="right")
    table.add_column("Ratio", justify="right")

    # Init metrics
    init_time_ratio = skimtoken_init["init_time"] / tiktoken_init["init_time"]
    table.add_row(
        "Init Time",
        f"{tiktoken_init['init_time']:.6f} s",
        f"{skimtoken_init['init_time']:.6f} s",
        format_ratio(init_time_ratio),
    )

    init_memory_ratio = skimtoken_init["init_memory"] / tiktoken_init["init_memory"]

    table.add_row(
        "Init Memory",
        f"{tiktoken_init['init_memory']:.4f} MB",
        f"{skimtoken_init['init_memory']:.4f} MB",
        format_ratio(init_memory_ratio),
        end_section=True,
    )

    # Execution metrics
    exec_time_ratio = skimtoken_exec["exec_time"] / tiktoken_exec["exec_time"]
    table.add_row(
        "Exec Time",
        f"{tiktoken_exec['exec_time']:.6f} s",
        f"{skimtoken_exec['exec_time']:.6f} s",
        format_ratio(exec_time_ratio),
    )

    exec_memory_ratio = skimtoken_exec["exec_memory"] / tiktoken_exec["exec_memory"]

    table.add_row(
        "Exec Memory",
        f"{tiktoken_exec['exec_memory']:.4f} MB",
        f"{skimtoken_exec['exec_memory']:.4f} MB",
        format_ratio(exec_memory_ratio),
        end_section=True,
    )

    # Total metrics
    total_time_tiktoken = tiktoken_init["init_time"] + tiktoken_exec["exec_time"]
    total_time_skimtoken = skimtoken_init["init_time"] + skimtoken_exec["exec_time"]
    total_memory_tiktoken = tiktoken_init["init_memory"] + tiktoken_exec["exec_memory"]
    total_memory_skimtoken = skimtoken_init["init_memory"] + skimtoken_exec["exec_memory"]

    total_time_ratio = total_time_skimtoken / total_time_tiktoken
    total_memory_ratio = total_memory_skimtoken / total_memory_tiktoken

    table.add_row(
        "[bold]Total Time[/bold]",
        f"[bold]{total_time_tiktoken:.6f} s[/bold]",
        f"[bold]{total_time_skimtoken:.6f} s[/bold]",
        f"[bold]{format_ratio(total_time_ratio)}[/bold]",
    )

    table.add_row(
        "[bold]Total Memory[/bold]",
        f"[bold]{total_memory_tiktoken:.4f} MB[/bold]",
        f"[bold]{total_memory_skimtoken:.4f} MB[/bold]",
        f"[bold]{format_ratio(total_memory_ratio)}[/bold]",
    )

    console.print(table)

    # Calculate and display category metrics
    category_metrics = calculate_category_metrics(
        valid_entries, tiktoken_exec["token_counts"], skimtoken_exec["token_counts"]
    )

    if category_metrics:
        console.print("\n[bold cyan]Category-wise Metrics[/bold cyan]")
        category_table = Table(show_header=True, header_style="bold cyan", show_lines=True)
        category_table.add_column("Category", style="bold")
        category_table.add_column("Count", justify="right")
        category_table.add_column("RMSE", justify="right")
        category_table.add_column("Error Rate (%)", justify="right")

        # Sort categories by error rate (ascending - smallest error first)
        sorted_categories = sorted(category_metrics.items(), key=lambda x: x[1]["error_rate"])

        for category, metrics in sorted_categories:
            category_table.add_row(
                category,
                str(metrics["count"]),
                f"{metrics['rmse']:.4f}",
                format_error_rate(metrics["error_rate"]),
            )

        console.print(category_table)


if __name__ == "__main__":
    main()
