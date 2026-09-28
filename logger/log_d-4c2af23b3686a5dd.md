# Cannot collect Windows performance sample

Product: Nimble Streamer
Component: Windows system statistics
Logging function: log_d
Log level: debug
Documentation ID: log_d-4c2af23b3686a5dd

## Log pattern

```text
PdhCollectQueryData failed status=%#x
```

## Parameters

- Argument 1 — Status: Windows performance-counter status code in hexadecimal.

The Windows status type is unsigned long; the hexadecimal conversion uses unsigned int. These have the same 32-bit width on Windows.

## Log examples

Illustrative examples; these are not captured server logs.

```text
PdhCollectQueryData failed status=0xc0000bc6
```

## Short description

Cannot collect Windows performance sample.

## Trigger condition

Collecting counter data fails for the CPU-load or paging-file-usage query.

## Explanation

The current metric call returns zero instead of a measured value. The existing query remains initialized for later sampling. This diagnostic is Windows-only.

## Possible causes

- Collecting counter data fails for the CPU-load or paging-file-usage query.

## What to check

1. Inspect the Windows performance-counter status and whether system metrics remain zero.
