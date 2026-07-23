```markdown
# mistral.rs Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill teaches you the core development patterns and conventions used in the `mistral.rs` Rust repository. You'll learn about file naming, import/export styles, commit conventions, and how to write and run tests in this codebase. This guide helps maintain consistency and efficiency when contributing to `mistral.rs`.

## Coding Conventions

### File Naming
- Use **snake_case** for all file names.
  - Example: `my_module.rs`, `data_processor.rs`

### Import Style
- Use **relative imports** within the crate.
  - Example:
    ```rust
    mod utils;
    use crate::utils::helper_function;
    ```

### Export Style
- Use **named exports** to expose specific items.
  - Example:
    ```rust
    pub fn important_function() { /* ... */ }
    pub struct MyStruct { /* ... */ }
    ```

### Commit Messages
- Follow **conventional commit** format.
- Use the `feat` prefix for new features.
  - Example: `feat: add support for custom tokens`

## Workflows

### Feature Development
**Trigger:** When adding a new feature to the codebase  
**Command:** `/feature-development`

1. Create a new branch for your feature.
2. Implement your feature using snake_case file naming and relative imports.
3. Export new functions or structs using named exports.
4. Write or update tests in files matching `*.test.*`.
5. Commit your changes using the `feat` prefix and a concise message.
6. Open a pull request for review.

### Testing
**Trigger:** When you need to verify code correctness  
**Command:** `/run-tests`

1. Identify or create test files using the `*.test.*` pattern.
2. Write tests for your modules and functions.
3. Run the test suite using the appropriate Rust test command (e.g., `cargo test`).
4. Review test results and fix any failing cases.

## Testing Patterns

- Test files follow the `*.test.*` naming pattern (e.g., `math.test.rs`).
- Testing framework is not explicitly specified; use Rust's built-in test framework.
- Example test:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_addition() {
          assert_eq!(add(2, 3), 5);
      }
  }
  ```

## Commands
| Command              | Purpose                                    |
|----------------------|--------------------------------------------|
| /feature-development | Guide for adding a new feature             |
| /run-tests           | Steps to write and execute tests           |
```
