#!/usr/bin/env python3

import argparse
from collections import defaultdict
from pathlib import Path
from typing import Any

import numpy as np
import numpy.typing as npt
import toml
from scipy.optimize import minimize_scalar  # type: ignore[import-untyped]

from scripts.optimize.utils import calculate_metrics, filter_outliers, load_dataset
from skimtoken import count_multilingual_simple


def extract_features(texts: list[str]) -> tuple[npt.NDArray[np.float64], list[str]]:
    """Extract character count and detect languages."""
    char_counts: list[float] = []
    languages: list[str] = []

    for text in texts:
        # count_multilingual_simple returns (char_count, language)
        char_count, language = count_multilingual_simple(text)
        char_counts.append(char_count)
        languages.append(language)

    return np.array(char_counts), languages


def compute_error_rate(coefficient: float, char_counts: np.ndarray, y_true: np.ndarray) -> float:
    """Compute error rate for given coefficient."""
    y_pred = coefficient * char_counts
    relative_errors = np.abs(y_true - y_pred) / y_true
    error_rate = np.mean(relative_errors)  # Full error rate, not threshold
    return float(error_rate)


def optimize_language_coefficient(
    char_counts: npt.NDArray[np.float64], y: npt.NDArray[np.float64]
) -> float:
    """Optimize coefficient for a specific language using minimize_scalar."""
    # Remove outliers (top/bottom 1%)
    char_counts_filtered, y_filtered = filter_outliers(
        char_counts.reshape(-1, 1), y, percentile=1.0
    )
    char_counts_filtered = char_counts_filtered.flatten()

    # Optimize for minimum error rate using fixed bounds
    result = minimize_scalar(
        compute_error_rate,
        args=(char_counts_filtered, y_filtered),
        bounds=(-2.0, 2.0),  # Wide bounds to explore all possibilities
        method="bounded",
        options={"xatol": 1e-8},
    )

    return float(result.x)  # type: ignore[attr-defined]


def optimize_parameters(
    dataset_path: Path,
    val_path: Path | None = None,
    max_samples: int | None = None,
    min_samples_per_lang: int = 10,
) -> dict[str, Any]:
    """Optimize language-specific coefficients to minimize error rate."""
    print(f"Loading training dataset from {dataset_path}...")
    data = load_dataset(dataset_path, max_samples)

    # Group data by pre-detected language
    print("Grouping samples by detected language...")
    lang_data_by_detected: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for item in data:
        detected_lang = item.get("detected_lang", "unknown")
        lang_data_by_detected[detected_lang].append(item)

    print(f"\nDetected languages in dataset: {list(lang_data_by_detected.keys())}")
    print(f"Total samples: {len(data)}")

    # Process each detected language separately
    language_params: dict[str, dict[str, float]] = {}

    for detected_lang, lang_samples in lang_data_by_detected.items():
        if detected_lang == "unknown":
            print(f"\nSkipping {detected_lang} language")
            continue

        if len(lang_samples) < min_samples_per_lang:
            print(
                f"\nSkipping {detected_lang}: only {len(lang_samples)} samples (min: {min_samples_per_lang})"
            )
            continue

        print(f"\nProcessing {detected_lang} ({len(lang_samples)} samples)...")

        # Extract texts for this language
        texts = [item["text"] for item in lang_samples]

        # Extract features (character counts)
        char_counts, _ = extract_features(texts)

        # Get true token counts
        y_lang = np.array([item["token_len"] for item in lang_samples])

        # Optimize coefficient for this specific language
        optimized_coef = optimize_language_coefficient(char_counts, y_lang)

        # Calculate metrics
        y_pred = optimized_coef * char_counts

        lang_metrics = calculate_metrics(y_lang, y_pred)
        print(f"  Optimized coefficient: {optimized_coef:.6f}")
        print(f"  R²: {lang_metrics['r2']:.4f}")
        print(f"  RMSE: {lang_metrics['rmse']:.2f}")
        print(f"  Error rate (full): {lang_metrics['error_rate']:.1f}%")
        print(f"  Error rate (>5%): {lang_metrics['error_rate_5pct']:.1f}%")

        language_params[detected_lang] = {
            "coefficient": float(optimized_coef),
        }

    # Fit default model on all data
    print("\nOptimizing default parameters (all languages)...")
    all_texts = [item["text"] for item in data]
    char_counts_all, _ = extract_features(all_texts)
    y_train_all = np.array([item["token_len"] for item in data])

    # Optimize default coefficient for minimum error rate
    optimized_default_coef = optimize_language_coefficient(char_counts_all, y_train_all)

    # Calculate metrics for default model
    y_pred = optimized_default_coef * char_counts_all

    default_metrics = calculate_metrics(y_train_all, y_pred)
    print(f"  Optimized coefficient: {optimized_default_coef:.6f}")
    print(f"  R²: {default_metrics['r2']:.4f}")
    print(f"  RMSE: {default_metrics['rmse']:.2f}")
    print(f"  Error rate (full): {default_metrics['error_rate']:.1f}%")
    print(f"  Error rate (>5%): {default_metrics['error_rate_5pct']:.1f}%")

    default_params = {
        "coefficient": float(optimized_default_coef),
    }

    # Evaluate on validation set if provided
    if val_path and val_path.exists():
        print("\n\nValidation Results:")
        print("=" * 60)

        print(f"Loading validation dataset from {val_path}...")
        val_data = load_dataset(val_path)

        # Group validation data by pre-detected language
        print("Grouping validation samples by detected language...")
        val_lang_data_by_detected: dict[str, list[dict[str, Any]]] = defaultdict(list)
        for item in val_data:
            detected_lang = item.get("detected_lang", "unknown")
            val_lang_data_by_detected[detected_lang].append(item)

        # Evaluate each detected language
        for detected_lang, val_lang_samples in val_lang_data_by_detected.items():
            if detected_lang == "unknown" or len(val_lang_samples) < 5:  # Skip if too few samples
                continue

            # Convert to 3-letter code
            lang_code = detected_lang.lower()[:3].title()

            # Use language-specific params if available
            if lang_code in language_params:
                val_texts = [item["text"] for item in val_lang_samples]
                char_counts_val, _ = extract_features(val_texts)
                y_val_lang = np.array([item["token_len"] for item in val_lang_samples])

                params = language_params[lang_code]

                # Make predictions
                y_pred = params["coefficient"] * char_counts_val

                # Calculate metrics
                val_lang_metrics = calculate_metrics(y_val_lang, y_pred)

                print(f"\nValidation metrics for {detected_lang} ({len(y_val_lang)} samples):")
                print(f"  Coefficient: {params['coefficient']:.6f}")
                print(f"  R²: {val_lang_metrics['r2']:.4f}")
                print(f"  RMSE: {val_lang_metrics['rmse']:.2f}")
                print(f"  Error rate (full): {val_lang_metrics['error_rate']:.1f}%")
                print(f"  Error rate (>5%): {val_lang_metrics['error_rate_5pct']:.1f}%")

        # Evaluate default model on all validation data
        all_val_texts = [item["text"] for item in val_data]
        char_counts_val_all, _ = extract_features(all_val_texts)
        y_val_all = np.array([item["token_len"] for item in val_data])

        y_pred = default_params["coefficient"] * char_counts_val_all

        val_all_metrics = calculate_metrics(y_val_all, y_pred)

        print("\nValidation metrics for default model (all languages):")
        print(f"  Coefficient: {default_params['coefficient']:.6f}")
        print(f"  R²: {val_all_metrics['r2']:.4f}")
        print(f"  RMSE: {val_all_metrics['rmse']:.2f}")
        print(f"  Error rate (full): {val_all_metrics['error_rate']:.1f}%")
        print(f"  Error rate (>5%): {val_all_metrics['error_rate_5pct']:.1f}%")

    return {"default_params": default_params, "language_params": language_params}


def main() -> None:
    parser = argparse.ArgumentParser(description="Optimize MultilingualSimpleMethod parameters")
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
        "--min-samples-per-lang",
        type=int,
        default=10,
        help="Minimum samples required to create language-specific parameters",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path("params/multilingual_simple.toml"),
        help="Output path for parameters",
    )

    args = parser.parse_args()

    # Optimize parameters
    params = optimize_parameters(
        args.dataset, args.val_dataset, args.max_samples, args.min_samples_per_lang
    )

    # Save to TOML file
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w") as f:
        toml.dump(params, f)

    print(f"\nParameters saved to {args.output}")


if __name__ == "__main__":
    main()
