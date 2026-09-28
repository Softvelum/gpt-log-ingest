# Runtime library loaded on Windows

Product: Nimble Streamer
Component: Runtime library loading
Logging function: log_d
Log level: debug
Documentation ID: log_d-08ae09f47fa9fbdd

## Log pattern

```text
loaded lib_name='%s' dl_=%p
```

## Parameters

- Argument 1 — Library: Requested library name or path, text.
- Argument 2 — Library handle: Loaded library handle, pointer printed in hexadecimal.

The library text is the original requested path; it does not necessarily identify the alternative loaded successfully.

## Log examples

Illustrative examples; these are not captured server logs.

```text
loaded lib_name='libexample.so' dl_=0x12340000
```

## Short description

Runtime library loaded on Windows.

## Trigger condition

The Windows library load succeeds, optionally after trying an alternative library.

## Explanation

The retained handle is ready for function lookup. The printed path is the originally constructed path even if the alternative succeeded.

## Related log messages

- [Required library function was not found](log_e-d2f5cb6edd489a0c.md)
- [Required library function was not found](log_v-388cf53f5da2b3d5.md)
