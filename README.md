# skimtoken (Beta)

A lightweight, fast token count estimation library written in Rust with Python bindings. Get token estimates with **65x less memory** than tiktoken.

[![PyPI](https://img.shields.io/pypi/v/skimtoken)](https://pypi.org/project/skimtoken/)
[![Crates.io](https://img.shields.io/crates/v/skimtoken)](https://crates.io/crates/skimtoken)
[![License](https://img.shields.io/github/license/masaishi/skimtoken)](https://github.com/masaishi/skimtoken/blob/main/LICENSE)

## Why skimtoken?

**The Problem**: [tiktoken](https://github.com/openai/tiktoken) is great for precise tokenization, but requires ~60MB of memory just to count tokens - problematic for memory-constrained environments.

**The Solution**: skimtoken estimates token counts using statistical patterns instead of loading entire vocabularies, achieving:

- ✅ **65x less memory** (0.9MB vs 60MB)
- ✅ **128x faster startup** (4ms vs 525ms) 
- ❌ Trade-off: ~27% error rate vs exact counts

## Installation

```bash
pip install skimtoken
```

Requirements: Python 3.9+

## Quick Start

```python
from skimtoken import estimate_tokens

# Basic usage
text = "Hello, world! How are you today?"
token_count = estimate_tokens(text)
print(f"Estimated tokens: {token_count}")  # ~7 tokens


# For better accuracy with non-English text
from skimtoken.multilingual import estimate_tokens as ml_estimate

multilingual_text = """
For non-space separated languages, the number of tokens is difficult to predict.
スペースで区切られていない言語の場合トークン数を予測するのは難しいです。
स्पेसद्वारावियोजितनहींभाषाओंकेलिएटोकनकीसंख्याकाअनुमानलगानाकठिनहै।
بالنسبةللغاتالتيلاتفصلبمسافاتفإنالتنبؤبعددالرموزصعب
"""
tokens = ml_estimate(multilingual_text)
print(f"Estimated tokens (multilingual): {tokens}")
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

### ❌ Not suitable for:

| Use Case | Why It Fails | Use Instead |
|----------|--------------|-------------|
| **Context Limits** | Underestimating causes failures | tiktoken |
| **Exact Billing** | 27% error = unhappy customers | tiktoken |
| **Token Splitting** | Chunks might exceed limits | tiktoken |
| **Embeddings** | Need exact token boundaries | tiktoken |

## Performance Comparison

### Large-Scale Benchmark (100k samples)

```
Results on CC100 multilingual dataset:
Total Samples: 100,726
Mean Error Rate: 27.05%

┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━┓
┃ Metric       ┃   tiktoken ┃  skimtoken ┃  Ratio ┃
┡━━━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━┩
│ Init Time    │ 0.525221 s │ 0.004084 s │ 0.008x │
│ Init Memory  │ 42.2386 MB │  0.0290 MB │ 0.001x │
│ Exec Time    │ 4.012049 s │ 7.635931 s │ 1.903x │
│ Exec Memory  │ 17.3251 MB │  0.8822 MB │ 0.051x │
│ Total Memory │ 59.5637 MB │  0.9111 MB │ 0.015x │
└──────────────┴────────────┴────────────┴────────┘
```

### Real-World Latency (Single Request)

```python
# Cold start (first request)
tiktoken:  525ms startup + 0.04ms/text = 525.04ms
skimtoken:   4ms startup + 0.08ms/text =   4.08ms  ← 128x faster

# Warm start (subsequent requests)  
tiktoken:  0.04ms/text
skimtoken: 0.08ms/text  ← 2x slower per text
```

## Available Methods

| Method | Import | Memory | Error | Best For |
|--------|---------|--------|-------|----------|
| **Simple** | `from skimtoken.simple import estimate_tokens` | 0.5MB | ~30% | English text, minimum memory |
| **Basic** | `from skimtoken.basic import estimate_tokens` | 0.7MB | ~28% | General use |
| **Multilingual** | `from skimtoken.multilingual import estimate_tokens` | 0.9MB | ~25% | Non-English, mixed languages |

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
# Output: 3

# From file
skimtoken -f document.txt
# Output: 1,523 tokens

# Multiple files
cat *.md | skimtoken
# Output: 48,291 tokens
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
                                                                  Only 0.9MB of parameters
```

## Accuracy Characteristics

Current performance: **RMSE of 37.74 tokens (27.05% error rate)** on multilingual dataset

### By Text Type:
- **English**: ~20% error (optimized)
- **Code**: ~25% error  
- **Chinese/Japanese**: ~30% error
- **Mixed languages**: ~35% error

### By Text Length:
- **Short (<100 tokens)**: ±40% error
- **Medium (100-1000)**: ±25% error  
- **Long (>1000)**: ±20% error

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

### Memory-Constrained Deployment

```python
# Cloudflare Worker example (128MB limit)
from skimtoken.simple import estimate_tokens  # Smallest footprint

export default {
  async fetch(request) {
    const text = await request.text();
    
    // Quick estimation for rate limiting
    const estimated = estimate_tokens(text);
    if (estimated > DAILY_LIMIT) {
      return new Response("Rate limit exceeded", { status: 429 });
    }
    
    // Process request...
  }
}
```

### Batch Processing

```python
from skimtoken import estimate_tokens

def process_documents(documents, token_budget=100000):
    """Process documents within token budget."""
    processed = []
    total_estimate = 0
    
    for doc in documents:
        doc_estimate = estimate_tokens(doc)
        
        # Conservative check (accounts for ~30% overestimation)
        if total_estimate + doc_estimate > token_budget * 0.7:
            break
            
        processed.append(doc)
        total_estimate += doc_estimate
    
    return processed
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

## Contributing

We welcome contributions! Priority areas:

1. **Accuracy improvements** - Better statistical models
2. **Language support** - More language-specific parameters
3. **Benchmarks** - Real-world performance data
4. **Use cases** - Examples and best practices

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](./LICENSE) for details.