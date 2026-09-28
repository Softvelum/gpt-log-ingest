# TLS context released with zero references

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-0d0455af9ca9a85f

## Log pattern

```text
releasing SSLCtx p=%p ref_count=%u
```

## Parameters

- Argument 1 — Context: Address identifying the cached context.
- Argument 2 — References: Tracked reference count; this branch prints zero.

## Log examples

Illustrative examples; these are not captured server logs.

```text
releasing SSLCtx p=0x12345000 ref_count=0
```

## Short description

TLS context released with zero references.

## Trigger condition

An active certificate context is released when its tracked reference count is already zero.

## Explanation

The count is left at zero and the context remains in the active cache. The release operation reports that the context was found.

## Possible causes

- An active certificate context is released when its tracked reference count is already zero.

## What to check

1. Check context acquire/release balance and nearby certificate reloads.
