# prompt-fmt

A CLI tool for optimizing LLM prompts by reducing token count while preserving content clarity. Save tokens and reduce API costs when working with large language models.

## Features

- Multiple formatting modes for different optimization levels
- Real-time token counting with support for various LLM models
- Cost calculation based on token usage
- Process files or stdin input
- Preserves prompt meaning while removing unnecessary whitespace

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/prompt_formatter_cli`.

## Usage

### Basic Usage

Format a file:
```bash
prompt-fmt input.txt
```

Format from stdin:
```bash
cat prompt.txt | prompt-fmt
```

### Formatting Modes

**Readable** (default): Balanced optimization
```bash
prompt-fmt input.txt --mode readable
```

**Compact**: Aggressive token reduction
```bash
prompt-fmt input.txt --mode compact
```

**Minimal**: Maximum compression (single line output)
```bash
prompt-fmt input.txt --mode minimal
```

### Model Selection

Specify the LLM model for accurate token counting:
```bash
prompt-fmt input.txt --model gpt-4
```

Default model is `gpt-4`.

## How It Works

The formatter optimizes prompts by:
- Removing excessive whitespace (2+ spaces → 1 space)
- Trimming trailing whitespace from lines
- Normalizing multiple newlines (3+ → 2)
- Optionally converting to single line (minimal mode)

Token counting uses the tiktoken library to provide accurate counts for different LLM models.

## Output

The tool displays:
- Original token count and estimated cost
- Formatted token count and estimated cost
- Tokens saved (count and percentage)
- Cost savings
- Formatted prompt text

## Dependencies

- clap: Command-line argument parsing
- tiktoken-rs: Token counting for LLM models
- regex: Text pattern matching
- colored: Terminal output formatting
- anyhow: Error handling

## License

MIT
