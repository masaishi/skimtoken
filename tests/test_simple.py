import json
import math
import time
from pathlib import Path
from typing import Any

import pytest
from pytest import approx
from rich import box
from rich.console import Console
from rich.panel import Panel
from rich.table import Table

from skimtoken import estimate_tokens


class TestSkimtoken:
    """Simple test class using dataset with high error tolerance."""

    @pytest.fixture(scope="class")
    def dataset(self) -> list[dict[str, str | int | None]]:
        """Load test dataset from JSONL file."""
        dataset_path = Path(__file__).parent.parent / "data" / "test_dataset.jsonl"
        data: list[dict[str, str | int | None]] = []
        with open(dataset_path, "r", encoding="utf-8") as f:
            for line in f:
                if line.strip():  # Skip empty lines
                    data.append(json.loads(line))
        return data

    @pytest.fixture(scope="class")
    def console(self) -> Console:
        """Create console for rich output."""
        return Console()

    def get_error_color(self, error_rate: float) -> str:
        """Get color based on error percentage."""
        if error_rate < 10:
            return "green"
        elif error_rate < 30:
            return "yellow"
        elif error_rate < 50:
            return "orange1"
        else:
            return "red"

    def test_comprehensive_analysis(
        self, dataset: list[dict[str, str | int | None]], console: Console
    ) -> None:
        """Comprehensive test that processes data once and performs all analyses."""
        console.print("\n")
        console.print(
            Panel("[bold cyan]Comprehensive Token Estimation Analysis[/bold cyan]", expand=False)
        )

        # Start timing
        start_time = time.perf_counter()

        # Data structures for collecting all metrics in one pass
        results: list[dict[str, Any]] = []
        category_errors: dict[str, list[float]] = {}
        overall_squared_errors: list[float] = []
        total_chars = 0

        # Process all data ONCE
        for entry in dataset:
            text = entry["text"]
            actual = entry.get("token_len")
            category = entry.get("category", "unknown")

            if not isinstance(text, str):
                continue

            if not isinstance(actual, int) or actual == 0:
                continue

            # Now actual is definitely int
            actual_tokens: int = actual

            # Count characters
            total_chars += len(text)

            # Estimate tokens once
            estimated_result = estimate_tokens(text)
            # Ensure we have an integer
            estimated = int(estimated_result)
            # Convert to float immediately to avoid type issues
            estimated_float = float(estimated)
            actual_float = float(actual_tokens)
            error: float = estimated_float - actual_float
            squared_error: float = error * error
            percentage_error: float = abs(error) / actual_float * 100

            # Store result for display
            results.append(
                {
                    "text": text,
                    "category": str(category),
                    "actual": actual_tokens,
                    "estimated": estimated,
                    "error": error,
                    "squared_error": squared_error,
                    "percentage_error": percentage_error,
                }
            )

            # Collect for overall RMSE
            overall_squared_errors.append(squared_error)

            # Collect by category
            if isinstance(category, str):
                if category not in category_errors:
                    category_errors[category] = []
                category_errors[category].append(squared_error)

        # End timing
        end_time = time.perf_counter()
        execution_time = end_time - start_time

        # Display sample results table
        console.print("\n[bold]All Results:[/bold]")
        table = Table(show_header=True, header_style="bold cyan", box=box.ROUNDED)
        table.add_column("Text Sample", style="dim", width=40)
        table.add_column("Category", width=10)
        table.add_column("Actual", justify="right", width=8)
        table.add_column("Estimated", justify="right", width=10)
        table.add_column("Error", justify="right", width=10)

        # Show all results
        for result in results:
            text_str = str(result["text"])
            display_text = text_str[:37] + "..." if len(text_str) > 40 else text_str
            display_text = display_text.replace("\n", " ")
            error_color = self.get_error_color(float(result["percentage_error"]))

            table.add_row(
                display_text,
                str(result["category"]),
                str(result["actual"]),
                str(result["estimated"]),
                f"[{error_color}]{float(result['percentage_error']):.1f}%[/{error_color}]",
            )

        console.print(table)

        # Calculate overall RMSE
        overall_rmse = math.sqrt(sum(overall_squared_errors) / len(overall_squared_errors))

        # Display RMSE by category
        console.print("\n[bold]RMSE by Category:[/bold]")
        cat_table = Table(show_header=True, header_style="bold cyan", box=box.ROUNDED)
        cat_table.add_column("Category", style="bold")
        cat_table.add_column("RMSE", justify="right")
        cat_table.add_column("Samples", justify="right")
        cat_table.add_column("Status", justify="center")

        for category, squared_errors in sorted(category_errors.items()):
            rmse = math.sqrt(sum(squared_errors) / len(squared_errors))
            color = self.get_error_color(rmse * 2)

            assert rmse <= 50, f"{category} RMSE too high: {rmse:.2f}"
            status = "[green]✓ PASS[/green]" if rmse <= 50 else "[red]✗ FAIL[/red]"
            cat_table.add_row(
                category, f"[{color}]{rmse:.2f}[/{color}]", str(len(squared_errors)), status
            )

        console.print(cat_table)

        # Display summary statistics
        console.print("\n[bold]Summary Statistics:[/bold]")
        console.print(f"Overall RMSE: {overall_rmse:.2f} tokens")
        console.print(f"Total samples processed: {len(overall_squared_errors)}")
        console.print(f"Total characters: {total_chars:,}")
        console.print(f"Execution time: {execution_time:.3f} seconds")
        console.print(
            f"Processing speed: {len(overall_squared_errors) / execution_time:.0f} samples/second"
        )
        console.print(f"Character throughput: {total_chars / execution_time:,.0f} chars/second")

        # Performance assertions
        avg_us_per_char = execution_time / total_chars * 1_000_000
        console.print(f"Average per character: {avg_us_per_char:.3f}μs")

        # Overall assertions
        assert overall_rmse <= 100, f"Overall RMSE {overall_rmse:.2f} is outside acceptable range"
        assert avg_us_per_char <= 50.0, f"Too slow: {avg_us_per_char:.3f}μs per character"

    def test_edge_cases(self, dataset: list[dict[str, str | int | None]]) -> None:
        """Test edge cases like empty strings."""
        edge_cases = [entry for entry in dataset if entry.get("category", "unknown") == "edge"]

        for entry in edge_cases:
            text = entry["text"]
            if not isinstance(text, str):
                continue

            estimated = estimate_tokens(text)

            # Should not crash and return non-negative
            assert estimated >= 0, f"Negative tokens for edge case: {repr(text)}"

    def test_consistency(self, dataset: list[dict[str, str | int | None]]) -> None:
        """Test that same input gives same output."""
        # Get 5 sample texts
        sample_texts = [
            entry["text"]
            for entry in dataset[:5]
            if isinstance(entry["text"], str) and entry["text"]
        ]

        for text in sample_texts:
            # Run 3 times for each text
            results: list[int] = [estimate_tokens(text) for _ in range(3)]

            # All results should be identical
            assert len(set(results)) == 1, f"Inconsistent results for: {text[:30]}..."

            # Verify using approx with rel=0 (exact match)
            first_result: int = results[0]
            for result in results[1:]:
                assert result == approx(float(first_result), rel=0), (
                    f"Results not identical: {first_result} vs {result}"
                )
