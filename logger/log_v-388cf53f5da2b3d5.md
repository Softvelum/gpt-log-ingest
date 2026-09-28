# Required library function was not found

Product: Nimble Streamer
Component: Runtime library loading
Logging function: log_v
Log level: verbose
Documentation ID: log_v-388cf53f5da2b3d5

## Log pattern

```text
GetProcAddress failed for dl_=%p symbol='%s' error=%d
```

## Parameters

- Argument 1 — Library handle: Loaded library handle, pointer printed in hexadecimal.
- Argument 2 — Function: Requested exported function name, text.
- Argument 3 — Windows error: Windows last-error code, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
GetProcAddress failed for dl_=0x12340000 symbol='example_function' error=127
```

## Short description

Required library function was not found.

## Trigger condition

Looking up a named function in a loaded library returns no address on Windows.

## Explanation

The lookup throws an exception to its caller. The error and verbose variants have the same failure behavior; the caller selects the reporting level.

## Possible causes

- The loaded library does not supply the requested function, or the loader rejected the lookup.

## What to check

1. Check that the library version exports the requested function and inspect the loader diagnostic.

## Recommended action

Provide a compatible runtime library.

## Related log messages

- [Required library function was not found](log_e-d2f5cb6edd489a0c.md)
