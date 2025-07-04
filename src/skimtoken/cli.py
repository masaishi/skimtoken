import sys
import argparse

from skimtoken import estimate_tokens


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Calculate estimated token count for the given text.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Example:\n  skimtoken 'Hello, world!'\n  echo 'Some text' | skimtoken",
    )

    parser.add_argument(
        "text", nargs="*", help="Text to estimate tokens for (reads from stdin if not provided)"
    )

    args = parser.parse_args()

    # Get text from args or stdin
    if args.text:
        # Join all arguments as the text
        text = " ".join(args.text)
    elif sys.stdin.isatty():
        # No args and no piped input
        parser.error("No text provided")
    else:
        # Read from stdin
        try:
            text = sys.stdin.read().strip()
        except Exception as e:
            print(f"Error reading from stdin: {e}", file=sys.stderr)
            sys.exit(1)

    if not text:
        parser.error("No text provided")

    # Estimate tokens and print result
    token_count = estimate_tokens(text)
    print(token_count)


if __name__ == "__main__":
    main()
