# skimtoken (Beta)

A lightweight, fast token count estimation library written in Rust with Python bindings. Built for applications where approximate token counts work fine and memory/startup time efficiency matters.

# Why skimtoken?

[tiktoken](https://github.com/openai/tiktoken) is great for precise tokenization, but comes with serious overhead for simple token counting - especially **memory usage and initialization time**:

```bash
./scripts/run_benchmark_multiple.sh
```

```
╭────────────────── Mean Results After 100 Runs ─────────────────╮
│ Mean RMSE: 12.5526 tokens                                      │
├─────────────────┬──────────────┬──────────────┬────────────────┤
│ Metric          │   tiktoken   │  skimtoken   │     Ratio      │
├─────────────────┼──────────────┼──────────────┼────────────────┤
│ Init Time       │   0.135954 s │   0.001022 s │         0.007x │
│ Init Memory     │    84.5169 MB│     0.4292 MB│         0.005x │
│ Exec Time       │   0.002947 s │   0.113127 s │        38.387x │
│ Exec Memory     │     0.6602 MB│     0.0485 MB│         0.073x │
├─────────────────┼──────────────┼──────────────┼────────────────┤
│ TOTAL Time      │   0.138901 s │   0.114149 s │         0.821x │
│ TOTAL Memory    │    85.1770 MB│     0.4777 MB│         0.005x │
╰─────────────────┴──────────────┴──────────────┴────────────────╯
```

## Memory Advantages

**skimtoken uses >99% less memory** than tiktoken:
- **tiktoken**: ~85MB for initialization (loading vocabulary and encoder files)
- **skimtoken**: ~0.43MB for initialization, ~0.48MB total peak usage
- **178x less memory usage** - perfect for memory-constrained environments

**Memory-Efficient Design**: 
- No large vocabulary files to load into memory
- Minimal runtime memory footprint
- Predictable memory usage patterns

**Performance Trade-offs**: skimtoken targets **memory-constrained scenarios** and **cold-start environments** where initialization time directly impacts user experience. While tiktoken is faster for individual operations (~38x) and more accurate, skimtoken's minimal initialization overhead (133x faster startup, 178x less memory) makes it **1.22x faster overall** when you need to load fresh each time.

This makes skimtoken valuable in:
- **Serverless functions** with strict memory limits (128MB-512MB)
- **Edge computing** environments with limited RAM
- **Mobile applications** where memory matters
- **Containerized microservices** with tight memory constraints
- **Shared hosting environments** where memory usage affects cost

## Installation

```bash
pip install skimtoken
```

## Usage

```python
from skimtoken import estimate_tokens

# Basic usage
text = "Hello, world! How are you today?"
token_count = estimate_tokens(text)
print(f"Estimated tokens: {token_count}")

# Works with any text
code = """
def hello_world():
    print("Hello, world!")
    return True
"""
tokens = estimate_tokens(code)
print(f"Code tokens: {tokens}")
```

## Language Support

skimtoken uses **language-specific parameters** tailored for different language families to improve estimation accuracy. Each language family has its own optimized coefficients based on tokenization patterns.

**Supported languages**: English, French, Spanish, German, Russian, Hindi, Arabic, Chinese, Japanese, Korean, etc.

**Current Accuracy**: RMSE of 12.55 across 146 samples (11,745 characters) with testing across multiple language families and text types

## When to Use skimtoken vs tiktoken

**Use skimtoken when:**
- Working in **serverless/edge environments** (Cloudflare Workers, AWS Lambda, Vercel Functions) where cold start time and memory usage matter
- You need **quick token estimates** for API planning and cost estimation
- **Initialization overhead** is a concern (e.g., short-lived processes that can't amortize tiktoken's startup cost)
- Approximate counts work for your use case
- Memory constraints are tight

**Use Tiktoken when:**
- You need **exact token counts** for specific models and tokenization-dependent features
- **Processing large batches** of text where you can load the encoder once and reuse it
- Building applications that require **precise tokenization** (not just counting)
- You have **persistent memory** and can afford tiktoken's initialization cost
- **Accuracy is more important** than speed/memory efficiency

**Key Trade-off**: While tiktoken is faster for individual tokenization operations and more accurate, skimtoken excels in environments where you **can't afford to keep encoders loaded in memory** or where **cold start performance matters more than raw throughput**.

## Roadmap

**Automated Parameter Optimization**: Plans to implement hyperparameter tuning using large-scale datasets like CC100 samples to minimize RMSE scores across language families.

The goal is to achieve sub-10 RMSE for major language families while preserving skimtoken's core advantages of minimal initialization overhead and memory usage.

## Testing & Development

```bash
# Install dependencies
uv sync

# Build for development
uv run maturin dev --features python

# Run tests
cargo test
uv run pytest

# Run specific test with verbose output
uv run pytest tests/test_skimtoken_simple.py -s

# Run performance benchmarks
uv run scripts/benchmark.py
```

### Test Results

Run accuracy testing:
```bash
uv run pytest tests/test_skimtoken_simple.py -s
```

```
RMSE by Category:
╭───────────────────────┬───────┬─────────┬────────╮
│ Category              │  RMSE │ Samples │ Status │
├───────────────────────┼───────┼─────────┼────────┤
│ ambiguous_punctuation │  2.88 │       7 │ ✓ PASS │
│ code                  │ 10.15 │      14 │ ✓ PASS │
│ edge                  │  0.00 │       2 │ ✓ PASS │
│ json                  │  8.54 │       3 │ ✓ PASS │
│ jsonl                 │ 15.51 │       2 │ ✓ PASS │
│ mixed                 │  4.12 │       3 │ ✓ PASS │
│ noisy_text            │  4.04 │       7 │ ✓ PASS │
│ repetitive            │  7.25 │       4 │ ✓ PASS │
│ rtl                   │  3.71 │       4 │ ✓ PASS │
│ special               │  4.69 │       3 │ ✓ PASS │
│ special_encoding      │ 10.65 │       8 │ ✓ PASS │
│ structured_format     │  3.82 │       8 │ ✓ PASS │
│ unknown               │ 15.43 │      81 │ ✓ PASS │
╰───────────────────────┴───────┴─────────┴────────╯

Summary Statistics:
Overall RMSE: 12.55 tokens
Total samples processed: 146
Total characters: 12,377
Execution time: 0.121 seconds
Processing speed: 1204 samples/second
Character throughput: 102,110 chars/second
Average per character: 9.793μs
```

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

MIT License - see [LICENSE](./LICENSE) for details.
