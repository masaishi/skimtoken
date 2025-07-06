#!/usr/bin/env python3
"""Run all optimization scripts to find the best parameters for each method."""

import argparse
import subprocess
import sys
from pathlib import Path


def run_optimization(script_path: Path, dataset_path: Path, max_samples: int | None = None) -> bool:
    """Run a single optimization script."""
    print(f"\n{'=' * 60}")
    print(f"Running {script_path.name}")
    print(f"{'=' * 60}")

    cmd = [
        sys.executable,
        "-m",
        f"scripts.optimize.{script_path.stem}",
        "--dataset",
        str(dataset_path),
    ]
    if max_samples:
        cmd.extend(["--max-samples", str(max_samples)])

    try:
        # Run without capturing output to show prints in real-time
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running {script_path.name}: {e}", file=sys.stderr)
        return False

    return True


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Run all optimization scripts for skimtoken methods"
    )
    parser.add_argument(
        "--dataset",
        type=Path,
        default=Path("data/cc100_samples/train.jsonl"),
        help="Path to training dataset JSONL file",
    )
    parser.add_argument(
        "--max-samples",
        type=int,
        default=None,
        help="Maximum number of samples to use for optimization",
    )
    parser.add_argument(
        "--methods",
        nargs="+",
        choices=["simple", "basic", "multilingual", "multilingual_simple", "all"],
        default=["all"],
        help="Methods to optimize (default: all)",
    )

    args = parser.parse_args()

    # Check if dataset exists
    if not args.dataset.exists():
        print(f"Error: Dataset file {args.dataset} does not exist.", file=sys.stderr)
        print("\nPlease run one of the following commands first:")
        print("  uv run python scripts/prepare_cc100_dataset.py  # Download CC100 samples")
        sys.exit(1)

    # Define optimization scripts
    optimize_dir = Path("scripts/optimize")
    scripts = {
        "simple": optimize_dir / "optimize_simple.py",
        "basic": optimize_dir / "optimize_basic.py",
        "multilingual": optimize_dir / "optimize_multilingual.py",
        "multilingual_simple": optimize_dir / "optimize_multilingual_simple.py",
    }

    # Determine which scripts to run
    if "all" in args.methods:
        methods_to_run = list(scripts.keys())
    else:
        methods_to_run = args.methods

    # Run optimizations
    print(f"Optimizing methods: {', '.join(methods_to_run)}")
    print(f"Using dataset: {args.dataset}")
    if args.max_samples:
        print(f"Max samples: {args.max_samples}")

    success_count = 0
    for method in methods_to_run:
        if method in scripts:
            if run_optimization(scripts[method], args.dataset, args.max_samples):
                success_count += 1
        else:
            print(f"Warning: Unknown method '{method}'", file=sys.stderr)

    # Summary
    print(f"\n{'=' * 60}")
    print("OPTIMIZATION COMPLETE")
    print(f"{'=' * 60}")
    print(f"Successfully optimized {success_count}/{len(methods_to_run)} methods")

    if success_count == len(methods_to_run):
        print("\nAll optimizations completed successfully!")
        print("\nOptimized parameters have been saved to:")
        for method in methods_to_run:
            print(f"  - params/{method}.toml")

        print("\nTo use the optimized parameters, simply run:")
        print("  import skimtoken")
        print("  tokens = skimtoken.estimate_tokens('your text', 'simple')")
        print("\nThe library will automatically load the optimized parameters.")
    else:
        print("\nSome optimizations failed. Please check the error messages above.")
        sys.exit(1)


if __name__ == "__main__":
    main()
