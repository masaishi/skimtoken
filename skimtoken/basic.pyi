"""Type stubs for skimtoken.basic module."""

def estimate_tokens(text: str) -> int:
    """Estimate token count using basic features (chars, words, etc.)."""
    ...

def count(text: str) -> tuple[int, int, float, int]:
    """Extract features for basic method: (char_count, word_count, avg_word_length, space_count)."""
    ...
