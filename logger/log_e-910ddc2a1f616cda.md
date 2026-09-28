# Invalid TLS server-name regular expression

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-910ddc2a1f616cda

## Log pattern

```text
exception caught while processing server_name regexp='%s' what()='%s'
```

## Parameters

- Argument 1 — Pattern: Server-name regular expression after removing its leading marker.
- Argument 2 — Error: Regular-expression exception description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
exception caught while processing server_name regexp='[' what()='Unmatched bracket'
```

## Short description

Invalid TLS server-name regular expression.

## Trigger condition

Compiling a server-name pattern prefixed as a regular expression throws an exception.

## Explanation

Certificate-context creation fails before loading the certificate. The same check is used for ordinary TLS and HTTP/3 when enabled.

## Possible causes

- Compiling a server-name pattern prefixed as a regular expression throws an exception.

## What to check

1. Check the displayed pattern and exception text.

## Recommended action

Correct the server-name regular expression.
