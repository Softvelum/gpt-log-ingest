# Reported application output rate

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_v
Log level: verbose
Documentation ID: log_v-f177bc6eb0f8c6f0

## Log pattern

```text
app='%s' outrate=%llu
```

## Parameters

- Argument 1 — App: Name of the statistics record as text.
- Argument 2 — Output rate: Unsigned 64-bit average sent-byte rate in bytes per second.

The rate uses a fixed-width 64-bit type while the conversion expects unsigned long long; exact underlying type compatibility depends on the build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
app='camera' outrate=125000
```

## Short description

Reported application output rate.

## Trigger condition

A application record is copied into the statistics snapshot.

## Explanation

When elapsed time is positive, sent bytes are divided by elapsed seconds to compute the snapshot rate. If time has not advanced, this snapshot retains its default zero rate. Byte counters and the last-report time are reset after logging.
