# Media worker thread started

Product: Nimble Streamer
Component: Media worker lifecycle
Logging function: log_d
Log level: debug
Documentation ID: log_d-94ef9debafb0d06d

## Log pattern

```text
started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
started
```

## Short description

Media worker thread started.

## Trigger condition

A media sender or camera worker has entered its thread routine, before initializing its loop timers and processing work.

## Explanation

The worker proceeds to its event and maintenance loop. This message is shared by general client senders, MPEG-TS senders, UDP constant-bitrate senders, SRT senders and servers, SRT listen and pull/rendezvous inputs, and Zixi inputs. It confirms entry into the worker routine; successful media connections and delivery are established by subsequent events. SRT variants require SRT support, and the UDP and SRT constant-bitrate variants are available on non-Windows builds.

## Possible causes

- A media sender or camera worker has entered its thread routine, before initializing its loop timers and processing work.

## What to check

1. Treat this as a normal worker lifecycle event. If media is unavailable afterward, check the subsequent connection, configuration, and transfer messages for the affected input or output.

## Recommended action

Treat this as a normal worker lifecycle event. If media is unavailable afterward, check the subsequent connection, configuration, and transfer messages for the affected input or output.

## Related log messages

- [Media worker thread stopped](log_d-8cfa90e45067858b.md)
