# NDI sender frame wait raised a system error

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-f1fbbff1476bbffb

## Log pattern

```text
cv wait failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cv wait failed
```

## Short description

NDI sender frame wait raised a system error.

## Trigger condition

Waiting on the sender frame condition variable throws a system error.

## Explanation

The waiting flag is cleared, a shared exit request is set, and waiting returns failure. Both sender loops can then stop. The exception detail is not included in this message.

## Possible causes

- Waiting on the sender frame condition variable throws a system error.

## What to check

1. Check the sender thread lifecycle and preceding system errors.
