#!/usr/bin/env python3

import argparse
from collections import defaultdict
from pathlib import Path
from typing import Any

import numpy as np
import numpy.typing as npt
import toml
from sklearn.linear_model import Ridge  # type: ignore[import-untyped]

from scripts.optimize.utils import calculate_metrics, filter_outliers, load_dataset
from skimtoken.multilingual import count

# Mapping from whatlang detected languages to CC100 language codes
# Based on ISO 639-3 codes from whatlang documentation
WHATLANG_TO_CC100 = {
    "Esperanto": "eo",
    "English": "en",
    "Russian": "ru",
    "Mandarin": "zh-Hans",  # Simplified Chinese
    "Spanish": "es",
    "Portuguese": "pt",
    "Italian": "it",
    "Bengali": "bn",
    "French": "fr",
    "German": "de",
    "Ukrainian": "uk",
    "Georgian": "ka",
    "Arabic": "ar",
    "Hindi": "hi",
    "Japanese": "ja",
    "Hebrew": "he",
    "Yiddish": "yi",
    "Polish": "pl",
    "Amharic": "am",
    "Javanese": "jv",
    "Korean": "ko",
    "Bokmal": "no",  # Norwegian
    "Danish": "da",
    "Swedish": "sv",
    "Finnish": "fi",
    "Turkish": "tr",
    "Dutch": "nl",
    "Hungarian": "hu",
    "Czech": "cs",
    "Greek": "el",
    "Bulgarian": "bg",
    "Belarusian": "be",
    "Marathi": "mr",
    "Kannada": "kn",
    "Romanian": "ro",
    "Slovene": "sl",
    "Croatian": "hr",
    "Serbian": "sr",
    "Macedonian": "mk",
    "Lithuanian": "lt",
    "Latvian": "lv",
    "Estonian": "et",
    "Tamil": "ta",
    "Vietnamese": "vi",
    "Urdu": "ur",
    "Thai": "th",
    "Gujarati": "gu",
    "Uzbek": "uz",
    "Punjabi": "pa",
    "Azerbaijani": "az",
    "Indonesian": "id",
    "Telugu": "te",
    "Persian": "fa",  # Note: whatlang uses "pes" but CC100 uses "fa"
    "Malayalam": "ml",
    "Oriya": "or",
    "Burmese": "my",
    "Nepali": "ne",
    "Sinhalese": "si",
    "Khmer": "km",
    "Turkmen": "tk",
    "Akan": "ak",
    "Zulu": "zu",
    "Shona": "sn",
    "Afrikaans": "af",
    "Latin": "la",
    "Slovak": "sk",
    "Catalan": "ca",
    "Tagalog": "tl",
    "Armenian": "hy",
}


def extract_features(texts: list[str]) -> tuple[npt.NDArray[np.float64], list[str]]:
    """Extract features and detect languages."""
    features: list[list[float]] = []
    languages: list[str] = []

    for text in texts:
        # count returns (char_count, word_count, avg_word_length, space_count, language)
        char_count, word_count, avg_word_length, space_count, language = count(text)

        features.append([char_count, word_count, avg_word_length, space_count])
        languages.append(language)

    return np.array(features), languages


def optimize_language_params(
    X: npt.NDArray[np.float64], y: npt.NDArray[np.float64]
) -> npt.NDArray[np.float64]:
    """Optimize parameters for a specific language using Ridge regression."""
    # Remove outliers (top/bottom 1%)
    X_filtered, y_filtered = filter_outliers(X, y, percentile=1.0)

    # Fit Ridge regression model
    model = Ridge(alpha=1.0, fit_intercept=True, max_iter=10000)
    model.fit(X_filtered, y_filtered)  # type: ignore[arg-type]

    # Return coefficients: [token_count_coef, char_coef, word_coef, avg_word_length_coef, space_coef, intercept]
    return np.append(model.coef_, model.intercept_)  # type: ignore[attr-defined]


def optimize_parameters(
    dataset_path: Path,
    val_path: Path | None = None,
    max_samples: int | None = None,
    min_samples_per_lang: int = 10,
) -> dict[str, Any]:
    """Optimize language-specific parameters to minimize error rate."""
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

        # Extract features (we already know the language, but extract for consistency)
        X_lang, _ = extract_features(texts)

        # Get true token counts
        y_lang = np.array([item["token_len"] for item in lang_samples])

        # Convert to numpy arrays
        X_lang = np.array(X_lang)

        # Optimize parameters for this specific language
        optimized_params = optimize_language_params(X_lang, y_lang)
        char_coef, word_coef, avg_word_length_coef, space_coef, intercept = optimized_params

        # Calculate metrics
        y_pred = (
            char_coef * X_lang[:, 0]
            + word_coef * X_lang[:, 1]
            + avg_word_length_coef * X_lang[:, 2]
            + space_coef * X_lang[:, 3]
            + intercept
        )

        lang_metrics = calculate_metrics(y_lang, y_pred)
        print(f"  R²: {lang_metrics['r2']:.4f}")
        print(f"  RMSE: {lang_metrics['rmse']:.2f}")
        print(f"  Error rate (full): {lang_metrics['error_rate']:.1f}%")
        print(f"  Error rate (>5%): {lang_metrics['error_rate_5pct']:.1f}%")

        # Store parameters for this detected language
        language_params[detected_lang] = {
            "char_coef": float(char_coef),
            "word_coef": float(word_coef),
            "avg_word_length_coef": float(avg_word_length_coef),
            "space_coef": float(space_coef),
            "intercept": float(intercept),
        }

    # Fit default model on all data
    print("\nOptimizing default parameters (all languages)...")
    all_texts = [item["text"] for item in data]
    X_train_all, _ = extract_features(all_texts)
    y_train_all = np.array([item["token_len"] for item in data])

    X_train_all = np.array(X_train_all)

    # Optimize default parameters for minimum error rate
    optimized_default = optimize_language_params(X_train_all, y_train_all)
    char_coef, word_coef, avg_word_length_coef, space_coef, intercept = optimized_default

    # Calculate metrics for default model
    y_pred = (
        char_coef * X_train_all[:, 0]
        + word_coef * X_train_all[:, 1]
        + avg_word_length_coef * X_train_all[:, 2]
        + space_coef * X_train_all[:, 3]
        + intercept
    )

    default_metrics = calculate_metrics(y_train_all, y_pred)
    print(f"  R²: {default_metrics['r2']:.4f}")
    print(f"  RMSE: {default_metrics['rmse']:.2f}")
    print(f"  Error rate (full): {default_metrics['error_rate']:.1f}%")
    print(f"  Error rate (>5%): {default_metrics['error_rate_5pct']:.1f}%")

    default_params = {
        "char_coef": float(char_coef),
        "word_coef": float(word_coef),
        "avg_word_length_coef": float(avg_word_length_coef),
        "space_coef": float(space_coef),
        "intercept": float(intercept),
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

            # Use language-specific params if available
            if detected_lang in language_params:
                val_texts = [item["text"] for item in val_lang_samples]
                X_val_lang, _ = extract_features(val_texts)
                y_val_lang = np.array([item["token_len"] for item in val_lang_samples])

                X_val_lang = np.array(X_val_lang)

                params = language_params[detected_lang]

                # Make predictions
                y_pred = (
                    params["char_coef"] * X_val_lang[:, 0]
                    + params["word_coef"] * X_val_lang[:, 1]
                    + params["avg_word_length_coef"] * X_val_lang[:, 2]
                    + params["space_coef"] * X_val_lang[:, 3]
                    + params["intercept"]
                )

                # Calculate metrics
                val_lang_metrics = calculate_metrics(y_val_lang, y_pred)

                print(f"\nValidation metrics for {detected_lang} ({len(y_val_lang)} samples):")
                print(f"  R²: {val_lang_metrics['r2']:.4f}")
                print(f"  RMSE: {val_lang_metrics['rmse']:.2f}")
                print(f"  Error rate (full): {val_lang_metrics['error_rate']:.1f}%")
                print(f"  Error rate (>5%): {val_lang_metrics['error_rate_5pct']:.1f}%")

        # Evaluate default model on all validation data
        all_val_texts = [item["text"] for item in val_data]
        X_val_all, _ = extract_features(all_val_texts)
        y_val_all = np.array([item["token_len"] for item in val_data])

        X_val_all = np.array(X_val_all)

        y_pred = (
            default_params["char_coef"] * X_val_all[:, 0]
            + default_params["word_coef"] * X_val_all[:, 1]
            + default_params["avg_word_length_coef"] * X_val_all[:, 2]
            + default_params["space_coef"] * X_val_all[:, 3]
            + default_params["intercept"]
        )

        val_all_metrics = calculate_metrics(y_val_all, y_pred)

        print("\nValidation metrics for default model (all languages):")
        print(f"  R²: {val_all_metrics['r2']:.4f}")
        print(f"  RMSE: {val_all_metrics['rmse']:.2f}")
        print(f"  Error rate (full): {val_all_metrics['error_rate']:.1f}%")
        print(f"  Error rate (>5%): {val_all_metrics['error_rate_5pct']:.1f}%")

    return {"default_params": default_params, "language_params": language_params}


def main() -> None:
    parser = argparse.ArgumentParser(description="Optimize MultilingualMethod parameters")
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
        default=Path("params/multilingual.toml"),
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
