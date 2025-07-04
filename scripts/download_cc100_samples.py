#!/usr/bin/env python3
"""
Download xu-song/cc100-samples dataset from Hugging Face.
This dataset contains multilingual text samples from CC100.
"""

import json
from pathlib import Path
from typing import Any, cast

from datasets import Dataset, load_dataset  # type: ignore[import-untyped]
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn


def download_cc100_samples(output_dir: Path, languages: list[str] | None = None) -> None:
    """Download CC100 samples dataset and save to JSONL format.

    Args:
        output_dir: Directory to save the dataset
        languages: List of language codes to download. If None, downloads all languages.
    """
    console = Console()

    # Available languages
    all_languages = [
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

    # Use specified languages or all languages
    languages_to_download = languages if languages else all_languages

    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)
    raw_data_path = output_dir / "cc100_samples_raw.jsonl"

    console.print(
        f"[cyan]Downloading xu-song/cc100-samples dataset for {len(languages_to_download)} language(s)...[/cyan]"
    )

    all_samples: list[dict[str, Any]] = []
    sample_id = 0

    try:
        # Load dataset for each language
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        ) as progress:
            for lang in languages_to_download:
                task = progress.add_task(f"Loading {lang}...", total=None)

                try:
                    # Load the dataset for this language
                    dataset = load_dataset("xu-song/cc100-samples", lang, split="train")

                    # Ensure we have a Dataset (not IterableDataset)
                    if not isinstance(dataset, Dataset):
                        raise ValueError(f"Expected Dataset but got IterableDataset for {lang}")

                    # Collect samples
                    for sample in dataset:
                        sample_dict = cast(dict[str, Any], sample)
                        # Skip samples with empty text
                        if not sample_dict.get("text", "").strip():
                            continue
                        sample_dict["_language_config"] = lang
                        sample_dict["_sample_id"] = sample_id
                        all_samples.append(sample_dict)
                        sample_id += 1

                    progress.update(task, completed=True)
                    console.print(f"[green]✓[/green] {lang}: {len(dataset):,} samples")

                except Exception as e:
                    console.print(f"[yellow]⚠[/yellow] Failed to load {lang}: {e}")
                    progress.update(task, completed=True)

        total_samples = len(all_samples)
        console.print("\n[green]All datasets loaded successfully![/green]")
        console.print(f"[dim]Total samples: {total_samples:,}[/dim]")

        # Save to JSONL format
        console.print(f"\n[cyan]Saving dataset to {raw_data_path}...[/cyan]")

        with open(raw_data_path, "w", encoding="utf-8") as f:
            for sample_dict in all_samples:
                # Convert to our format
                entry = {
                    "id": sample_dict["_sample_id"],
                    "text": str(sample_dict.get("text", "")),
                    "lang": str(
                        sample_dict.get("lang", sample_dict.get("_language_config", "unknown"))
                    ),
                    "source": "cc100",
                    "config": sample_dict.get("_language_config", "unknown"),
                }

                # Add any additional metadata from the original dataset
                for key, value in sample_dict.items():
                    if key not in ["text", "lang", "_language_config", "_sample_id"]:
                        entry[f"original_{key}"] = value

                f.write(json.dumps(entry, ensure_ascii=False) + "\n")

        console.print(f"[green]Dataset saved to {raw_data_path}[/green]")

        # Print dataset statistics
        console.print("\n[bold cyan]Dataset Statistics:[/bold cyan]")

        # Count languages
        lang_counts: dict[str, int] = {}
        for sample_dict in all_samples:
            lang = str(sample_dict.get("lang", sample_dict.get("_language_config", "unknown")))
            lang_counts[lang] = lang_counts.get(lang, 0) + 1

        # Sort by count
        sorted_langs = sorted(lang_counts.items(), key=lambda x: x[1], reverse=True)

        console.print("\n[dim]Language distribution:[/dim]")
        for lang, count in sorted_langs[:10]:  # Show top 10
            console.print(f"  {lang}: {count:,} samples")

        if len(sorted_langs) > 10:
            console.print(f"  ... and {len(sorted_langs) - 10} more languages")

    except Exception as e:
        console.print(f"[red]Error downloading dataset: {e}[/red]")
        raise


def main() -> None:
    """Main function."""
    console = Console()

    # Set output directory
    project_root = Path(__file__).parent.parent
    data_dir = project_root / "data"

    console.print("[bold cyan]CC100 Samples Dataset Downloader[/bold cyan]\n")

    # Download dataset
    download_cc100_samples(data_dir)

    console.print("\n[green]Download complete![/green]")
    console.print(f"[dim]Data saved to: {data_dir / 'cc100_samples_raw.jsonl'}[/dim]")


if __name__ == "__main__":
    main()
