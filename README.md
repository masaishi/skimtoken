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

### Quick Start

```python
from skimtoken import estimate_tokens

# Basic usage (uses simple method by default)
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

### Lightweight Imports

For minimal memory usage, import only the method you need:

```python
# Import only what you need
from skimtoken.simple import estimate_tokens    # ~0.43MB memory (simple method)
from skimtoken.basic import estimate_tokens     # Basic multi-feature estimation
from skimtoken.language import estimate_tokens  # Language detection included

# Example usage
text = "def factorial(n): return 1 if n <= 1 else n * factorial(n-1)"
tokens = estimate_tokens(text)
```

### CLI Usage

```bash
# From command line
echo "Hello, world!" | skimtoken

# From file
skimtoken -f document.txt

# Direct text
skimtoken "The quick brown fox jumps over the lazy dog."
```

## How it Works

skimtoken uses a **simple character-based approach** by default:

- **Approach**: Character count × coefficient (~0.4)
- **Speed**: Fastest initialization (133x faster than tiktoken)
- **Memory**: Minimal (~0.43MB vs tiktoken's ~85MB)
- **Use case**: Quick estimates, memory-constrained environments

Different estimation methods are available through lightweight imports:
- **Simple method**: `from skimtoken.simple import estimate_tokens`
- **Basic method**: `from skimtoken.basic import estimate_tokens` (multi-feature regression)
- **Language method**: `from skimtoken.language import estimate_tokens` (language detection + specific parameters)

## Language Support

The language method detects text language and applies optimized parameters for:

- **Latin scripts**: English, Spanish, French, German, etc.
- **Asian languages**: Chinese, Japanese, Korean
- **RTL languages**: Arabic, Hebrew
- **Cyrillic scripts**: Russian, Ukrainian, Bulgarian
- **Indic scripts**: Hindi, Bengali, Tamil

**Current Accuracy**: RMSE of 12.55 tokens across diverse multilingual test data

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

## Architecture

skimtoken is built with a modular architecture in Rust with Python bindings:

```
skimtoken/
├── src/
│   ├── lib.rs                 # Core library and Python bindings
│   ├── method.rs             # Trait definition for estimation methods
│   ├── method_simple.rs      # Simple character-based estimation
│   ├── method_basic.rs       # Multi-feature regression
│   ├── method_language.rs    # Language-aware estimation
│   └── skimtoken/            # Python module
│       ├── __init__.py       # Main API and CLI
│       ├── simple.py         # Simple method exports
│       ├── basic.py          # Basic method exports
│       └── language.py       # Language method exports
├── params/                   # Optimized parameters
│   └── simple.toml          # Simple method coefficient
└── scripts/
    ├── optimize/            # Parameter optimization tools
    └── benchmark.py         # Performance benchmarking
```

## Parameter Optimization

skimtoken includes tools to optimize parameters using your own data:

```bash
# Prepare dataset with token counts
uv run python scripts/prepare_cc100_dataset.py

# Optimize all methods
uv run python scripts/optimize_all.py

# Or optimize individual methods
uv run python scripts/optimize/optimize_simple.py
uv run python scripts/optimize/optimize_basic.py
uv run python scripts/optimize/optimize_language.py
```

Parameters are stored in TOML files and automatically loaded at runtime.

## Testing & Development

```bash
# Install dependencies
uv sync

# Build for development
uv run maturin dev --features python

# Run tests
cargo test
uv run pytest

# Code quality checks
uv run ruff format && uv run ruff check --fix && uv run pyright
cargo fmt && cargo clippy -- -D warnings

# Run performance benchmarks
uv run scripts/benchmark.py

# Run multiple benchmarks
./scripts/run_benchmark_multiple.sh
```

## Examples

Check out the examples directory:

- **`examples/example.py`** - Basic usage demonstration
- **`examples/lightweight_imports.py`** - Comparing all three methods with tiktoken ground truth

Run examples:
```bash
# Basic example
uv run python examples/example.py

# Compare methods
uv run python examples/lightweight_imports.py
```

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

MIT License - see [LICENSE](./LICENSE) for details.
