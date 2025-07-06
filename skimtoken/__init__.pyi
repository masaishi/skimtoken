"""Type stubs for skimtoken."""

from typing import Literal, TYPE_CHECKING

__version__: str

# Type stubs for submodules
if TYPE_CHECKING:
    pass

def estimate_tokens(
    text: str,
    method: Literal["simple", "basic", "multilingual", "multilingual_simple"] | None = None,
) -> int:
    """Estimate token count for text using specified method."""
    ...

def estimate_tokens_simple(text: str) -> int:
    """Estimate token count using simple character count method."""
    ...

def estimate_tokens_basic(text: str) -> int:
    """Estimate token count using basic features (chars, words, etc.)."""
    ...

def estimate_tokens_multilingual(text: str) -> int:
    """Estimate token count using language-specific parameters."""
    ...

def estimate_tokens_multilingual_simple(text: str) -> int:
    """Estimate token count using language-specific simple method."""
    ...

def count_simple(text: str) -> int:
    """Extract character count for simple method."""
    ...

def count_basic(text: str) -> tuple[int, int, float, int]:
    """Extract features for basic method: (char_count, word_count, avg_word_length, space_count)."""
    ...

def count_multilingual(text: str) -> tuple[int, int, float, int, str]:
    """Extract features for multilingual method: (char_count, word_count, avg_word_length, space_count, language)."""
    ...

def count_multilingual_simple(text: str) -> tuple[int, str]:
    """Extract features for multilingual simple method: (char_count, language)."""
    ...

def detect_language(text: str) -> str:
    """Detect language of text using whatlang."""
    ...

def main() -> None:
    """CLI entry point."""
    ...

__all__: list[str]
