#!/usr/bin/env python3
"""
Script to load parameters from TOML file and update the Rust parameters in method_language.rs
"""

import toml
from pathlib import Path
from typing import Any


def load_params_from_toml(toml_path: str) -> dict[str, Any]:
    """Load parameters from TOML file."""
    with open(toml_path, "r") as f:
        data = toml.load(f)
    return data


def generate_language_params_rust(params_data: dict[str, Any]) -> str:
    """Generate Rust code for language parameters."""
    rust_code: list[str] = []

    # Generate default parameters
    default_params = params_data["default_params"]
    rust_code.append("impl Default for LanguageParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        Self {")
    rust_code.append(f"            char_coef: {default_params['char_coef']},")
    rust_code.append(f"            word_coef: {default_params['word_coef']},")
    rust_code.append(f"            avg_word_length_coef: {default_params['avg_word_length_coef']},")
    rust_code.append(f"            space_coef: {default_params['space_coef']},")
    rust_code.append(f"            intercept: {default_params['intercept']},")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")
    rust_code.append("")

    # Generate language-specific parameters
    rust_code.append("impl Default for LanguageMethodParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        let mut language_params = HashMap::new();")
    rust_code.append("")

    # Add language-specific parameters
    for lang_key, lang_params in params_data["language_params"].items():
        # Convert language code to proper format (first letter uppercase)
        lang_code = lang_key.capitalize()

        rust_code.append(f"        // {lang_code}")
        rust_code.append("        language_params.insert(")
        rust_code.append(f'            "{lang_code}".to_string(),')
        rust_code.append("            LanguageParameters {")
        rust_code.append(f"                char_coef: {lang_params['char_coef']},")
        rust_code.append(f"                word_coef: {lang_params['word_coef']},")
        rust_code.append(
            f"                avg_word_length_coef: {lang_params['avg_word_length_coef']},"
        )
        rust_code.append(f"                space_coef: {lang_params['space_coef']},")
        rust_code.append(f"                intercept: {lang_params['intercept']},")
        rust_code.append("            },")
        rust_code.append("        );")
        rust_code.append("")

    rust_code.append("        Self {")
    rust_code.append("            default_params: LanguageParameters::default(),")
    rust_code.append("            language_params,")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")

    return "\n".join(rust_code)


def update_rust_file(rust_file_path: str, new_params_code: str) -> None:
    """Update the Rust file with new parameters."""
    with open(rust_file_path, "r") as f:
        content = f.read()

    # Find the start and end of the impl blocks to replace
    start_marker = "impl Default for LanguageParameters {"
    end_marker = "impl Default for LanguageMethodParameters {"

    # Find positions
    start_pos = content.find(start_marker)
    if start_pos == -1:
        raise ValueError("Could not find LanguageParameters impl block")

    # Find the end of the LanguageMethodParameters impl block
    end_pos = content.find(end_marker)
    if end_pos == -1:
        raise ValueError("Could not find LanguageMethodParameters impl block")

    # Find the final closing brace of the LanguageMethodParameters impl
    # Count braces to find the matching closing brace
    brace_count = 0
    pos = end_pos
    while pos < len(content):
        if content[pos] == "{":
            brace_count += 1
        elif content[pos] == "}":
            brace_count -= 1
            if brace_count == 0:
                final_end_pos = pos + 1
                break
        pos += 1
    else:
        raise ValueError("Could not find end of LanguageMethodParameters impl block")

    # Replace the content
    new_content = content[:start_pos] + new_params_code + "\n\n" + content[final_end_pos:]

    # Write back to file
    with open(rust_file_path, "w") as f:
        f.write(new_content)


def main() -> None:
    # Define paths
    project_root = Path(__file__).parent.parent
    toml_path = project_root / "params" / "language.toml"
    rust_file_path = project_root / "src" / "methods" / "method_language.rs"

    print(f"Loading parameters from: {toml_path}")
    print(f"Updating Rust file: {rust_file_path}")

    # Load parameters
    params_data = load_params_from_toml(str(toml_path))

    # Generate Rust code
    rust_code = generate_language_params_rust(params_data)

    # Update Rust file
    update_rust_file(str(rust_file_path), rust_code)

    print("Successfully updated Rust parameters!")
    print(f"Updated {len(params_data['language_params'])} language-specific parameter sets")


if __name__ == "__main__":
    main()
