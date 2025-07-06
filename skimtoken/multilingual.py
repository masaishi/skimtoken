"""Multilingual method: language-aware estimation."""

from ._skimtoken_core import estimate_tokens_multilingual as estimate_tokens
from ._skimtoken_core import count_multilingual as count

__all__ = ["estimate_tokens", "count"]
