# IP range name is already present

Product: Nimble Streamer
Component: IP access ranges
Logging function: log_e
Log level: error
Documentation ID: log_e-87c5beed55158f5b

## Log pattern

```text
range='%s' already exists
```

## Parameters

- Argument 1 — Range name: Text identifying the named IP range group.

## Log examples

Illustrative examples; these are not captured server logs.

```text
range='viewers' already exists
```

## Short description

IP range name is already present.

## Trigger condition

While applying the supplied range settings, a range name is found in the destination map before insertion.

## Explanation

That named definition is skipped. The destination is cleared before the update and the settings are provided as a map, so the shown normal update path does not establish how duplicate names would arise.

## What to check

1. Check whether the same named range is being applied more than once.
