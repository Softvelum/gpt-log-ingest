# Advertising rule added

Product: Nimble Streamer
Component: Advertising
Logging function: log_i
Log level: info
Documentation ID: log_i-7d90c5c89b0e6cf1

## Log pattern

```text
added advertising rule id='%s'
```

## Parameters

- Argument 1 — Rule ID: Text identifying the advertising rule.

## Log examples

Illustrative examples; these are not captured server logs.

```text
added advertising rule id='rule-01'
```

## Short description

Advertising rule added.

## Trigger condition

A global or session rule update contains a new rule identifier.

## Explanation

The rule is inserted into the corresponding rule collection.

## Possible causes

- A global or session rule update contains a new rule identifier.
