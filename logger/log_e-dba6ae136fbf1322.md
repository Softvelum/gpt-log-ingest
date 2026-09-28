# Peer address could not be resolved

Product: Nimble Streamer
Component: Host address resolution
Logging function: log_e
Log level: error
Documentation ID: log_e-dba6ae136fbf1322

## Log pattern

```text
failed to resolve ip address from %s
```

## Parameters

- Argument 1 — Host: Requested hostname or address text.

The same text covers an unsupported resolved family and failure of the IPv4-only lookup.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to resolve ip address from example.invalid
```

## Short description

Peer address could not be resolved.

## Trigger condition

With IPv6 enabled, name resolution returns an unsupported address family; with IPv6 disabled, IPv4 lookup fails or produces no single selected address.

## Explanation

Peer-address preparation reports failure.

## Possible causes

- No accepted IPv4 or IPv6 peer address is available.

## What to check

1. Check IPv6 enablement, the hostname, and preceding resolver messages.

## Recommended action

Provide a name that resolves to a supported address family.
