"""
Comprehensive tests for skimtoken covering normal and edge cases.
"""

from skimtoken import (
    estimate_tokens,
    estimate_tokens_basic,
    estimate_tokens_simple,
    estimate_tokens_multilingual,
    estimate_tokens_multilingual_simple,
)


class TestNormalCases:
    """Test normal use cases for all estimation methods."""

    def test_english_text(self):
        """Test common English text."""
        text = "Hello, world! This is a test."

        default = estimate_tokens(text)
        basic = estimate_tokens_basic(text)
        simple = estimate_tokens_simple(text)
        multi = estimate_tokens_multilingual(text)
        multi_simple = estimate_tokens_multilingual_simple(text)

        # All methods should return positive values
        assert default > 0
        assert basic > 0
        assert simple > 0
        assert multi > 0
        assert multi_simple > 0

        # Values should be reasonable (roughly 1 token per word)
        assert 5 <= default <= 15
        assert 5 <= basic <= 15
        assert 5 <= simple <= 15
        assert 5 <= multi <= 15
        assert 5 <= multi_simple <= 15

    def test_numbers(self):
        """Test numeric content."""
        text = "123 456 789 3.14159 -42"

        assert estimate_tokens(text) > 0
        assert estimate_tokens_basic(text) > 0
        assert estimate_tokens_simple(text) > 0
        assert estimate_tokens_multilingual(text) > 0
        assert estimate_tokens_multilingual_simple(text) > 0

    def test_code_snippet(self):
        """Test programming code."""
        text = """def hello(name):
    print(f"Hello, {name}!")
    return True"""

        result = estimate_tokens(text)
        assert result > 10  # Code typically has more tokens

    def test_url_and_email(self):
        """Test URLs and email addresses."""
        text = "Visit https://example.com or email user@example.com"

        assert estimate_tokens(text) > 0
        assert estimate_tokens_basic(text) > 0
        assert estimate_tokens_simple(text) > 0
        assert estimate_tokens_multilingual(text) > 0
        assert estimate_tokens_multilingual_simple(text) > 0

    def test_mixed_content(self):
        """Test mixed content types."""
        text = "Price: $19.99 (20% off!) Order #12345 @ store.com"

        assert estimate_tokens(text) > 0
        assert estimate_tokens_basic(text) > 0
        assert estimate_tokens_simple(text) > 0
        assert estimate_tokens_multilingual(text) > 0
        assert estimate_tokens_multilingual_simple(text) > 0


class TestMultilingualCases:
    """Test multilingual text handling."""

    def test_chinese(self):
        """Test Chinese text."""
        text = "你好，世界！这是一个测试。"
        result = estimate_tokens_multilingual(text)
        result_simple = estimate_tokens_multilingual_simple(text)
        assert result > 0
        assert result_simple > 0
        # Chinese typically has more tokens due to character-based tokenization
        assert result >= 6
        assert result_simple >= 3

    def test_japanese(self):
        """Test Japanese text."""
        text = "こんにちは、世界！これはテストです。"
        result = estimate_tokens_multilingual(text)
        result_simple = estimate_tokens_multilingual_simple(text)
        assert result > 0
        assert result_simple > 0

    def test_arabic(self):
        """Test Arabic text."""
        text = "مرحبا بالعالم! هذا اختبار."
        result = estimate_tokens_multilingual(text)
        result_simple = estimate_tokens_multilingual_simple(text)
        assert result > 0
        assert result_simple > 0

    def test_cyrillic(self):
        """Test Cyrillic text."""
        text = "Привет, мир! Это тест."
        result = estimate_tokens_multilingual(text)
        result_simple = estimate_tokens_multilingual_simple(text)
        assert result > 0
        assert result_simple > 0

    def test_mixed_languages(self):
        """Test mixed language text."""
        text = "Hello 你好 こんにちは مرحبا Привет"

        default = estimate_tokens(text)
        multi = estimate_tokens_multilingual(text)
        multi_simple = estimate_tokens_multilingual_simple(text)

        assert default > 0
        assert multi > 0
        assert multi_simple > 0
        # Multilingual should handle this well
        assert multi >= 5
        assert multi_simple >= 5


class TestEdgeCases:
    """Test edge cases and boundary conditions."""

    def test_empty_string(self):
        """Test empty string."""
        assert estimate_tokens("") >= 0
        assert estimate_tokens_basic("") >= 0
        assert estimate_tokens_simple("") >= 0
        assert estimate_tokens_multilingual("") >= 0

    def test_single_character(self):
        """Test single characters."""
        chars = ["a", "1", " ", "!", "你", "🚀"]

        for char in chars:
            assert estimate_tokens(char) >= 0
            assert estimate_tokens_basic(char) >= 0
            assert estimate_tokens_simple(char) >= 0
            assert estimate_tokens_multilingual(char) >= 0

    def test_whitespace_only(self):
        """Test whitespace-only strings."""
        texts = [" ", "   ", "\n", "\t", "\n\n\n", "\t \n"]

        for text in texts:
            result = estimate_tokens(text)
            assert result >= 0
            # Whitespace typically counts as tokens
            if len(text) == 1:
                assert result <= 2

    def test_special_characters(self):
        """Test special characters."""
        texts = [
            "!@#$%^&*()",
            '<>?:"{}|',
            "\\n\\t\\r",
            "\u0000\u0001\u0002",
            "\u200b\u200c\u200d",  # Zero-width characters
        ]

        for text in texts:
            assert estimate_tokens(text) >= 0
            assert estimate_tokens_basic(text) >= 0
            assert estimate_tokens_simple(text) >= 0
            assert estimate_tokens_multilingual(text) >= 0
            assert estimate_tokens_multilingual_simple(text) >= 0

    def test_very_long_text(self):
        """Test very long text."""
        # Repetitive pattern
        long_text = "Hello world! " * 10000
        result = estimate_tokens(long_text)
        assert result > 10000
        assert result < 50000  # Should be reasonable

        # Long single word
        long_word = "a" * 10000
        result = estimate_tokens(long_word)
        assert result > 0

    def test_unicode_edge_cases(self):
        """Test Unicode edge cases."""
        texts = [
            "🏳️‍🌈",  # Rainbow flag (complex emoji)
            "👨‍👩‍👧‍👦",  # Family emoji
            "🧑🏻‍💻",  # Person with skin tone modifier
            "é",  # Combining diacritical
            "ñ",  # Single character with tilde
        ]

        for text in texts:
            assert estimate_tokens(text) >= 0
            assert estimate_tokens_multilingual(text) >= 0

    def test_control_characters(self):
        """Test control characters."""
        text = "\x00\x01\x02\x03\x04\x05"
        assert estimate_tokens(text) >= 0

    def test_null_byte(self):
        """Test null byte handling."""
        text = "Hello\x00World"
        assert estimate_tokens(text) > 0


class TestConsistency:
    """Test consistency across methods."""

    def test_method_consistency(self):
        """Test that methods give consistent relative results."""
        texts = [
            "Simple text",
            "A longer piece of text with more words",
            "Short text example",
        ]

        for text in texts:
            default = estimate_tokens(text)
            basic = estimate_tokens_basic(text)
            simple = estimate_tokens_simple(text)
            multi = estimate_tokens_multilingual(text)

            # All should be non-negative
            assert default >= 0
            assert basic >= 0
            assert simple >= 0
            assert multi >= 0

            # Results should be in the same ballpark (more lenient)
            values = [v for v in [default, basic, simple, multi] if v > 0]
            if len(values) >= 2:
                min_val = min(values)
                max_val = max(values)
                assert max_val <= min_val * 5  # Allow more variance

    def test_proportionality(self):
        """Test that token count scales with text length."""
        base = "Hello world! "

        for method in [
            estimate_tokens,
            estimate_tokens_basic,
            estimate_tokens_simple,
            estimate_tokens_multilingual,
            estimate_tokens_multilingual_simple,
        ]:
            single = method(base)
            double = method(base * 2)
            triple = method(base * 3)

            # Skip if base returns 0
            if single == 0:
                continue

            # Should scale roughly linearly (more lenient)
            assert 0.8 * single <= double <= 3.0 * single
            assert 1.5 * single <= triple <= 4.5 * single

    def test_deterministic(self):
        """Test that results are deterministic."""
        text = "This is a test of deterministic behavior."

        for method in [
            estimate_tokens,
            estimate_tokens_basic,
            estimate_tokens_simple,
            estimate_tokens_multilingual,
            estimate_tokens_multilingual_simple,
        ]:
            results = [method(text) for _ in range(5)]
            # All results should be identical
            assert len(set(results)) == 1


class TestSpecialFormats:
    """Test special text formats."""

    def test_json(self):
        """Test JSON format."""
        text = '{"name": "test", "value": 42, "array": [1, 2, 3]}'
        result = estimate_tokens(text)
        assert result > 10  # JSON has many tokens due to structure

    def test_markdown(self):
        """Test Markdown format."""
        text = """# Title
        
## Subtitle

- List item 1
- List item 2

**Bold** and *italic* text."""

        result = estimate_tokens(text)
        assert result > 15

    def test_html(self):
        """Test HTML format."""
        text = '<div class="test"><p>Hello <strong>world</strong>!</p></div>'
        result = estimate_tokens(text)
        assert result > 10

    def test_csv_like(self):
        """Test CSV-like format."""
        text = "name,age,city\nJohn,30,NYC\nJane,25,LA"
        result = estimate_tokens(text)
        assert result > 10
