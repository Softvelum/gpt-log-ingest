# Required IP address is invalid

Product: Nimble Streamer
Component: Host address resolution
Logging function: log_e
Log level: error
Documentation ID: log_e-b0b7cde7ff498676

## Log pattern

```text
failed to get ip address from %s
```

## Parameters

- Argument 1 — Host: Requested hostname or address text.

This applies both to generic IP-only address handling and to SRT listener addresses.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get ip address from example.invalid
```

## Short description

Required IP address is invalid.

## Trigger condition

An IP-only address request, or an SRT listener setup, receives text that is not an IPv4 address and is not an accepted IPv6 address when IPv6 is enabled.

## Explanation

Peer-address preparation or SRT listener creation fails before proceeding with the connection.

## Possible causes

- A hostname or invalid address was supplied where an IP literal is required, or IPv6 is disabled for an IPv6 literal.

## What to check

1. Check the address and IPv6 enablement.

## Recommended action

Use a valid IP literal for an enabled address family.
