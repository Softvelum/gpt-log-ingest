# Scenario output queue dropped frames

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_e
Log level: error
Documentation ID: log_e-70ad07f765ced5e2

## Log pattern

```text
oop transcoder dropped %d output frames scenario=%s
```

## Parameters

- Argument 1 — Dropped frames: Reported count of rejected output messages, 32-bit count printed as signed decimal; the message calls these frames.
- Argument 2 — Scenario: Transcoding scenario identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
oop transcoder dropped 12 output frames scenario=scenario1
```

## Short description

Scenario output queue dropped frames.

## Trigger condition

The parent receives a child report containing the accumulated count of messages rejected when its output queue was full.

## Explanation

The dropped output cannot be recovered by this report; the parent records the count and continues processing the connection. The count is attached to a subsequent accepted message after queue space becomes available.

## Possible causes

- The child produces output messages faster than its bounded queue can accept them.

## What to check

1. Check scenario output rate, parent/child processing load and configured interprocess message queue capacity.

## Recommended action

Reduce the output load or restore timely queue consumption; adjust queue capacity if suitable.
