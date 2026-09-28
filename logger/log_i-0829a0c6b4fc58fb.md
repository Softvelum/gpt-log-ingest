# Advertising rule removed

Product: Nimble Streamer
Component: Advertising
Logging function: log_i
Log level: info
Documentation ID: log_i-0829a0c6b4fc58fb

## Log pattern

```text
removed advertising rule id='%s'
```

## Parameters

- Argument 1 — Rule ID: Text identifying the advertising rule.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed advertising rule id='rule-01'
```

## Short description

Advertising rule removed.

## Trigger condition

A global or session rule update omits an existing rule identifier.

## Explanation

The old rule is removed from the corresponding rule collection.

## Possible causes

- A global or session rule update omits an existing rule identifier.
