# Dependency Management and Supply Chain Security 🟢

> **What you'll learn:**
> - Scanning for known vulnerabilities with `cargo-audit`
> - Enforcing license, advisory, and source policies with `cargo-deny`
> - Supply chain trust verification with Mozilla's `cargo-vet`
> - Tracking outdated dependencies and detecting breaking API changes
> - Visualizing and deduplicating your dependency tree
>
> **Cross-references:** [Release Profiles](ch07-release-profiles-and-binary-size.md) — `cargo-udeps` trims unused dependencies found here · [CI/CD Pipeline](ch11-putting-it-all-together-a-production-cic.md) — audit and deny jobs in the pipeline · [Build Scripts](ch01-build-scripts-buildrs-in-depth.md) — `build-dependencies` are part of your supply chain too

A Rust binary doesn't just contain your code — it contains every transitive
dependency in your `Cargo.lock`. A vulnerability, license violation, or
malicious crate anywhere in that tree becomes *your* problem. This chapter
covers the tools that make dependency management auditable and automated.

### cargo-audit — Known Vulnerability Scanning

[`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
checks your `Cargo.lock` against the [RustSec Advisory Database](https://rustsec.org/).

```bash
# Install and scan
cargo install cargo-audit
cargo audit
```

### cargo-deny — Comprehensive Policy Enforcement

[`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) enforces policies across four dimensions:
1. **Advisories**: Known vulnerabilities (like cargo-audit).
2. **Licenses**: Allowed/denied license list.
3. **Bans**: Forbidden crates or duplicate versions.
4. **Sources**: Allowed registries and git sources.

```bash
# Initialize and check
cargo deny init
cargo deny check
```

### cargo-vet — Supply Chain Trust Verification

[`cargo-vet`](https://github.com/mozilla/cargo-vet) is used for manual audit
certification of your dependencies. It answers: "has a trusted human actually
reviewed this code?"

### cargo-tree — Visualization and Deduplication

`cargo tree` is built into Cargo. It helps you understand why a crate is in
your tree and identify duplicate versions.

```bash
# Find why a crate is included
cargo tree --invert --package openssl-sys

# Find duplicate versions
cargo tree --duplicates
```

### 🏋️ Exercises

#### 🟢 Exercise 1: Audit Your Dependencies

Run `cargo audit` and `cargo deny init && cargo deny check` on any Rust project. How many advisories are found?

#### 🟡 Exercise 2: Find and Eliminate Duplicate Dependencies

Run `cargo tree --duplicates` on a workspace. Find a crate that appears at two versions. Can you update `Cargo.toml` to unify them?

### Key Takeaways

- `cargo audit` catches known CVEs — run it on every push.
- `cargo deny` enforces licenses, bans, and sources.
- Use `[workspace.dependencies]` to centralize version management.
- `cargo tree --duplicates` reveals bloat and potential version drift.

***
