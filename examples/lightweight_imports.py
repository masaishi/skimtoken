#!/usr/bin/env python3
"""Compare skimtoken estimates with tiktoken ground truth."""

from skimtoken.simple import estimate_tokens as simple_estimate
from skimtoken.basic import estimate_tokens as basic_estimate
from skimtoken.multilingual import estimate_tokens as multilingual_estimate

# Try to import tiktoken for ground truth
try:
    import tiktoken

    enc = tiktoken.get_encoding("cl100k_base")
except ImportError:
    enc = None
    print("Note: Install tiktoken to see accuracy comparisons\n")

# Test texts
texts = [
    "Hello, world!",
    "The quick brown fox jumps over the lazy dog.",
    "def factorial(n): return 1 if n <= 1 else n * factorial(n-1)",
    "これは日本語のテキストです。",
]

# Compare estimation methods
print("Skimtoken Estimation Methods Comparison:")
print("=" * 50)

for text in texts:
    print(f"\nText: '{text}'")

    # Get estimates
    simple = simple_estimate(text)
    basic = basic_estimate(text)
    multilingual = multilingual_estimate(text)

    # Show results
    if enc:
        actual = len(enc.encode(text))
        print(f"  Actual (tiktoken): {actual} tokens")
        print(f"  Simple:    {simple:2d} tokens ({simple / actual * 100:5.1f}% accuracy)")
        print(f"  Basic:     {basic:2d} tokens ({basic / actual * 100:5.1f}% accuracy)")
        print(
            f"  Multilingual: {multilingual:2d} tokens ({multilingual / actual * 100:5.1f}% accuracy)"
        )
    else:
        print(f"  Simple:    {simple} tokens")
        print(f"  Basic:     {basic} tokens")
        print(f"  Multilingual: {multilingual} tokens")
