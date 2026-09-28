# IP range lacks address and prefix parts

Product: Nimble Streamer
Component: IP access ranges
Logging function: log_e
Log level: error
Documentation ID: log_e-f7e3fe80b5a9f6bb

## Log pattern

```text
failed to split range='%s' to netmask parts
```

## Parameters

- Argument 1 — Range: Text containing the individual address range being parsed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to split range='192.0.2.0' to netmask parts
```

## Short description

IP range lacks address and prefix parts.

## Trigger condition

An IPv4 or IPv6 range fails to split into exactly two slash-separated parts.

## Explanation

The allocated range remains in its group with zero endpoints, and parsing continues with later ranges. This message does not mean the whole group was discarded.

## Possible causes

- The range does not contain exactly an address and prefix-length part.

## What to check

1. Check the slash-separated address and prefix length.

## Recommended action

Supply each range as an address followed by a slash and its prefix length.

## Related log messages

- [IP range prefix length conversion failed](log_e-419b8a27bfe8e9a9.md)
