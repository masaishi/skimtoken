#!/usr/bin/env python3
"""
Script to load parameters from TOML files and update the Rust parameter files.
Handles all parameter types: simple, basic, multilingual, and multilingual_simple.
"""

from pathlib import Path
from typing import Any, Callable, TypedDict

import toml


class ParamConfig(TypedDict):
    name: str
    toml_path: Path
    rust_path: Path
    generator: Callable[[dict[str, Any]], str]


def load_params_from_toml(toml_path: str) -> dict[str, Any]:
    """Load parameters from TOML file."""
    with open(toml_path, "r") as f:
        data = toml.load(f)
    return data


def format_f32(value: float) -> str:
    """Format a float value for f32 with underscores for readability."""
    str_val = f"{value:.7g}"  # Use 7 significant digits for f32 precision

    if "e" in str_val.lower():
        return str_val

    if "." in str_val:
        integer_part, decimal_part = str_val.split(".")

        if len(decimal_part) > 3:
            formatted_decimal = ""
            for i, digit in enumerate(decimal_part):
                if i > 0 and i % 3 == 0:
                    formatted_decimal += "_"
                formatted_decimal += digit
            return f"{integer_part}.{formatted_decimal}"
        else:
            return str_val
    else:
        return str_val


def generate_simple_params_rust(params_data: dict[str, Any]) -> str:
    """Generate Rust code for simple parameters."""
    rust_code: list[str] = []

    rust_code.append("impl Default for SimpleParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        Self {")
    rust_code.append(f"            coefficient: {format_f32(params_data['coefficient'])},")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")

    return "\n".join(rust_code)


def generate_basic_params_rust(params_data: dict[str, Any]) -> str:
    """Generate Rust code for basic parameters."""
    rust_code: list[str] = []

    rust_code.append("impl Default for BasicParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        Self {")
    rust_code.append(f"            char_coef: {format_f32(params_data['char_coef'])},")
    rust_code.append(f"            word_coef: {format_f32(params_data['word_coef'])},")
    rust_code.append(
        f"            avg_word_length_coef: {format_f32(params_data['avg_word_length_coef'])},"
    )
    rust_code.append(f"            space_coef: {format_f32(params_data['space_coef'])},")
    rust_code.append(f"            intercept: {format_f32(params_data['intercept'])},")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")

    return "\n".join(rust_code)


def generate_multilingual_params_rust(params_data: dict[str, Any]) -> str:
    """Generate Rust code for multilingual parameters."""
    rust_code: list[str] = []

    # Generate default parameters
    default_params = params_data["default_params"]
    rust_code.append("impl Default for MultilingualParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        Self {")
    rust_code.append(f"            char_coef: {format_f32(default_params['char_coef'])},")
    rust_code.append(f"            word_coef: {format_f32(default_params['word_coef'])},")
    rust_code.append(
        f"            avg_word_length_coef: {format_f32(default_params['avg_word_length_coef'])},"
    )
    rust_code.append(f"            space_coef: {format_f32(default_params['space_coef'])},")
    rust_code.append(f"            intercept: {format_f32(default_params['intercept'])},")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")
    rust_code.append("")

    # Generate language-specific parameters
    rust_code.append("impl Default for MultilingualMethodParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        let mut language_params = HashMap::new();")
    rust_code.append("")

    # Add language-specific parameters
    for lang_key, lang_params in params_data["language_params"].items():
        # Language codes are already in proper format
        rust_code.append(f"        // {lang_key}")
        rust_code.append("        language_params.insert(")
        rust_code.append(f'            "{lang_key}".to_string(),')
        rust_code.append("            MultilingualParameters {")
        rust_code.append(f"                char_coef: {format_f32(lang_params['char_coef'])},")
        rust_code.append(f"                word_coef: {format_f32(lang_params['word_coef'])},")
        rust_code.append(
            f"                avg_word_length_coef: {format_f32(lang_params['avg_word_length_coef'])},"
        )
        rust_code.append(f"                space_coef: {format_f32(lang_params['space_coef'])},")
        rust_code.append(f"                intercept: {format_f32(lang_params['intercept'])},")
        rust_code.append("            },")
        rust_code.append("        );")
        rust_code.append("")

    rust_code.append("        Self {")
    rust_code.append("            default_params: MultilingualParameters::default(),")
    rust_code.append("            language_params,")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")

    return "\n".join(rust_code)


def generate_multilingual_simple_params_rust(params_data: dict[str, Any]) -> str:
    """Generate Rust code for multilingual simple parameters."""
    rust_code: list[str] = []

    # Generate default parameters
    default_params = params_data["default_params"]
    rust_code.append("impl Default for MultilingualSimpleParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        Self {")
    rust_code.append(f"            coefficient: {format_f32(default_params['coefficient'])},")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")
    rust_code.append("")

    # Generate language-specific parameters
    rust_code.append("impl Default for MultilingualSimpleMethodParameters {")
    rust_code.append("    fn default() -> Self {")
    rust_code.append("        let mut language_params = HashMap::new();")
    rust_code.append("")

    # Add language-specific parameters
    for lang_key, lang_params in params_data["language_params"].items():
        rust_code.append(f"        // {lang_key}")
        rust_code.append("        language_params.insert(")
        rust_code.append(f'            "{lang_key}".to_string(),')
        rust_code.append("            MultilingualSimpleParameters {")
        rust_code.append(f"                coefficient: {format_f32(lang_params['coefficient'])},")
        rust_code.append("            },")
        rust_code.append("        );")
        rust_code.append("")

    rust_code.append("        Self {")
    rust_code.append("            default_params: MultilingualSimpleParameters::default(),")
    rust_code.append("            language_params,")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")

    return "\n".join(rust_code)


def update_rust_file(rust_file_path: str, new_params_code: str, param_type: str) -> None:
    """Update the Rust file with new parameters."""
    with open(rust_file_path, "r") as f:
        content = f.read()

    # Define markers based on parameter type
    if param_type == "simple":
        start_marker = "impl Default for SimpleParameters {"
        end_marker = "impl Default for SimpleParameters {"
        single_block = True
    elif param_type == "basic":
        start_marker = "impl Default for BasicParameters {"
        end_marker = "impl Default for BasicParameters {"
        single_block = True
    elif param_type == "multilingual":
        start_marker = "impl Default for MultilingualParameters {"
        end_marker = "impl Default for MultilingualMethodParameters {"
        single_block = False
    elif param_type == "multilingual_simple":
        start_marker = "impl Default for MultilingualSimpleParameters {"
        end_marker = "impl Default for MultilingualSimpleMethodParameters {"
        single_block = False
    else:
        raise ValueError(f"Unknown parameter type: {param_type}")

    # Find positions
    start_pos = content.find(start_marker)
    if start_pos == -1:
        raise ValueError(f"Could not find start marker: {start_marker}")

    if single_block:
        # For simple and basic, we only have one impl block
        brace_count = 0
        pos = start_pos
        while pos < len(content):
            if content[pos] == "{":
                brace_count += 1
            elif content[pos] == "}":
                brace_count -= 1
                if brace_count == 0:
                    end_pos = pos + 1
                    break
            pos += 1
        else:
            raise ValueError("Could not find end of impl block")

        # Replace the content
        new_content = content[:start_pos] + new_params_code + content[end_pos:]
    else:
        # For multilingual types, we have two impl blocks
        end_marker_pos = content.find(end_marker)
        if end_marker_pos == -1:
            raise ValueError(f"Could not find end marker: {end_marker}")

        # Find the final closing brace of the second impl block
        brace_count = 0
        pos = end_marker_pos
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
            raise ValueError("Could not find end of second impl block")

        # Replace the content
        new_content = content[:start_pos] + new_params_code + content[final_end_pos:]

    # Write back to file
    with open(rust_file_path, "w") as f:
        f.write(new_content)


def main() -> None:
    # Define paths
    project_root = Path(__file__).parent.parent

    # Define parameter configurations
    param_configs: list[ParamConfig] = [
        {
            "name": "simple",
            "toml_path": project_root / "params" / "simple.toml",
            "rust_path": project_root / "src" / "methods" / "method_simple.rs",
            "generator": generate_simple_params_rust,
        },
        {
            "name": "basic",
            "toml_path": project_root / "params" / "basic.toml",
            "rust_path": project_root / "src" / "methods" / "method_basic.rs",
            "generator": generate_basic_params_rust,
        },
        {
            "name": "multilingual",
            "toml_path": project_root / "params" / "multilingual.toml",
            "rust_path": project_root / "src" / "methods" / "method_multilingual.rs",
            "generator": generate_multilingual_params_rust,
        },
        {
            "name": "multilingual_simple",
            "toml_path": project_root / "params" / "multilingual_simple.toml",
            "rust_path": project_root / "src" / "methods" / "method_multilingual_simple.rs",
            "generator": generate_multilingual_simple_params_rust,
        },
    ]

    # Process each parameter type
    for config in param_configs:
        print(f"\nProcessing {config['name']} parameters...")
        print(f"  Loading from: {config['toml_path']}")
        print(f"  Updating: {config['rust_path']}")

        try:
            # Load parameters
            params_data = load_params_from_toml(str(config["toml_path"]))

            # Generate Rust code
            rust_code = config["generator"](params_data)

            # Update Rust file
            update_rust_file(str(config["rust_path"]), rust_code, config["name"])

            # Report success
            if "language_params" in params_data:
                print(
                    f"  ✓ Updated {len(params_data['language_params'])} language-specific parameter sets"
                )
            else:
                print("  ✓ Updated parameters successfully")

        except Exception as e:
            print(f"  ✗ Error processing {config['name']}: {e}")
            continue

    print("\nAll parameter updates completed!")
    print("\nRemember to run:")
    print("  cargo fmt")
    print("  cargo build")


if __name__ == "__main__":
    main()
