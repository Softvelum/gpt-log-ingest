# Routing rules replaced

Product: Nimble Streamer
Component: Request routing
Logging function: log_d
Log level: debug
Documentation ID: log_d-e1f5937d1d4d44af

## Log pattern

```text
replaced rules with a new set hash='%s'
```

## Parameters

- Argument 1 — Rules hash: Text identifying the applied routing-rule version.

## Log examples

Illustrative examples; these are not captured server logs.

```text
replaced rules with a new set hash='abc123'
```

## Short description

Routing rules replaced.

## Trigger condition

A new set of routing rules and its version hash have replaced the previous set.

## Explanation

New rule and origin identifiers are assigned next. Remote HTTPS file origins are then marked for TLS use.
