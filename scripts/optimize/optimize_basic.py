#!/usr/bin/env python3

import argparse
from pathlib import Path

import numpy as np
import numpy.typing as npt
import toml
from sklearn.linear_model import Ridge  # type: ignore[import-untyped]

from scripts.optimize.utils import (
    calculate_metrics,
    filter_outliers,
    load_dataset_with_texts,
    print_metrics,
)
from skimtoken.basic import count


def extract_features(texts: list[str]) -> npt.NDArray[np.float64]:
    """Extract features: char_count, word_count, avg_word_length, space_count."""
    features: list[list[float]] = []
    for text in texts:
        char_count, word_count, avg_word_length, space_count = count(text)
        features.append([char_count, word_count, avg_word_length, space_count])

    return np.array(features)


def optimize_parameters(
    dataset_path: Path, val_path: Path | None = None, max_samples: int | None = None
) -> dict[str, float]:
    """Optimize basic method parameters using least squares."""
    # Load training data
    print(f"Loading training data from {dataset_path}...")
    texts, token_lens = load_dataset_with_texts(dataset_path, max_samples)
    print(f"Loaded {len(texts)} text samples")

    # Extract features and token counts
    print("Extracting features...")
    X_train = extract_features(texts)
    print("Using pre-calculated token counts...")
    y_train = np.array(token_lens)

    # Remove outliers (top/bottom 1%)
    X_train, y_train = filter_outliers(X_train, y_train, percentile=1.0)
    print(f"After filtering: {len(y_train)} samples")

    # Fit linear regression model (least squares)
    print("Fitting linear regression model using least squares...")
    model = Ridge(alpha=1.0, fit_intercept=True, max_iter=10000)
    model.fit(X_train, y_train)  # type: ignore[arg-type]

    # Extract coefficients
    char_coef, word_coef, avg_word_length_coef, space_coef = model.coef_  # type: ignore[attr-defined]
    intercept = model.intercept_  # type: ignore[attr-defined]

    # Calculate training metrics
    y_train_pred = np.maximum(model.predict(X_train), 0)  # type: ignore[attr-defined] # Clip negative predictions
    train_metrics = calculate_metrics(y_train, y_train_pred)

    # Print results
    print("\nOptimized Parameters:")
    print(f"  char_coef: {char_coef:.6f}")
    print(f"  word_coef: {word_coef:.6f}")
    print(f"  avg_word_length_coef: {avg_word_length_coef:.6f}")
    print(f"  space_coef: {space_coef:.6f}")
    print(f"  intercept: {intercept:.6f}")

    print_metrics(train_metrics, "Training Metrics:")

    # Evaluate on validation set if provided
    if val_path and val_path.exists():
        print(f"\nLoading validation data from {val_path}...")
        val_texts, val_token_lens = load_dataset_with_texts(val_path)
        print(f"Loaded {len(val_texts)} validation samples")

        X_val = extract_features(val_texts)
        y_val = np.array(val_token_lens)

        y_val_pred = np.maximum(model.predict(X_val), 0)  # type: ignore[attr-defined]
        val_metrics = calculate_metrics(y_val, y_val_pred)
        print_metrics(val_metrics, "Validation Metrics:")

    # Return parameters
    return {
        "char_coef": float(char_coef),  # type: ignore[arg-type]
        "word_coef": float(word_coef),  # type: ignore[arg-type]
        "avg_word_length_coef": float(avg_word_length_coef),  # type: ignore[arg-type]
        "space_coef": float(space_coef),  # type: ignore[arg-type]
        "intercept": float(intercept),  # type: ignore[arg-type]
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Optimize BasicMethod parameters")
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
        "--output", type=Path, default=Path("params/basic.toml"), help="Output path for parameters"
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
