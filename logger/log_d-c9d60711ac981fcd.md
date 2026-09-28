# Scenario transcoder process started

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_d
Log level: debug
Documentation ID: log_d-c9d60711ac981fcd

## Log pattern

```text
started scenario transcoder process: %s
```

## Parameters

- Argument 1 — Command: Complete child-process command line, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
started scenario transcoder process: nimble_scenario_transcoder --ip=127.0.0.1 --port=45000 --id=scenario1 --config-file=nimble.conf --nimble-logging=info
```

## Short description

Scenario transcoder process started.

## Trigger condition

Creating the scenario transcoder child process succeeds.

## Explanation

The child has started with the logged connection, scenario, configuration-file and logging arguments; its connection is established separately.

## Related log messages

- [Scenario transcoder connection setup began](log_d-42127c04adc540af.md)
- [Scenario transcoder connection is ready](log_i-596f2077a8da104d.md)
