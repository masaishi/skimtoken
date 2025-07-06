#!/usr/bin/env python3
"""Common utilities for optimization scripts."""

import json
from pathlib import Path
from typing import Any

import numpy as np


def load_dataset(path: Path, max_samples: int | None = None) -> list[dict[str, Any]]:
    """Load dataset from JSONL file."""
    data: list[dict[str, Any]] = []
    with open(path, "r", encoding="utf-8") as f:
        for i, line in enumerate(f):
            if max_samples and i >= max_samples:
                break
            if line.strip():
                data.append(json.loads(line))
    return data


def load_dataset_with_texts(
    path: Path, max_samples: int | None = None
) -> tuple[list[str], list[int]]:
    """Load text samples and token counts from JSONL file."""
    texts: list[str] = []
    token_lens: list[int] = []
    with open(path, "r", encoding="utf-8") as f:
        for i, line in enumerate(f):
            if max_samples and i >= max_samples:
                break
            if line.strip():
                data = json.loads(line)
                if isinstance(data.get("text"), str) and data["text"].strip():
                    texts.append(data["text"])
                    token_lens.append(data["token_len"])
    return texts, token_lens


def calculate_metrics(y_true: np.ndarray, y_pred: np.ndarray) -> dict[str, float]:
    """Calculate RMSE, R², and error rates."""
    # RMSE
    rmse = float(np.sqrt(np.mean((y_true - y_pred) ** 2)))

    # R²
    ss_res = np.sum((y_true - y_pred) ** 2)
    ss_tot = np.sum((y_true - np.mean(y_true)) ** 2)
    r2 = float(1 - (ss_res / ss_tot)) if ss_tot > 0 else 0.0

    # Error rates
    relative_errors = np.abs(y_true - y_pred) / y_true
    error_rate = float(np.mean(relative_errors) * 100)
    error_rate_5pct = float(np.mean(relative_errors > 0.05) * 100)

    return {
        "rmse": rmse,
        "r2": r2,
        "error_rate": error_rate,
        "error_rate_5pct": error_rate_5pct,
    }


def print_metrics(metrics: dict[str, float], prefix: str = "") -> None:
    """Print metrics in a standardized format."""
    if prefix:
        print(f"\n{prefix}")
    print(f"  R²: {metrics['r2']:.4f}")
    print(f"  RMSE: {metrics['rmse']:.2f}")
    print(f"  Error rate: {metrics['error_rate']:.1f}%")
    print(f"  Error rate (>5%): {metrics['error_rate_5pct']:.1f}%")


def filter_outliers(
    X: np.ndarray, y: np.ndarray, percentile: float = 1.0
) -> tuple[np.ndarray, np.ndarray]:
    """Remove outliers from top/bottom percentiles."""
    q_high = np.percentile(y, 100 - percentile)
    q_low = np.percentile(y, percentile)
    valid_mask = (y >= q_low) & (y <= q_high) & (y > 0)
    return X[valid_mask], y[valid_mask]
