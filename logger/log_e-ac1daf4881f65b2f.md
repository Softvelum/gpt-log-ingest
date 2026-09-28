# Scenario x264 library failed to load

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ac1daf4881f65b2f

## Log pattern

```text
failed to load x264 libs, scenario_id=%s
```

## Parameters

- Argument 1 — Scenario ID: Configured identifier of the scenario encoder process.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load x264 libs, scenario_id=scenario-1
```

## Short description

Scenario x264 library failed to load.

## Trigger condition

Loading the x264 encoding library returns failure during scenario startup.

## Explanation

The x264 library handle is unloaded, but startup continues to create the encoding worker. This message alone does not mean the entire scenario exits.

## Possible causes

- Loading the x264 encoding library returns failure during scenario startup.

## What to check

1. Check whether the scenario needs x264 and whether that library can be loaded.

## Related log messages

- [Scenario media libraries failed to initialize](log_e-fe146f192049e048.md)
