# Listener socket creation failed

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-98b1a1b6e9ee7b6e

## Log pattern

```text
listener socket creation failed errno=%d
```

## Parameters

- Argument 1 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

Main server setup can request TCP or UDP sockets; this message is not limited to IPv4 TCP.

## Log examples

Illustrative examples; these are not captured server logs.

```text
listener socket creation failed errno=5
```

## Short description

A listening or server receive socket could not be created.

## Trigger condition

Creating a generic IPv4 listener, a WHIP/WHEP IPv4 control listener, or a main server IPv4/IPv6 socket fails.

## Explanation

Generic and WebRTC control listener setup returns failure. Main server listener setup exits the process with failure.

## Possible causes

- The operating system rejected socket creation.

## What to check

1. Check the socket error and available socket resources.

## Recommended action

Resolve the reported socket creation failure.
