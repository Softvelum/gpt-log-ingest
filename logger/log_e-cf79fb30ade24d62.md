# UDP input cannot join a multicast group

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_e
Log level: error
Documentation ID: log_e-cf79fb30ade24d62

## Log pattern

```text
failed to configure multicast(add_membership) s=%d '%s:%d on %s' [%d]
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.
- Argument 2 — Group: Configured multicast IPv4 address.
- Argument 3 — Port: Input UDP port.
- Argument 4 — Interface: Explicit or inherited interface IPv4 address; empty selects the default interface.
- Argument 5 — Error: Operating-system socket error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to configure multicast(add_membership) s=42 '239.1.1.1:5000 on 192.0.2.10' [19]
```

## Short description

UDP input cannot join a multicast group.

## Trigger condition

Binding succeeds but joining the configured multicast group without a source filter fails.

## Explanation

UDP input connection setup returns failure.

## Possible causes

- Binding succeeds but joining the configured multicast group without a source filter fails.

## What to check

1. Check group address, port, interface selection, and socket error.

## Recommended action

Correct multicast/interface configuration or resolve the reported socket error.
