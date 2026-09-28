# Last advertising content observer removed

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-26ccb8c131e3bf4b

## Log pattern

```text
[SndT::RemO] key='%s' p=%p empty set
```

## Parameters

- Argument 1 — Content: Advertising content identifier.
- Argument 2 — Observer: Address of the consumer being removed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SndT::RemO] key='example-ad' p=0x12345000 empty set
```

## Short description

Last advertising content observer removed.

## Trigger condition

Removing a consumer leaves the advertising-content observer set empty.

## Explanation

The thread unregisters from the advertising rules manager for that content and erases the empty observer set.

## Possible causes

- Removing a consumer leaves the advertising-content observer set empty.

## What to check

1. Check whether all consumers of this advertising content have ended.

## Related log messages

- [Advertising content observer removed](log_d-0195bf45e840018b.md)
