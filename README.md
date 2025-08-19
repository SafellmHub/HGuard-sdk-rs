#HallucinationGuard
Go Report Card GoDoc License: MIT

HallucinationGuard is a Rust Crate for validating and enforcing guardrails on LLM tool calls. It provides schema validation, policy enforcement, and extensibility for production-grade AI integrations.

status: experimental

⚠️ Experimental Notice

This package is currently experimental and still under active development.

We welcome your feedback and encourage you to report issues or suggest improvements.

Features
Schema Validation: Structured validation of tool calls against JSON schemas
Context-Aware Policies: Role-based, time-based, and session-based policy enforcement
Conditional Logic: Complex conditional expressions for advanced policy rules
Policy Priority: Hierarchical policy system with priority-based rule resolution
Auto-Correction: Automatic tool name correction for common typos
Thread-Safe: Safe for concurrent use in production environments
Extensible: Custom schema loaders and policy engines