import json
import math
import time
from pathlib import Path
from typing import Any

import psutil
from rich.console import Console
from rich.table import Table


def get_memory_usage() -> float:
    """Get current process memory usage in MB."""
    process = psutil.Process()
    return process.memory_info().rss / 1024 / 1024


def load_dataset(path: Path) -> list[dict[str, Any]]:
    """Load test dataset from JSONL file."""
    data: list[dict[str, Any]] = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip():
                data.append(json.loads(line))
    return data


def benchmark_tiktoken_init() -> dict[str, Any]:
    """Benchmark tiktoken import and encoding load."""
    initial_memory = get_memory_usage()
    start_time = time.perf_counter()

    # Import tiktoken
    import tiktoken

    # Load encoding (gpt-4o, gpt-4o-mini)
    enc = tiktoken.get_encoding("o200k_base")

    end_time = time.perf_counter()
    final_memory = get_memory_usage()

    return {
        "init_time": end_time - start_time,
        "init_memory": final_memory - initial_memory,
        "encoder": enc,
    }


def benchmark_skimtoken_init() -> dict[str, Any]:
    """Benchmark skimtoken import."""
    initial_memory = get_memory_usage()
    start_time = time.perf_counter()

    # Import skimtoken
    from skimtoken import estimate_tokens

    end_time = time.perf_counter()
    final_memory = get_memory_usage()

    return {
        "init_time": end_time - start_time,
        "init_memory": final_memory - initial_memory,
        "estimate_func": estimate_tokens,
    }


def benchmark_tiktoken_execution(texts: list[str], enc: Any) -> dict[str, Any]:
    """Benchmark tiktoken execution."""
    initial_memory = get_memory_usage()
    start_time = time.perf_counter()

    token_counts: list[int] = []
    for text in texts:
        tokens = len(enc.encode(text))
        token_counts.append(tokens)

    end_time = time.perf_counter()
    final_memory = get_memory_usage()

    return {
        "exec_time": end_time - start_time,
        "exec_memory": final_memory - initial_memory,
        "token_counts": token_counts,
    }


def benchmark_skimtoken_execution(texts: list[str], estimate_func: Any) -> dict[str, Any]:
    """Benchmark skimtoken execution."""
    initial_memory = get_memory_usage()
    start_time = time.perf_counter()

    token_counts: list[int] = []
    for text in texts:
        tokens = estimate_func(text)
        token_counts.append(tokens)

    end_time = time.perf_counter()
    final_memory = get_memory_usage()

    return {
        "exec_time": end_time - start_time,
        "exec_memory": final_memory - initial_memory,
        "token_counts": token_counts,
    }


def format_ratio(ratio: float) -> str:
    """Format ratio with color based on value."""
    if ratio < 1.0:
        return f"[green]{ratio:.3f}x[/green]"
    else:
        return f"[red]{ratio:.3f}x[/red]"


def calculate_rmse(true_counts: list[int], estimated_counts: list[int]) -> float:
    """Calculate Root Mean Square Error between true and estimated token counts."""
    if len(true_counts) != len(estimated_counts):
        raise ValueError("Lists must have the same length")

    squared_errors = [(true - est) ** 2 for true, est in zip(true_counts, estimated_counts)]
    mean_squared_error = sum(squared_errors) / len(squared_errors)
    return math.sqrt(mean_squared_error)


def main() -> None:
    """Run benchmark comparing skimtoken and tiktoken."""
    console = Console()

    # Load dataset
    dataset_path = Path(__file__).parent.parent / "data" / "cc100_samples" / "test.jsonl"
    console.print(f"[cyan]Loading dataset from {dataset_path}...[/cyan]")
    dataset = load_dataset(dataset_path)

    # Filter entries with valid token counts
    valid_entries = [
        entry
        for entry in dataset
        if isinstance(entry.get("token_len"), int) and entry["token_len"] > 0
    ]

    texts = [entry["text"] for entry in valid_entries]
    console.print(f"[green]Loaded {len(texts)} valid entries[/green]\n")

    # Benchmark initialization
    console.print("[bold cyan]Initialization Benchmark[/bold cyan]")
    console.print("[cyan]Benchmarking tiktoken initialization...[/cyan]")
    tiktoken_init = benchmark_tiktoken_init()

    console.print("[cyan]Benchmarking skimtoken initialization...[/cyan]")
    skimtoken_init = benchmark_skimtoken_init()

    # Benchmark execution
    console.print("\n[bold cyan]Execution Benchmark[/bold cyan]")
    console.print("[cyan]Benchmarking tiktoken execution...[/cyan]")
    tiktoken_exec = benchmark_tiktoken_execution(texts, tiktoken_init["encoder"])

    console.print("[cyan]Benchmarking skimtoken execution...[/cyan]")
    skimtoken_exec = benchmark_skimtoken_execution(texts, skimtoken_init["estimate_func"])

    # Display results
    console.print("\n[bold]Results:[/bold]")

    # Display dataset metadata
    total_chars = sum(len(text) for text in texts)
    console.print(f"[dim]Total Samples: {len(texts):,}[/dim]")
    console.print(f"[dim]Total Characters: {total_chars:,}[/dim]")

    # Calculate and display RMSE
    rmse = calculate_rmse(tiktoken_exec["token_counts"], skimtoken_exec["token_counts"])
    console.print(f"[dim]Mean RMSE: {rmse:.4f} tokens[/dim]\n")

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


if __name__ == "__main__":
    main()
