# DRM key count is invalid

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-3f5aee2e37fe7b27

## Log pattern

```text
invalid key count value=%s (must be from 1 to 100)
```

## Parameters

- Argument 1 — Key count text: Configured key-count text.

The implementation excludes 100 despite the inclusive range suggested by the original message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid key count value=100 (must be from 1 to 100)
```

## Short description

DRM key count is invalid.

## Trigger condition

The parsed key count is zero, negative, or at least 100.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- The parsed key count is zero, negative, or at least 100.

## What to check

1. Use a key count from 1 through 99.

## Recommended action

Use a key count from 1 through 99.
