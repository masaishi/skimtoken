"""
Basic usage examples for skimtoken library.
This file demonstrates simple usage patterns.
"""

from skimtoken import (
    estimate_tokens,
    estimate_tokens_basic,
    estimate_tokens_simple,
    estimate_tokens_multilingual,
)


def test_simplest_usage():
    """The simplest way to use skimtoken."""
    text = "Hello, world!"
    token_count = estimate_tokens(text)

    assert token_count > 0
    assert isinstance(token_count, int)
    print(f"'{text}' has approximately {token_count} tokens")


def test_basic_english_text():
    """Basic English text estimation."""
    # Simple sentence
    text = "The quick brown fox jumps over the lazy dog."
    tokens = estimate_tokens(text)

    # Should be roughly 1 token per word (9 words)
    assert 7 <= tokens <= 20  # More lenient range
    print(f"English sentence: {tokens} tokens")


def test_counting_tokens_in_paragraph():
    """Count tokens in a paragraph."""
    paragraph = """
    Python is a high-level programming language.
    It is known for its simplicity and readability.
    Many developers love Python for data science and web development.
    """

    tokens = estimate_tokens(paragraph)
    assert tokens > 20  # Multiple sentences
    print(f"Paragraph has {tokens} tokens")


def test_different_methods():
    """Compare different estimation methods."""
    text = "Compare different estimation methods for the same text."

    # Try all methods
    default = estimate_tokens(text)
    basic = estimate_tokens_basic(text)
    simple = estimate_tokens_simple(text)
    multilingual = estimate_tokens_multilingual(text)

    print(f"Text: '{text}'")
    print(f"  Default method: {default} tokens")
    print(f"  Basic method: {basic} tokens")
    print(f"  Simple method: {simple} tokens")
    print(f"  Multilingual method: {multilingual} tokens")

    # All should give reasonable estimates
    assert all(5 <= count <= 25 for count in [default, basic, simple, multilingual])


def test_code_estimation():
    """Estimate tokens in code."""
    code = """
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"""

    tokens = estimate_tokens(code)
    assert tokens > 15  # Code has more tokens due to symbols
    print(f"Code snippet has {tokens} tokens")


def test_multilingual_text():
    """Estimate tokens in multilingual text."""
    # English
    english = "Hello, how are you?"
    en_tokens = estimate_tokens_multilingual(english)

    # Chinese
    chinese = "你好，你好吗？"
    cn_tokens = estimate_tokens_multilingual(chinese)

    # Japanese
    japanese = "こんにちは、元気ですか？"
    jp_tokens = estimate_tokens_multilingual(japanese)

    print(f"English: '{english}' = {en_tokens} tokens")
    print(f"Chinese: '{chinese}' = {cn_tokens} tokens")
    print(f"Japanese: '{japanese}' = {jp_tokens} tokens")

    assert all(count > 0 for count in [en_tokens, cn_tokens, jp_tokens])


def test_empty_and_whitespace():
    """Handle empty and whitespace strings."""
    # Empty string
    assert estimate_tokens("") == 0

    # Just spaces
    assert estimate_tokens("   ") >= 0

    # Just newlines
    assert estimate_tokens("\n\n") >= 0

    print("Empty and whitespace strings handled correctly")


def test_special_text():
    """Handle special text formats."""
    # URLs
    url = "https://github.com/masaishi/skimtoken"
    url_tokens = estimate_tokens(url)
    assert url_tokens > 0

    # Email
    email = "user@example.com"
    email_tokens = estimate_tokens(email)
    assert email_tokens > 0

    # Numbers and symbols
    mixed = "Price: $29.99 (save 40%!)"
    mixed_tokens = estimate_tokens(mixed)
    assert mixed_tokens > 0

    print(f"URL: {url_tokens} tokens")
    print(f"Email: {email_tokens} tokens")
    print(f"Mixed content: {mixed_tokens} tokens")


def test_file_content_estimation():
    """Example of estimating tokens in file content."""
    # Simulate reading from a file
    file_content = """
# README.md

This is a sample project.

## Features
- Fast performance
- Low memory usage
- Easy to use

## Installation
pip install skimtoken
"""

    tokens = estimate_tokens(file_content)
    assert tokens > 20
    print(f"README file would have approximately {tokens} tokens")


def test_batch_processing():
    """Process multiple texts."""
    texts = [
        "First text",
        "Second text is a bit longer",
        "Third text is the longest of all three texts",
    ]

    results: list[tuple[str, int]] = []
    for text in texts:
        tokens = estimate_tokens(text)
        results.append((text, tokens))
        print(f"'{text}' = {tokens} tokens")

    # Verify increasing token counts
    assert results[0][1] < results[1][1] < results[2][1]


def test_quick_check():
    """Quick way to check if text is within token limits."""
    max_tokens = 100

    short_text = "This is a short text."
    long_text = " ".join(["This is a very long text."] * 50)

    short_tokens = estimate_tokens(short_text)
    long_tokens = estimate_tokens(long_text)

    assert short_tokens < max_tokens
    assert long_tokens > max_tokens

    print(f"Short text: {short_tokens} tokens (within limit)")
    print(f"Long text: {long_tokens} tokens (exceeds limit)")


if __name__ == "__main__":
    # Run all test functions when executed directly
    test_simplest_usage()
    test_basic_english_text()
    test_counting_tokens_in_paragraph()
    test_different_methods()
    test_code_estimation()
    test_multilingual_text()
    test_empty_and_whitespace()
    test_special_text()
    test_file_content_estimation()
    test_batch_processing()
    test_quick_check()
    print("\nAll basic usage tests passed!")
