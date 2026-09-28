# TLS context deleted with outstanding references

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-df847de4ce39ccfe

## Log pattern

```text
deleting SSL ctx=%p ref_count=%lld
```

## Parameters

- Argument 1 — Context: Address identifying the cached certificate context.
- Argument 2 — References: Intended outstanding reference count; its type does not match the format.

The reference count is unsigned 32-bit but the format requests a signed long long.

## Log examples

The mismatched reference-count conversion prevents a portable sound example.

## Short description

TLS context deleted with outstanding references.

## Trigger condition

During cache destruction, an active or retired certificate context still has a nonzero reference count.

## Explanation

The context is freed despite the outstanding count. This records a lifecycle inconsistency during cache destruction.

## Possible causes

- During cache destruction, an active or retired certificate context still has a nonzero reference count.

## What to check

1. Check whether TLS sessions were released before certificate-cache shutdown.
