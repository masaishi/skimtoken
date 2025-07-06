#!/usr/bin/env python3

import argparse
import json
from pathlib import Path
from typing import Any

import tiktoken


def update_token_counts(
    encode_model: str = "o200k_base", dataset_path: Path | str | None = None
) -> None:
    """Update token_len field in existing JSONL dataset."""
    if dataset_path is None:
        dataset_path = Path(__file__).parent.parent / "data" / "test_dataset.jsonl"
    elif isinstance(dataset_path, str):
        dataset_path = Path(dataset_path)

    if not dataset_path.exists():
        print(f"Dataset not found at {dataset_path}")
        return

    # Read all lines
    with open(dataset_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    # Load encoder
    encoder = tiktoken.get_encoding(encode_model)

    updated_data: list[dict[str, Any]] = []
    updated_count = 0

    for line in lines:
        if not line.strip():
            continue

        entry = json.loads(line)
        text = entry.get("text", "")
        old_token_len = entry.get("token_len")
        new_token_len = len(encoder.encode(text)) if text else 0

        updated_entry = {
            "category": entry.get("category", entry.get("lang", "unknown")),
            "token_len": new_token_len,
            "text": entry["text"],
        }

        if old_token_len != new_token_len:
            updated_count += 1
            print(f"Updated: '{text[:30]}...' from {old_token_len} to {new_token_len}")

        updated_data.append(updated_entry)

    # Save updated dataset
    with open(dataset_path, "w", encoding="utf-8") as f:
        for entry in updated_data:
            f.write(json.dumps(entry, ensure_ascii=False) + "\n")

    print(f"\nUpdated {updated_count} entries out of {len(updated_data)} total.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Update token counts in JSONL dataset")
    parser.add_argument(
        "--encode-model",
        type=str,
        default="o200k_base",
        help="Tiktoken encoding model (default: o200k_base)",
    )
    parser.add_argument(
        "--dataset-path",
        type=str,
        default=None,
        help="Path to JSONL dataset (default: data/test_dataset.jsonl)",
    )
    args = parser.parse_args()

    update_token_counts(encode_model=args.encode_model, dataset_path=args.dataset_path)
