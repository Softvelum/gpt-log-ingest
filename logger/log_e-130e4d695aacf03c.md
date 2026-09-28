# NDI runtime library could not be loaded

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-130e4d695aacf03c

## Log pattern

```text
failed to load NDI library
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load NDI library
```

## Short description

NDI runtime library could not be loaded.

## Trigger condition

Loading the NDI shared library or its required entry point fails.

## Explanation

NDI initialization returns failure before obtaining the function table or starting a receiver or sender.

## Possible causes

- Loading the NDI shared library or its required entry point fails.

## What to check

1. Check whether the NDI runtime library and required entry point are available in the configured library search location.

## Related log messages

- [NDI or OMT shared-library loading raised an exception](log_d-922ce98beb260eaf.md)
- [NDI runtime returned no function table](log_e-851fabf8960f1ab9.md)
