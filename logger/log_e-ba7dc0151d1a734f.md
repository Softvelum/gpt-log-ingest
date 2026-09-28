# Invalid local address for SRT bind

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-ba7dc0151d1a734f

## Log pattern

```text
failed to get ip address from %s, s=%d
```

## Parameters

- Argument 1 — Address: Rejected local IP-address string.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get ip address from invalid-address, s=100
```

## Short description

Invalid local address for SRT bind.

## Trigger condition

The configured local bind address cannot be parsed in the IPv4 or IPv6 family selected for the connection.

## Explanation

Local binding fails before the library bind call.

## Possible causes

- The configured local bind address cannot be parsed in the IPv4 or IPv6 family selected for the connection.

## What to check

1. Check that the local address is a numeric address in the expected family.

## Recommended action

Correct the local bind address.
