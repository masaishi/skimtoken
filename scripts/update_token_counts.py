import json
from pathlib import Path
from typing import Any, Dict, List

import tiktoken


def update_token_counts() -> None:
    """Update token_len field in existing JSONL dataset."""
    dataset_path = Path(__file__).parent.parent / "data" / "test_dataset.jsonl"

    if not dataset_path.exists():
        print(f"Dataset not found at {dataset_path}")
        return

    # Read all lines
    with open(dataset_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    # Load gpt-4o and gpt-4o-mini encoder
    encoder = tiktoken.get_encoding("o200k_base")

    updated_data: List[Dict[str, Any]] = []
    updated_count = 0

    for line in lines:
        if not line.strip():
            continue

        entry = json.loads(line)
        text = entry.get("text", "")
        old_token_len = entry.get("token_len")
        new_token_len = len(encoder.encode(text)) if text else 0

        updated_entry = {
            "category": entry.get("category", "unknown"),
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
    update_token_counts()
