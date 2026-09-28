# Publisher authorization thread entered

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_d
Log level: debug
Documentation ID: log_d-d85abf7ecd558044

## Log pattern

```text
PayPerPublishThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
PayPerPublishThread started
```

## Short description

Publisher authorization thread entered.

## Trigger condition

The authorization thread starts running.

## Explanation

It gathers pending requests, optionally delays for grouping, and then submits batches to the handler. An empty handler URL grants the collected requests directly.
