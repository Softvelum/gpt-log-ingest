# Component-specific splice timing is unsupported

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-cbdf45b4f71c107b

## Log pattern

```text
splice_insert() with multiple components pts currently unsupported
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
splice_insert() with multiple components pts currently unsupported
```

## Short description

Component-specific splice timing is unsupported.

## Trigger condition

A splice-insert command has its program-splice flag clear and a component count greater than zero.

## Explanation

Splice-insert parsing fails; even a single listed component enters this unsupported path.

## Possible causes

- The marker uses component-specific timing instead of supported program timing.

## What to check

1. Check how the upstream source constructs its splice-insert markers.

## Recommended action

Use program-level splice timing supported by this reader.
