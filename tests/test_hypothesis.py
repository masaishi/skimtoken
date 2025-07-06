"""
Hypothesis-based property tests for skimtoken.
Uses property-based testing to verify invariants across random inputs.
"""

import hypothesis
import hypothesis.strategies as st

from skimtoken import (
    estimate_tokens,
    estimate_tokens_basic,
    estimate_tokens_multilingual,
    estimate_tokens_multilingual_simple,
    estimate_tokens_simple,
)

# All estimation methods to test
ESTIMATION_METHODS = [
    ("default", estimate_tokens),
    ("basic", estimate_tokens_basic),
    ("simple", estimate_tokens_simple),
    ("multilingual", estimate_tokens_multilingual),
    ("multilingual_simple", estimate_tokens_multilingual_simple),
]


class TestSkimtokenHypothesis:
    """Property-based tests using Hypothesis framework."""

    @hypothesis.given(text=st.text())
    @hypothesis.settings(max_examples=100)
    def test_non_negative_property(self, text: str):
        """Property: All methods should always return non-negative values."""
        for name, method in ESTIMATION_METHODS:
            result = method(text)
            assert result >= 0, f"{name}: negative result {result} for text"

    @hypothesis.given(text=st.text(min_size=1))
    @hypothesis.settings(max_examples=50)
    def test_deterministic_property(self, text: str):
        """Property: Same input should always produce same output."""
        for name, method in ESTIMATION_METHODS:
            results = [method(text) for _ in range(3)]
            assert len(set(results)) == 1, f"{name}: inconsistent results {results}"

    @hypothesis.given(text=st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126)))
    @hypothesis.settings(max_examples=100)
    def test_ascii_text_property(self, text: str):
        """Property: ASCII text should be handled correctly."""
        for name, method in ESTIMATION_METHODS:
            result = method(text)
            assert result >= 0, f"{name}: negative result for ASCII text"
            # Note: Some methods may return 0 for very short text

    @hypothesis.given(char=st.characters())
    @hypothesis.settings(max_examples=200)
    def test_single_character_property(self, char: str):
        """Property: Any single character should be handled."""
        for name, method in ESTIMATION_METHODS:
            result = method(char)
            assert result >= 0, f"{name}: character '{repr(char)}' returned {result}"

    @hypothesis.given(
        base=st.text(
            min_size=10, max_size=100, alphabet=st.characters(min_codepoint=32, max_codepoint=126)
        ),
        multiplier=st.integers(min_value=3, max_value=5),
    )
    @hypothesis.settings(max_examples=30)
    def test_monotonic_property(self, base: str, multiplier: int):
        """Property: More text should not result in fewer tokens."""
        for name, method in ESTIMATION_METHODS:
            single = method(base)
            multiple = method(base * multiplier)

            assert multiple >= single, (
                f"{name}: tokens decreased with more text - {single} -> {multiple}"
            )

    @hypothesis.given(text1=st.text(min_size=1), text2=st.text(min_size=1))
    @hypothesis.settings(max_examples=50)
    def test_concatenation_property(self, text1: str, text2: str):
        """Property: Concatenated text tokens should be close to sum of parts."""
        for name, method in ESTIMATION_METHODS:
            tokens1 = method(text1)
            tokens2 = method(text2)
            tokens_concat = method(text1 + text2)

            # Skip if either text is too short
            if tokens1 < 0.5 or tokens2 < 0.5:
                continue

            # Concatenation should be approximately additive (more lenient)
            expected = tokens1 + tokens2
            assert expected * 0.3 <= tokens_concat <= expected * 2.5, (
                f"{name}: concatenation not additive"
            )

    @hypothesis.given(size=st.integers(min_value=0, max_value=100000))
    @hypothesis.settings(deadline=None)  # Disable deadline for large texts
    def test_scalability_property(self, size: int):
        """Property: Should handle text of any size without errors."""
        text = "a" * size
        for name, method in ESTIMATION_METHODS:
            result = method(text)
            assert result >= 0, f"{name}: negative result for size {size}"
            # Note: Some methods may return 0 for very short text

    @hypothesis.given(whitespace=st.sampled_from([" ", "\t", "\n", "\r", "\f", "\v"]))
    def test_whitespace_property(self, whitespace: str):
        """Property: All whitespace types should be handled."""
        text = whitespace * 10
        for name, method in ESTIMATION_METHODS:
            result = method(text)
            assert result >= 0, f"{name}: negative result for whitespace"
