# Panel JSON member inspected

Product: Nimble Streamer
Component: Stream checking and statistics recovery
Logging function: log_d
Log level: debug
Documentation ID: log_d-967fbacaec4f327e

## Log pattern

```text
+ %s: %s
```

## Parameters

- Argument 1 — member\_name: JSON member key at the current parser level.
- Argument 2 — json\_value: JSON serialization of its value; strings retain JSON quotation marks.

## Log examples

Illustrative examples; these are not captured server logs.

```text
+ status: "success"
```

## Short description

Panel JSON member inspected.

## Trigger condition

The stream-checker response parser visits a top-level or nested stream-response member, or the recovery-response parser visits a top-level member.

## Explanation

The key and JSON-serialized value are printed before the member is interpreted. This trace alone does not establish that a recognized or successful status was found.
