# Listener IPv4 address could not be parsed

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-6a2cf21a55bd993b

## Log pattern

```text
unable to convert server listen address to network byte order: %s
```

## Parameters

- Argument 1 — Listen address: Configured interface-address text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to convert server listen address to network byte order: invalid-address
```

## Short description

Listener IPv4 address could not be parsed.

## Trigger condition

An explicit non-wildcard address is selected for the IPv4 socket branch and address conversion fails.

## Explanation

The process exits with failure before binding this listener.

## Possible causes

- The selected address is not accepted as an IPv4 address.

## What to check

1. Check the interface address and the selected IP family.

## Recommended action

Use a valid address for the enabled listening family.
