# Negative or rejected origin content length

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_e
Log level: error
Documentation ID: log_e-ff06ce704cd87a80

## Log pattern

```text
invalid content-length value in origin resp='%s'
```

## Parameters

- Argument 1 — Value: Original Content-Length header text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid content-length value in origin resp='-1'
```

## Short description

Negative or rejected origin content length.

## Trigger condition

Converting the origin Content-Length produces a negative value, or a zero value accompanied by a conversion error.

## Explanation

The numeric content-length accessor reports failure without returning a length. The check does not reject every possible malformed decimal string.

## Possible causes

- Converting the origin Content-Length produces a negative value, or a zero value accompanied by a conversion error.

## What to check

1. Check the exact Content-Length header.

## Recommended action

Correct the origin response to provide a valid nonnegative content length.
