# Contributing to Stellar-Save

Thank you for your interest in contributing to Stellar-Save! This document covers everything you need to know to contribute effectively.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Code Style Guidelines](#code-style-guidelines)
- [Commit Message Conventions](#commit-message-conventions)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for everyone.

### Our Standards

**Expected behavior:**
- Use welcoming and inclusive language
- Respect differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy toward other contributors

**Unacceptable behavior:**
- Harassment, discrimination, or offensive comments of any kind
- Personal or political attacks
- Publishing others' private information without consent
- Trolling or deliberately disruptive behavior

### Enforcement

Violations may be reported to the maintainers via [GitHub Issues](https://github.com/Xoulomon/Stellar-Save/issues) or by contacting [@Xoulomon] directly. All reports will be reviewed and investigated. Maintainers reserve the right to remove, edit, or reject contributions that violate this Code of Conduct.

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/Stellar-Save.git
   cd Stellar-Save
   ```
3. **Set upstream** remote:
   ```bash
   git remote add upstream https://github.com/Xoulomon/Stellar-Save.git
   ```
4. **Install prerequisites:**
   - [Rust](https://www.rust-lang.org/tools/install) (1.70+)
   - [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
   - [Node.js](https://nodejs.org/) (18+) for frontend work
5. **Create a branch** for your work (see [Pull Request Process](#pull-request-process))

---

## Code Style Guidelines

### Rust (Smart Contract)

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing — all code must be formatted
- Run `cargo clippy` and resolve all warnings before opening a PR
- Use descriptive variable and function names
- Every public function **must** have a doc comment (`///`) covering:
  - What the function does
  - All arguments (`# Arguments`)
  - Return value (`# Returns`)
  - Possible errors (`# Errors`)
- Keep functions focused — one responsibility per function
- Prefer explicit error handling with `Result<T, StellarSaveError>` over panics
- Use `StorageKeyBuilder` for all storage key generation — never construct keys manually

```rust
// Good
/// Returns the current balance for a group.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `group_id` - Unique identifier of the group
///
/// # Returns
/// * `Ok(i128)` - Current balance in stroops
/// * `Err(StellarSaveError::GroupNotFound)` - If group does not exist
pub fn get_group_balance(env: Env, group_id: u64) -> Result<i128, StellarSaveError> {
    // ...
}

// Bad — no doc comment, vague name
pub fn get_bal(env: Env, id: u64) -> i128 {
    // ...
}
```

### TypeScript / React (Frontend)

- Follow the existing ESLint configuration (`npm run lint` must pass)
- Use functional components with hooks — no class components
- Type everything explicitly — avoid `any`
- Component files use PascalCase: `GroupCard.tsx`
- Utility/hook files use camelCase: `useWallet.ts`
- Keep components small and composable
- Use the existing MUI component library for UI elements

### General

- No commented-out code in PRs
- No `console.log` left in production code
- Keep lines under 100 characters where practical

---

## Commit Message Conventions

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation changes only |
| `test` | Adding or updating tests |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `chore` | Build process, dependency updates, tooling |
| `style` | Formatting changes (no logic change) |

### Scope (optional)

Use the area of the codebase affected: `contract`, `frontend`, `storage`, `security`, `payout`, etc.

### Examples

```
feat(contract): implement verify_signature function

docs: add contributing guidelines

fix(payout): correct recipient lookup in execute_payout

test(security): add edge case tests for verify_signature_bytes

chore: add ed25519-dalek dev dependency
```

### Rules

- Use the **imperative mood** in the description: "add feature" not "added feature"
- Keep the subject line under **72 characters**
- Reference issues in the footer: `Closes #42` or `Fixes #42`
- Do not end the subject line with a period

---

## Pull Request Process

### Before Opening a PR

1. **Sync with upstream** to avoid conflicts:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```
2. Ensure all tests pass: `cargo test`
3. Ensure formatting is clean: `cargo fmt --check`
4. Ensure no clippy warnings: `cargo clippy`
5. For frontend changes: `npm run lint` and `npm run build` must pass

### Branch Naming

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feature/<short-description>` | `feature/verify-signature` |
| Bug fix | `fix/<short-description>` | `fix/payout-recipient-lookup` |
| Documentation | `docs/<short-description>` | `docs/contributing-guidelines` |
| Tests | `test/<short-description>` | `test/security-edge-cases` |
| Refactor | `refactor/<short-description>` | `refactor/storage-keys` |

### Opening the PR

1. Push your branch to your fork:
   ```bash
   git push origin <your-branch>
   ```
2. Open a Pull Request against `Xoulomon/Stellar-Save:main`
3. Fill in the PR template completely:
   - **What** does this PR do?
   - **Why** is this change needed? (link the issue)
   - **How** was it tested?
4. Reference the related issue: `Closes #<issue-number>`
5. Assign relevant labels if you have access

### PR Review

- At least **one maintainer approval** is required before merging
- Address all review comments — either make the change or explain why not
- Keep the PR focused — one issue per PR
- Do not force-push after a review has started unless asked to
- PRs that have been inactive for 14 days may be closed

### After Merge

Delete your branch after it is merged:
```bash
git branch -d <your-branch>
git push origin --delete <your-branch>
```

---

## Testing Requirements

All contributions to the smart contract **must** include tests. PRs without adequate test coverage will not be merged.

### Rust Contract Tests

- Tests live in a `#[cfg(test)]` module at the bottom of the relevant source file
- Use `Env::default()` for the Soroban test environment
- Use `env.mock_all_auths()` when testing functions that require authorization
- Every public function needs at minimum:
  - A **happy path** test (valid inputs, expected output)
  - An **error path** test (invalid inputs return the correct `StellarSaveError`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        env.mock_all_auths();
        // ... test valid behavior
    }

    #[test]
    fn test_returns_error_on_invalid_input() {
        let env = Env::default();
        // ... test error case
        assert_eq!(result, Err(StellarSaveError::InvalidAmount));
    }
}
```

- Run the full test suite before submitting:
  ```bash
  cargo test
  ```

### Frontend Tests

- Use [Vitest](https://vitest.dev/) and [React Testing Library](https://testing-library.com/)
- New components should have at minimum a render test
- Run tests with:
  ```bash
  npm run test -- --run
  ```

### What Good Test Coverage Looks Like

- Happy path with valid inputs
- All documented error cases
- Boundary values (e.g. min/max amounts, empty lists)
- Edge cases specific to the feature (e.g. empty payload for signature verification)

---

## Questions?

If you're unsure about anything, open a [GitHub Discussion](https://github.com/Xoulomon/Stellar-Save/discussions) or comment on the relevant issue before starting work. We're happy to help.

**Built with ❤️ for financial inclusion on Stellar**
