"""Type stubs for skimtoken.multilingual module."""

def estimate_tokens(text: str) -> int:
    """Estimate token count using language-specific parameters."""
    ...

def count(text: str) -> tuple[int, int, float, int, str]:
    """Extract features for multilingual method: (char_count, word_count, avg_word_length, space_count, language)."""
    ...
