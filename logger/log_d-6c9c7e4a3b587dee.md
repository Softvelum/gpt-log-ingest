# HTTP byte-range values parsed

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-6c9c7e4a3b587dee

## Log pattern

```text
parsed range %llu-%llu (size %llu)
```

## Parameters

- Argument 1 — Start: Parsed start value in bytes, or zero if omitted.
- Argument 2 — End: Parsed nonzero end value in bytes.
- Argument 3 — Size: Unsigned arithmetic end minus start plus one; meaningful as an inclusive size only for ordered explicit bounds.

The unsigned 64-bit bounds use long-long conversions; exact typedef compatibility is platform dependent.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsed range 0-1023 (size 1024)
```

## Short description

HTTP byte-range values parsed.

## Trigger condition

A bytes range is parsed and its ending value is nonzero.

## Explanation

The helper returns the parsed numeric bounds and their unsigned end-minus-start-plus-one result. This step does not validate ordering or resolve suffix-range meaning.

## Possible causes

- A bytes range is parsed and its ending value is nonzero.

## What to check

1. Check range ordering and later request validation, particularly for suffix ranges.
