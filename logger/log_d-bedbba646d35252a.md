# Scenario transcoder instance created

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_d
Log level: debug
Documentation ID: log_d-bedbba646d35252a

## Log pattern

```text
created scenario transcoder, id=%s
```

## Parameters

- Argument 1 — Scenario: Transcoding scenario identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created scenario transcoder, id=scenario1
```

## Short description

Scenario transcoder instance created.

## Trigger condition

A new scenario transcoder controller has been constructed.

## Explanation

The controller exists; its child process and connection are created in later steps.
