# aeo-cli

Command-line tool for the [AEO Protocol v0.1](https://github.com/mizcausevic-dev/aeo-protocol-spec). Validate, fetch, inspect, and extract claims from AEO declaration documents.

Built on top of [aeo-sdk-rust](https://github.com/mizcausevic-dev/aeo-sdk-rust).

## Install

```bash
cargo install --git https://github.com/mizcausevic-dev/aeo-cli
```

This installs an `aeo` binary on your `PATH`.

## Usage

```
aeo --help

Validate, fetch, and inspect AEO Protocol declaration documents.

Spec: https://github.com/mizcausevic-dev/aeo-protocol-spec

Usage: aeo <COMMAND>

Commands:
  validate  Validate a local AEO document file against the v0.1 schema
  fetch     Fetch and pretty-print the AEO declaration at an origin's well-known URL
  inspect   Show a structured summary of an AEO document
  claim     Extract and print a specific claim by ID
  help      Print this message or the help of the given subcommand
```

## Examples

**Validate a local file:**
```bash
aeo validate ./aeo.json
# OK ./aeo.json — Miz Causevic (6 claims)
```

**Fetch a live declaration:**
```bash
aeo fetch https://mizcausevic-dev.github.io
# {
#   "aeo_version": "0.1",
#   "entity": { "name": "Miz Causevic", ... },
#   ...
# }
```

**Inspect (works on either a path or a URL):**
```bash
aeo inspect https://mizcausevic-dev.github.io
# Protocol:            0.1
# Entity:              Person
# Name:                Miz Causevic
# ID:                  https://mizcausevic-dev.github.io/#person
# Canonical URL:       https://mizcausevic-dev.github.io/
# Primary sources:     4
# Verifications:       3
# Claims:              6
#   - current-role             jobTitle                                 High
#   - location                 address                                  High
#   - years-experience         aeo:yearsOfExperience                    High
#   - live-products            aeo:operates                             High
#   - primary-stack            aeo:primaryLanguageStack                 High
#   - authored-spec            aeo:authoredSpecification                High
# Audit mode:          none
```

**Extract a specific claim:**
```bash
aeo claim https://mizcausevic-dev.github.io years-experience
# {
#   "id": "years-experience",
#   "predicate": "aeo:yearsOfExperience",
#   "value": 30,
#   "confidence": "high"
# }
```

## Conformance

Supports AEO Protocol v0.1 at **conformance Level 1 (Declare)**. Signature verification (L2) and audit-endpoint reporting (L3) deferred to v0.2.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | Error (parse failure, network failure, missing claim, etc.) |

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
cargo test
```

Integration tests build the binary and invoke it as a subprocess; tests cover validate (happy + malformed), inspect, claim (found + missing).

## Specification

Full spec at [github.com/mizcausevic-dev/aeo-protocol-spec](https://github.com/mizcausevic-dev/aeo-protocol-spec).

## License

AGPL-3.0.

## Kinetic Gain Protocol Suite

| Spec | Implementation |
|---|---|
| [AEO Protocol](https://github.com/mizcausevic-dev/aeo-protocol-spec) | [aeo-sdk-python](https://github.com/mizcausevic-dev/aeo-sdk-python) · [aeo-sdk-typescript](https://github.com/mizcausevic-dev/aeo-sdk-typescript) · [aeo-sdk-rust](https://github.com/mizcausevic-dev/aeo-sdk-rust) · [aeo-sdk-go](https://github.com/mizcausevic-dev/aeo-sdk-go) · **aeo-cli** (this) · [aeo-crawler](https://github.com/mizcausevic-dev/aeo-crawler) |
| [Prompt Provenance](https://github.com/mizcausevic-dev/prompt-provenance-spec) | — |
| [Agent Cards](https://github.com/mizcausevic-dev/agent-cards-spec) | — |
| [AI Evidence Format](https://github.com/mizcausevic-dev/ai-evidence-format-spec) | — |
| [MCP Tool Cards](https://github.com/mizcausevic-dev/mcp-tool-card-spec) | — |

---

**Connect:** [LinkedIn](https://www.linkedin.com/in/mirzacausevic/) · [Kinetic Gain](https://kineticgain.com) · [Medium](https://medium.com/@mizcausevic/) · [Skills](https://mizcausevic.com/skills/)
