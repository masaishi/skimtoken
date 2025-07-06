"""Multilingual simple method: simplified language-aware estimation."""

from ._skimtoken_core import estimate_tokens_multilingual_simple as estimate_tokens
from ._skimtoken_core import count_multilingual_simple as count

__all__ = ["estimate_tokens", "count"]
