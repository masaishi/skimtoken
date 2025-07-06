# skimtoken (Early Beta)

**⚠️ WARNING: This is an early beta version. The current implementation is not production-ready.**

A lightweight, fast token count estimation library written in Rust with Python bindings.

[![PyPI](https://img.shields.io/pypi/v/skimtoken)](https://pypi.org/project/skimtoken/)
[![Crates.io](https://img.shields.io/crates/v/skimtoken)](https://crates.io/crates/skimtoken)
[![License](https://img.shields.io/github/license/masaishi/skimtoken)](https://github.com/masaishi/skimtoken/blob/main/LICENSE)

## ⚠️ Current Limitations

**This library is currently in early beta and has significant accuracy issues:**

- **Multilingual method**: Takes 1.13x longer than tiktoken due to inefficient implementation
- **Overall accuracy**: 15.11% error rate, which is too high for most use cases


## Why skimtoken?

**The Problem**: [tiktoken](https://github.com/openai/tiktoken) is great for precise tokenization, but requires ~60MB of memory just to count tokens - problematic for memory-constrained environments.

**The Solution**: skimtoken estimates token counts using statistical patterns instead of loading entire vocabularies, achieving:

- ✅ **64x less memory** (0.92MB vs 60MB)
- ✅ **128x faster startup** (4ms vs 485ms) 
- ❌ **1.13x slower execution** (5.51s vs 4.59s) for multilingual method
- ❌ Trade-off: ~15.11% error rate vs exact counts

## Installation

```bash
pip install skimtoken
```

Requirements: Python 3.9+

## Quick Start

Simple method (Just char length x coefficient):
```python
from skimtoken import estimate_tokens

# Basic usage
text = "Hello, world! How are you today?"
token_count = estimate_tokens(text)
print(f"Estimated tokens: {token_count}")
```

Multilingual simple method:
```python
from skimtoken.multilingual_single import estimate_tokens

multilingual_text = """
For non-space separated languages, the number of tokens is difficult to predict.
スペースで区切られていない言語の場合トークン数を予測するのは難しいです。
स्पेसद्वारावियोजितनहींभाषाओंकेलिएटोकनकीसंख्याकाअनुमानलगानाकठिनहै।
بالنسبةللغاتالتيلاتفصلبمسافاتفإنالتنبؤبعددالرموزصعب
"""
token_count = estimate_tokens(multilingual_text)
print(f"Estimated tokens (multilingual): {token_count}")
```

## When to Use skimtoken

### ✅ Perfect for:

| Use Case | Why It Works | Example |
|----------|--------------|---------|
| **Rate Limiting** | Overestimating is safe | Prevent API quota exceeded |
| **Cost Estimation** | Users prefer conservative estimates | "$0.13" (actual: $0.10) |
| **Progress Bars** | Approximate progress is fine | Processing documents |
| **Serverless/Edge** | Memory constraints (128MB limits) | Cloudflare Workers |
| **Quick Filtering** | Remove obviously too-long content | Pre-screening |
| **Model Switching** | Switch to smart model when context long | Auto-escalation |

### ❌ Not suitable for:

| Use Case | Why It Fails | Use Instead |
|----------|--------------|-------------|
| **Context Limits** | Underestimating causes failures | tiktoken |
| **Exact Billing** | 15% error = unhappy customers | tiktoken |
| **Token Splitting** | Chunks might exceed limits | tiktoken |
| **Embeddings** | Need exact token boundaries | tiktoken |

## Performance Comparison

### Large-Scale Benchmark (100k samples)

Simple method (Just char length x coefficient):
```
Results:
Total Samples: 100,726
Total Characters: 13,062,391
Mean RMSE: 38.4863 tokens
Mean Error Rate: 21.63%

┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━┓
┃ Metric       ┃   tiktoken ┃  skimtoken ┃  Ratio ┃
┡━━━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━┩
│ Init Time    │ 0.481672 s │ 0.182308 s │ 0.378x │
├──────────────┼────────────┼────────────┼────────┤
│ Init Memory  │ 42.2386 MB │  0.0291 MB │ 0.001x │
├──────────────┼────────────┼────────────┼────────┤
│ Exec Time    │ 4.710224 s │ 0.805272 s │ 0.171x │
├──────────────┼────────────┼────────────┼────────┤
│ Exec Memory  │ 17.3251 MB │  0.8849 MB │ 0.051x │
├──────────────┼────────────┼────────────┼────────┤
│ Total Time   │ 5.191896 s │ 0.928758 s │ 0.190x │
├──────────────┼────────────┼────────────┼────────┤
│ Total Memory │ 59.5637 MB │  0.9214 MB │ 0.015x │
└──────────────┴────────────┴────────────┴────────┘
```

Multilingual simple method:
```
Results:
Total Samples: 100,726
Total Characters: 13,062,391
Mean RMSE: 21.3034 tokens
Mean Error Rate: 15.11%

┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━┓
┃ Metric       ┃   tiktoken ┃  skimtoken ┃  Ratio ┃
┡━━━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━┩
│ Init Time    │ 0.815441 s │ 0.138714 s │ 0.170x │
├──────────────┼────────────┼────────────┼────────┤
│ Init Memory  │ 42.4791 MB │  0.1613 MB │ 0.004x │
├──────────────┼────────────┼────────────┼────────┤
│ Exec Time    │ 4.041857 s │ 5.380782 s │ 1.331x │
├──────────────┼────────────┼────────────┼────────┤
│ Exec Memory  │ 17.3227 MB │  0.8950 MB │ 0.052x │
├──────────────┼────────────┼────────────┼────────┤
│ Total Time   │ 4.857297 s │ 5.519496 s │ 1.136x │
├──────────────┼────────────┼────────────┼────────┤
│ Total Memory │ 59.8018 MB │  1.0563 MB │ 0.018x │
└──────────────┴────────────┴────────────┴────────┘
```

## Available Methods

| Method | Import | Memory | Error | Best For |
|--------|---------|--------|-------|----------|
| **Simple** | `from skimtoken.simple import estimate_tokens` | 0.8MB | ~21% | English text, minimum memory |
| **Basic** | `from skimtoken.basic import estimate_tokens` | 0.8MB | ~27% | General use |
| **Multilingual** | `from skimtoken.multilingual import estimate_tokens` | 0.9MB | ~15% | Non-English, mixed languages |

```python
# Example: Choose method based on your needs
if memory_critical:
    from skimtoken.simple import estimate_tokens
elif mixed_languages:
    from skimtoken.multilingual import estimate_tokens
else:
    from skimtoken import estimate_tokens  # Default: simple
```

## CLI Usage

```bash
# From command line
echo "Hello, world!" | skimtoken
# Output: 5

# From file
skimtoken -f document.txt
# Output: 236

# Multiple files
cat *.md | skimtoken
# Output: 4846
```

## How It Works

Unlike tiktoken's vocabulary-based approach, skimtoken uses statistical patterns:

**tiktoken**:
```
Text → Tokenizer → ["Hello", ",", " world"] → Vocabulary Lookup → [1234, 11, 4567] → Count: 3
                                                      ↑
                                              Requires 60MB dictionary
```

**skimtoken**:
```
Text → Feature Extraction → {chars: 13, words: 2, lang: "en"} → Statistical Model → ~3 tokens
                                                                         ↑
                                                                  Only 0.92MB of parameters
```

## Advanced Usage

### Optimize for Your Domain

Improve accuracy on domain-specific content:

```bash
# 1. Prepare labeled data
# Format: {"text": "your content", "actual_tokens": 123}
uv run scripts/prepare_dataset.py --input your_texts.txt

# 2. Optimize parameters
uv run scripts/optimize_all.py --dataset your_data.jsonl

# 3. Rebuild with custom parameters
uv run maturin build --release
```


## Architecture

```
skimtoken/
├── src/
│   ├── lib.rs                        # Core Rust library with PyO3 bindings
│   └── methods/
│       ├── method_simple.rs          # Character-based estimation
│       ├── method_basic.rs           # Multi-feature regression  
│       └── method_multilingual.rs    # Language-aware estimation
├── skimtoken/                        # Python package
│   ├── __init__.py                   # Main API
│   └── {method}.py                   # Method-specific imports
├── params/                           # Learned parameters (TOML)
└── scripts/
    ├── benchmark.py                  # Performance testing
    └── optimize/                     # Parameter training
```

## Development

```bash
# Setup
git clone https://github.com/masaishi/skimtoken
cd skimtoken
uv sync

# Development build
uv run maturin dev --features python

# Run tests
cargo test
uv run pytest

# Benchmark
uv run scripts/benchmark.py
```

## FAQ

**Q: Can I improve accuracy?**  
A: Yes! You can adjust the parameters using your own data to improve accuracy. See [Advanced Usage](#advanced-usage) for details.

**Q: Is the API stable?**  
A: Beta = breaking changes possible.

## Future Plans

We are actively working to improve skimtoken's accuracy and performance:

1. **Better estimation algorithms**: Moving beyond simple character multiplication to more sophisticated statistical models
2. **Performance optimization**: Fixing the 60x slowdown in multilingual method
3. **Improved language support**: Better handling of non-English languages
4. **Higher accuracy**: Targeting <10% error rate while maintaining low memory footprint

## License

MIT License - see [LICENSE](./LICENSE) for details.
