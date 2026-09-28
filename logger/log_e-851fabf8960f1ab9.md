# NDI runtime returned no function table

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-851fabf8960f1ab9

## Log pattern

```text
failed to load NDI functions
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load NDI functions
```

## Short description

NDI runtime returned no function table.

## Trigger condition

The NDI library loads but its version-four loader returns no function table.

## Explanation

Initialization returns failure before calling the NDI runtime initializer.

## Possible causes

- The NDI library loads but its version-four loader returns no function table.

## What to check

1. Check whether the loaded NDI runtime provides the requested function table.

## Related log messages

- [NDI runtime library could not be loaded](log_e-130e4d695aacf03c.md)
- [NDI runtime initialization failed](log_e-80890175fb9db320.md)
