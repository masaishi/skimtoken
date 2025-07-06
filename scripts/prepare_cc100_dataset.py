#!/usr/bin/env python3
# type: ignore
"""
Download CC100 samples dataset, add token counts, and split into train/val/test sets.
"""

import argparse
import json
import random
from pathlib import Path

import tiktoken
from datasets import load_dataset  # type: ignore[import-untyped]


def main():
    parser = argparse.ArgumentParser(description="Prepare CC100 samples dataset")
    parser.add_argument("--seed", type=int, default=42, help="Random seed for train/val/test split")
    parser.add_argument("--train-ratio", type=float, default=0.8, help="Training set ratio")
    parser.add_argument("--val-ratio", type=float, default=0.1, help="Validation set ratio")
    parser.add_argument("--test-ratio", type=float, default=0.1, help="Test set ratio")
    args = parser.parse_args()

    # Create output directory
    data_dir = Path(__file__).parent.parent / "data" / "cc100_samples"
    data_dir.mkdir(parents=True, exist_ok=True)

    print("Downloading CC100 samples dataset...")

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

    for lang_code in language_codes:
        try:
            print(f"Loading {lang_code}...")
            dataset = load_dataset("xu-song/cc100-samples", lang_code, split="train")

            for sample in dataset:
                text = sample.get("text", "")
                if text.strip():  # Skip empty texts
                    # Calculate token count
                    token_count = len(encoder.encode(text))

                    # Create entry with desired format: category, token_len, text
                    entry = {"category": lang_code, "token_len": token_count, "text": text}
                    all_samples.append(entry)

            print(f"  Loaded {len(dataset)} samples from {lang_code}")
        except Exception as e:
            print(f"  Failed to load {lang_code}: {e}")

    print(f"\nTotal samples collected: {len(all_samples)}")

    # Shuffle and split dataset
    random.seed(args.seed)
    random.shuffle(all_samples)

    n = len(all_samples)
    train_size = int(n * args.train_ratio)
    val_size = int(n * args.val_ratio)

    train_data = all_samples[:train_size]
    val_data = all_samples[train_size : train_size + val_size]
    test_data = all_samples[train_size + val_size :]

    # Save splits
    for split_name, split_data in [("train", train_data), ("val", val_data), ("test", test_data)]:
        output_path = data_dir / f"{split_name}.jsonl"
        with open(output_path, "w", encoding="utf-8") as f:
            for entry in split_data:
                f.write(json.dumps(entry, ensure_ascii=False) + "\n")
        print(f"Saved {len(split_data)} samples to {output_path}")

    print("\nDataset preparation complete!")
    print(f"Train: {len(train_data)} samples ({args.train_ratio * 100:.0f}%)")
    print(f"Val: {len(val_data)} samples ({args.val_ratio * 100:.0f}%)")
    print(f"Test: {len(test_data)} samples ({args.test_ratio * 100:.0f}%)")


if __name__ == "__main__":
    main()
