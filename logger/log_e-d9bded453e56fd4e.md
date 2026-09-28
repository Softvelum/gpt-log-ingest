# WHIP control send count inconsistent

Product: Nimble Streamer
Component: WHIP publishing helper
Logging function: log_e
Log level: error
Documentation ID: log_e-d9bded453e56fd4e

## Log pattern

```text
whip control socket sent more that requested. impossible
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
whip control socket sent more that requested. impossible
```

## Short description

WHIP control send count inconsistent.

## Trigger condition

A control-socket send reports more bytes written than were present in the pending buffer.

## Explanation

The control socket is closed, pending starts are marked finished, pending data is cleared, and active WHIP publications are removed and their stream registrations released.

## Possible causes

- The send result violates the buffer-length contract.

## What to check

1. Capture surrounding helper/socket diagnostics and the conditions that preceded the inconsistent count.

## Recommended action

Treat a repeat as an internal transport defect and provide diagnostic evidence for investigation.
