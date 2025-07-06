"""skimtoken - Lightweight token count estimation library.

Lightweight import patterns:
    from skimtoken.simple import estimate_tokens    # Only load simple method
    from skimtoken.basic import estimate_tokens     # Only load basic method
    from skimtoken.language import estimate_tokens  # Only load language method

Full import:
    import skimtoken
    skimtoken.estimate_tokens(text)
"""

# Import from the Rust module
from ._skimtoken_core import (
    estimate_tokens,
    estimate_tokens_simple,
    estimate_tokens_basic,
    estimate_tokens_multilingual,
    estimate_tokens_multilingual_simple,
    count_simple,
    count_basic,
    count_multilingual,
    count_multilingual_simple,
    detect_language,
)

__version__ = "0.2.0"

__all__ = [
    "estimate_tokens",
    "estimate_tokens_simple",
    "estimate_tokens_basic",
    "estimate_tokens_multilingual",
    "estimate_tokens_multilingual_simple",
    "count_simple",
    "count_basic",
    "count_multilingual",
    "count_multilingual_simple",
    "detect_language",
    "simple",
    "basic",
    "multilingual",
    "multilingual_simple",
]

# Submodules for lightweight imports
from . import simple, basic, multilingual, multilingual_simple


# CLI entry point
def main():
    """CLI entry point for skimtoken."""
    import argparse
    import sys

    parser = argparse.ArgumentParser(
        description="Estimate token count for text",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "text",
        nargs="?",
        help="Text to estimate tokens for. If not provided, reads from stdin.",
    )

    parser.add_argument(
        "-f",
        "--file",
        type=str,
        help="Read text from file instead of command line",
    )

    args = parser.parse_args()

    # Get text from appropriate source
    if args.file:
        try:
            with open(args.file, "r", encoding="utf-8") as f:
                text = f.read()
        except Exception as e:
            print(f"Error reading file: {e}", file=sys.stderr)
            sys.exit(1)
    elif args.text:
        text = args.text
    else:
        text = sys.stdin.read()

    if not text:
        print("No text provided", file=sys.stderr)
        sys.exit(1)

    # Estimate tokens
    try:
        token_count = estimate_tokens(text)
        print(f"{token_count}")
    except Exception as e:
        print(f"Error estimating tokens: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
