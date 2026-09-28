# Advertising chunk has no cache item

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-76eb2f14bb3bea2c

## Log pattern

```text
failed to process advertising chunk request, cache item is null
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process advertising chunk request, cache item is null
```

## Short description

Advertising chunk has no cache item.

## Trigger condition

An advertising media or initialization chunk request starts without a cache item.

## Explanation

Generation returns before writing any chunk.

## Possible causes

- An advertising media or initialization chunk request starts without a cache item.

## What to check

1. Check the preceding cache reservation for the advertising request.

## Recommended action

Check the preceding cache reservation for the advertising request.
