from skimtoken import estimate_tokens

# Basic usage
text = "Hello, world!"
tokens = estimate_tokens(text, method="simple")
print(f"'{text}' → {tokens} tokens")

# Different languages
examples = [
    "The quick brown fox jumps over the lazy dog.",
    "こんにちは世界",
    "你好世界",
    "def factorial(n): return 1 if n <= 1 else n * factorial(n-1)",
]

print("\nMultilingual examples:")
for text in examples:
    tokens = estimate_tokens(text, method="simple")
    print(f"'{text[:30]}...' → {tokens} tokens")
