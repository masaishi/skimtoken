#!/usr/bin/env python3
# type: ignore
"""
Download CC100 samples dataset, add token counts, and split into train/val/test sets.
"""

import argparse
import json
import random
from collections import Counter
from pathlib import Path

import tiktoken
from datasets import load_dataset  # type: ignore[import-untyped]
from rich.console import Console
from rich.progress import (
    Progress,
    SpinnerColumn,
    TextColumn,
    BarColumn,
    TaskProgressColumn,
    TimeRemainingColumn,
)
from rich.table import Table
from skimtoken import detect_language


def main():
    parser = argparse.ArgumentParser(description="Prepare CC100 samples dataset")
    parser.add_argument("--seed", type=int, default=42, help="Random seed for train/val/test split")
    parser.add_argument("--train-ratio", type=float, default=0.8, help="Training set ratio")
    parser.add_argument("--val-ratio", type=float, default=0.1, help="Validation set ratio")
    parser.add_argument("--test-ratio", type=float, default=0.1, help="Test set ratio")
    args = parser.parse_args()

    # Initialize console
    console = Console()

    # Create output directory
    data_dir = Path(__file__).parent.parent / "data" / "cc100_samples"
    data_dir.mkdir(parents=True, exist_ok=True)

    console.print("[bold blue]Downloading CC100 samples dataset...[/bold blue]")

    # Get all available language codes
    language_codes = [
        "am",
        "ar",
        "as",
        "az",
        "be",
        "bg",
        "bn",
        "bn_rom",
        "br",
        "bs",
        "ca",
        "cs",
        "cy",
        "da",
        "de",
        "el",
        "en",
        "eo",
        "es",
        "et",
        "eu",
        "fa",
        "ff",
        "fi",
        "fr",
        "fy",
        "ga",
        "gd",
        "gl",
        "gn",
        "gu",
        "ha",
        "he",
        "hi",
        "hi_rom",
        "hr",
        "ht",
        "hu",
        "hy",
        "id",
        "ig",
        "is",
        "it",
        "ja",
        "jv",
        "ka",
        "kk",
        "km",
        "kn",
        "ko",
        "ku",
        "ky",
        "la",
        "lg",
        "li",
        "ln",
        "lo",
        "lt",
        "lv",
        "mg",
        "mk",
        "ml",
        "mn",
        "mr",
        "ms",
        "my",
        "my_zaw",
        "ne",
        "nl",
        "no",
        "ns",
        "om",
        "or",
        "pa",
        "pl",
        "ps",
        "pt",
        "qu",
        "rm",
        "ro",
        "ru",
        "sa",
        "si",
        "sc",
        "sd",
        "sk",
        "sl",
        "so",
        "sq",
        "sr",
        "ss",
        "su",
        "sv",
        "sw",
        "ta",
        "ta_rom",
        "te",
        "te_rom",
        "th",
        "tl",
        "tn",
        "tr",
        "ug",
        "uk",
        "ur",
        "ur_rom",
        "uz",
        "vi",
        "wo",
        "xh",
        "yi",
        "yo",
        "zh-Hans",
        "zh-Hant",
        "zu",
    ]

    # Initialize tokenizer
    encoder = tiktoken.get_encoding("o200k_base")

    # Collect all samples
    all_samples = []
    language_stats = []

    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        BarColumn(),
        TaskProgressColumn(),
        TimeRemainingColumn(),
        console=console,
    ) as progress:
        # Main progress for all languages
        main_task = progress.add_task("[cyan]Processing languages...", total=len(language_codes))

        for idx, lang_code in enumerate(language_codes, 1):
            try:
                dataset = load_dataset("xu-song/cc100-samples", lang_code, split="train")
                dataset_size = len(dataset)

                # Create task for this language
                lang_task = progress.add_task(f"[yellow]{lang_code}", total=dataset_size)

                samples_processed = 0
                lang_samples = []
                mismatches = 0

                for i, sample in enumerate(dataset):
                    text = sample.get("text", "")
                    if text.strip():  # Skip empty texts
                        # Calculate token count
                        token_count = len(encoder.encode(text))

                        # Detect language
                        detected_lang = detect_language(text)

                        # Check for language mismatch
                        if detected_lang != lang_code:
                            mismatches += 1

                        # Create entry with desired format: category, detected_lang, token_len, text
                        entry = {
                            "category": lang_code,
                            "detected_lang": detected_lang,
                            "token_len": token_count,
                            "text": text,
                        }
                        lang_samples.append(entry)
                        samples_processed += 1

                    # Update progress
                    progress.update(lang_task, completed=i + 1)

                all_samples.extend(lang_samples)
                mismatch_rate = (
                    (mismatches / samples_processed * 100) if samples_processed > 0 else 0
                )

                # Store stats for summary
                language_stats.append(
                    {
                        "lang": lang_code,
                        "samples": samples_processed,
                        "mismatches": mismatches,
                        "rate": mismatch_rate,
                    }
                )

                # Remove completed language task
                progress.remove_task(lang_task)

            except Exception as e:
                console.print(f"[red]✗ Failed to load {lang_code}: {e}[/red]")
                language_stats.append(
                    {"lang": lang_code, "samples": 0, "mismatches": 0, "rate": 0.0}
                )

            # Update main progress
            progress.update(main_task, completed=idx)

    console.print(f"\n[bold green]Total samples collected: {len(all_samples)}[/bold green]")

    # Shuffle and split dataset
    random.seed(args.seed)
    random.shuffle(all_samples)

    n = len(all_samples)
    train_size = int(n * args.train_ratio)
    val_size = int(n * args.val_ratio)

    train_data = all_samples[:train_size]
    val_data = all_samples[train_size : train_size + val_size]
    test_data = all_samples[train_size + val_size :]

    # Save splits with progress
    console.print("\n[bold]Saving dataset splits...[/bold]")
    with Progress(
        TextColumn("[progress.description]{task.description}"),
        BarColumn(),
        TaskProgressColumn(),
        console=console,
    ) as progress:
        save_task = progress.add_task("[cyan]Saving files...", total=3)

        for split_name, split_data in [
            ("train", train_data),
            ("val", val_data),
            ("test", test_data),
        ]:
            output_path = data_dir / f"{split_name}.jsonl"
            with open(output_path, "w", encoding="utf-8") as f:
                for entry in split_data:
                    f.write(json.dumps(entry, ensure_ascii=False) + "\n")
            progress.update(save_task, advance=1)

    # Create summary table for dataset splits
    split_table = Table(title="Dataset Splits", show_header=True, header_style="bold magenta")
    split_table.add_column("Split", style="cyan", width=10)
    split_table.add_column("Samples", justify="right", style="green")
    split_table.add_column("Percentage", justify="right", style="yellow")

    split_table.add_row("Train", str(len(train_data)), f"{args.train_ratio * 100:.0f}%")
    split_table.add_row("Val", str(len(val_data)), f"{args.val_ratio * 100:.0f}%")
    split_table.add_row("Test", str(len(test_data)), f"{args.test_ratio * 100:.0f}%")

    console.print("\n")
    console.print(split_table)

    # Show language processing summary
    if language_stats:
        lang_table = Table(
            title="Language Processing Summary", show_header=True, header_style="bold magenta"
        )
        lang_table.add_column("Language", style="cyan", width=12)
        lang_table.add_column("Samples", justify="right", style="green")
        lang_table.add_column("Mismatches", justify="right", style="yellow")
        lang_table.add_column("Mismatch %", justify="right", style="red")

        # Show top languages with most mismatches
        sorted_stats = sorted(language_stats, key=lambda x: x["rate"], reverse=True)
        for stat in sorted_stats[:20]:  # Show top 20
            if stat["samples"] > 0:
                lang_table.add_row(
                    stat["lang"],
                    str(stat["samples"]),
                    str(stat["mismatches"]),
                    f"{stat['rate']:.1f}%",
                )

        console.print("\n")
        console.print(lang_table)

    # Show overall statistics
    detected_langs = Counter(sample["detected_lang"] for sample in all_samples)
    mismatch_count = sum(
        1 for sample in all_samples if sample["category"] != sample["detected_lang"]
    )
    mismatch_rate = (mismatch_count / len(all_samples) * 100) if all_samples else 0

    stats_table = Table(title="Overall Statistics", show_header=False, box=None)
    stats_table.add_column("Metric", style="cyan")
    stats_table.add_column("Value", style="green")

    stats_table.add_row("Total samples", str(len(all_samples)))
    stats_table.add_row("Unique detected languages", str(len(detected_langs)))
    stats_table.add_row("Total mismatches", f"{mismatch_count} ({mismatch_rate:.1f}%)")

    console.print("\n")
    console.print(stats_table)

    console.print("\n[bold green]✅ Dataset preparation complete![/bold green]")


if __name__ == "__main__":
    main()
