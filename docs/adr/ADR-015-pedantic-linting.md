# ADR-015: Pedantic Clippy Linting

**Status:** Accepted
**Date:** 2025-12-17
**Deciders:** Louis C. Tavares, Claude Opus 4.5

## Context

DANEEL implements a cognitive architecture where code quality directly impacts system reliability.
For AI alignment work, bugs are unacceptable - a misaligned AI due to a software bug would be catastrophic.

We need a linting strategy that:

1. Catches bugs before they reach production
2. Enforces consistent code style across contributors (human and AI)
3. Makes code review faster (linter catches mechanical issues)
4. Produces maintainable, idiomatic Rust

## Decision

Enable **pedantic Clippy linting** via `Cargo.toml` configuration:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }
# Allow common patterns that don't improve code clarity
missing_errors_doc = "allow"
missing_panics_doc = "allow"
module_name_repetitions = "allow"
# Allow for cognitive architecture patterns
too_many_lines = "allow"
cognitive_complexity = "allow"
```

### Lint Levels Explained

| Level | Purpose |
|-------|---------|
| `pedantic` | Stricter lints that may have false positives but catch real bugs |
| `nursery` | Newer lints being tested - early adoption finds issues |
| `cargo` | Cargo.toml best practices (metadata, features) |

### Allowed Exceptions

| Category | Lint | Reason |
|----------|------|--------|
| **Documentation** | `missing_errors_doc` | Error types are self-documenting via `thiserror` |
| | `missing_panics_doc` | Panics are architectural invariants (THE BOX) |
| | `doc_markdown` | Bold actor names are intentional style |
| **Naming** | `module_name_repetitions` | `MemoryActor::MemoryMessage` is clearer |
| | `similar_names` | TMI domain terms: `content`/`context` |
| **Code Style** | `must_use_candidate` | Too noisy, add manually when needed |
| | `missing_const_for_fn` | Will upgrade when stable |
| | `option_if_let_else` | if/else often more readable than `map_or_else` |
| | `manual_let_else` | Explicit match is clearer than `let...else` |
| | `uninlined_format_args` | Older format style is fine |
| | `redundant_else` | Explicit else blocks are clearer |
| | `if_not_else` | Negative conditions can be clearer |
| | `use_self` | Explicit type names can be clearer |
| | `match_same_arms` | Explicit arms document intent |
| | `suboptimal_flops` | `mul_add` is micro-optimization |
| **Types** | `derive_partial_eq_without_eq` | `Eq` not always needed |
| | `needless_pass_by_value` | Actor messages often move values |
| | `cast_precision_loss` | f32 salience scores are intentional |
| | `trivially_copy_pass_by_ref` | Micro-optimization not worth API change |
| **Collections** | `useless_vec` | Prefer consistency |
| | `needless_collect` | Explicit collect often clearer |
| **Tests** | `float_cmp` | Exact float comparisons in tests are fine |
| **Async** | `future_not_send` | Will address when needed |
| **Cognitive** | `too_many_lines` | Cognitive actors may need long implementations |
| | `cognitive_complexity` | TMI logic can be inherently complex |
| **Dependencies** | `multiple_crate_versions` | Transitive deps we don't control |

## CI Integration

The CI workflow enforces zero warnings:

```yaml
- name: Clippy
  run: cargo clippy --all-targets --all-features -- -D warnings
```

The `-D warnings` flag treats all warnings as errors, ensuring no code with lint warnings can be merged.

## Consequences

**Positive:**

- Catches subtle bugs (unused results, type coercions, missing derives)
- Enforces idiomatic Rust patterns
- Makes AI-generated code more consistent
- Reduces code review burden
- Documents intent through allowed exceptions

**Negative:**

- Initial adoption requires fixing existing warnings
- Some false positives require `#[allow(...)]` annotations
- Stricter than default Clippy (learning curve)

## Comparison with Forge

This decision mirrors the linting strategy in RoyalBit Forge (now archived), which successfully used pedantic linting across 89% code coverage and 2,486 tests.

## References

- [Clippy Lint Levels](https://doc.rust-lang.org/clippy/lint_configuration.html)
- [Rust 2024 Edition Lints](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- RoyalBit Forge Cargo.toml (internal reference)
