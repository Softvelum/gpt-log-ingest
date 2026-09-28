# Connection names an unknown scenario

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_e
Log level: error
Documentation ID: log_e-19609eda074daaec

## Log pattern

```text
removed unknown sock=%d scenario=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Scenario field: A scenario text pointer is supplied to an integer conversion. The printed number is not a valid scenario identifier and has no reliable interpretation.

## Log examples

No reliable example can be provided: the scenario argument is a text pointer but the format requires an integer, so this call has undefined formatting behavior.

## Short description

Connection names an unknown scenario.

## Trigger condition

A complete connect message names a scenario that is not in the parent scenario map.

## Explanation

The socket is removed from event monitoring and closed, and unconnected children are checked for cleanup. The scenario field has an incompatible printf conversion and cannot be trusted as an identifier.

## Possible causes

- The connecting child names a scenario that has already been removed or is unknown to this parent.

## What to check

1. Correlate the preceding connecting message, which prints the actual scenario identifier, with scenario creation/removal.

## Recommended action

Correct stale or mismatched child connections and scenario identifiers.

## Related log messages

- [Scenario connection request received](log_d-c962e04495dd4381.md)
