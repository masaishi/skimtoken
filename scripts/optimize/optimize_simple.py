#!/usr/bin/env python3

import argparse
from pathlib import Path

import numpy as np
import toml
from scipy.optimize import minimize_scalar  # type: ignore[import-untyped]

from scripts.optimize.utils import calculate_metrics, filter_outliers, load_dataset, print_metrics
from skimtoken.simple import count


def extract_features(texts: list[str]) -> np.ndarray:
    """Extract character count features."""
    features: list[list[float]] = []
    for text in texts:
        char_count = count(text)
        features.append([char_count])
    return np.array(features)


def compute_error_rate(coefficient: float, char_counts: np.ndarray, y_true: np.ndarray) -> float:
    """Compute error rate for given coefficient."""
    y_pred = coefficient * char_counts
    relative_errors = np.abs(y_true - y_pred) / y_true
    error_rate = np.mean(relative_errors)  # Full error rate, not threshold
    return float(error_rate)


def optimize_parameters(
    dataset_path: Path, val_path: Path | None = None, max_samples: int | None = None
) -> dict[str, float]:
    """Optimize simple method parameters to minimize error rate."""
    print(f"Loading training dataset from {dataset_path}...")
    data = load_dataset(dataset_path, max_samples)
    texts = [item["text"] for item in data]

    print("Extracting training features...")
    X_train = extract_features(texts)
    char_counts_train = X_train[:, 0]

    print("Getting training token counts...")
    y_train_array = np.array([item["token_len"] for item in data])

    # Remove outliers (top/bottom 1%)
    char_counts_filtered, y_train_filtered = filter_outliers(
        char_counts_train.reshape(-1, 1), y_train_array, percentile=1.0
    )
    char_counts_filtered = char_counts_filtered.flatten()
    print(f"After filtering: {len(y_train_filtered)} samples")

    # Optimize for minimum error rate using fixed bounds
    print("Optimizing for minimum error rate...")
    result = minimize_scalar(
        compute_error_rate,
        args=(char_counts_filtered, y_train_filtered),
        bounds=(-2.0, 2.0),  # Wide bounds to explore all possibilities
        method="bounded",
        options={"xatol": 1e-8},
    )

    coefficient = float(result.x)  # type: ignore[attr-defined]

    # Calculate training metrics with optimized coefficient
    y_train_pred = coefficient * char_counts_filtered
    train_metrics = calculate_metrics(y_train_filtered, y_train_pred)

    print(f"\nOptimized coefficient: {coefficient:.6f}")
    print_metrics(train_metrics, "Training Metrics:")

    # Evaluate on validation set if provided
    if val_path and val_path.exists():
        print(f"\nLoading validation dataset from {val_path}...")
        val_data = load_dataset(val_path)
        val_texts = [item["text"] for item in val_data]

        print("Extracting validation features...")
        X_val = extract_features(val_texts)
        char_counts_val = X_val[:, 0]

        print("Getting validation token counts...")
        y_val_array = np.array([item["token_len"] for item in val_data])

        # Calculate validation predictions
        y_val_pred = coefficient * char_counts_val
        val_metrics = calculate_metrics(y_val_array, y_val_pred)
        print_metrics(val_metrics, "Validation Metrics:")

    return {"coefficient": float(coefficient)}


def main() -> None:
    parser = argparse.ArgumentParser(description="Optimize SimpleMethod parameters")
    parser.add_argument(
        "--dataset",
        type=Path,
        default=Path("data/cc100_samples/train.jsonl"),
        help="Path to training dataset JSONL file",
    )
    parser.add_argument(
        "--val-dataset",
        type=Path,
        default=Path("data/cc100_samples/val.jsonl"),
        help="Path to validation dataset JSONL file",
    )
    parser.add_argument(
        "--max-samples", type=int, default=None, help="Maximum number of samples to use"
    )
    parser.add_argument(
        "--output", type=Path, default=Path("params/simple.toml"), help="Output path for parameters"
    )

    args = parser.parse_args()

    # Optimize parameters
    params = optimize_parameters(args.dataset, args.val_dataset, args.max_samples)

    # Save to TOML file
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w") as f:
        toml.dump(params, f)

    print(f"\nParameters saved to {args.output}")


if __name__ == "__main__":
    main()
