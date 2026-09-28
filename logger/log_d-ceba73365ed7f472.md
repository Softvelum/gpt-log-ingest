# PAT discovers a new program-map PID

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-ceba73365ed7f472

## Log pattern

```text
PMT found pid=%d
```

## Parameters

- Argument 1 — PID: Newly discovered program-map packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
PMT found pid=100
```

## Short description

PAT discovers a new program-map PID.

## Trigger condition

A processed PAT section contains a nonzero program whose PMT PID is not yet tracked.

## Explanation

The PMT tracker and program mapping are created and the transport-information version changes. The PMT payload itself need not have arrived yet.

## Possible causes

- A processed PAT section contains a nonzero program whose PMT PID is not yet tracked.

## What to check

1. Compare the discovered PID with the PAT and subsequent PMT data.

## Recommended action

No action is needed for expected program discovery.
