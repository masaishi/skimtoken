from skimtoken import estimate_tokens

examples = [
    "Hello world",
    "こんにちは世界",
    "Hello こんにちは 你好",
    "function hello() { return 42; }",
]

for text in examples:
    tokens = estimate_tokens(text)
    print(f"'{text}' → {tokens} tokens")
