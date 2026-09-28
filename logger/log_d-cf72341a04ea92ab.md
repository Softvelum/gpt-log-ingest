# UDP input read would block

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_d
Log level: debug
Documentation ID: log_d-cf72341a04ea92ab

## Log pattern

```text
recv error=%d (EAGAIN | EWOULDBLOCK) on s=%d (udp)
```

## Parameters

- Argument 1 — Error: Operating-system socket error number.
- Argument 2 — Socket: Numeric socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
recv error=11 (EAGAIN | EWOULDBLOCK) on s=42 (udp)
```

## Short description

UDP input read would block.

## Trigger condition

A receive call returns the platform EAGAIN or EWOULDBLOCK error.

## Explanation

The receiver returns success and waits for later data.

## Possible causes

- A receive call returns the platform EAGAIN or EWOULDBLOCK error.

## What to check

1. Check whether normal datagrams resume.

## Recommended action

No action is needed for an isolated would-block event.
