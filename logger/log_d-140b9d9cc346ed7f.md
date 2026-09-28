# RTCP connection created

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-140b9d9cc346ed7f

## Log pattern

```text
rtcp open, s=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtcp open, s=25
```

## Short description

RTCP connection created.

## Trigger condition

The RTCP connection object is constructed.

## Explanation

The connection reports itself to its parent RTSP input and queues an initial eight-byte receiver report. This records creation, not proof that remote media has arrived.

## Possible causes

- Normal creation of a RTCP connection.

## Related log messages

- [RTCP connection destroyed](log_d-208553e3b3602bd3.md)
- [RTCP shutdown requested](log_d-84215053852f6ac9.md)
