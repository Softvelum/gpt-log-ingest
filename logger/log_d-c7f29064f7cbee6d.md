# Cannot open Windows performance query

Product: Nimble Streamer
Component: Windows system statistics
Logging function: log_d
Log level: debug
Documentation ID: log_d-c7f29064f7cbee6d

## Log pattern

```text
PdhOpenQuery failed status=%#x
```

## Parameters

- Argument 1 — Status: Windows performance-counter status code in hexadecimal.

The Windows status type is unsigned long; the hexadecimal conversion uses unsigned int. These have the same 32-bit width on Windows.

## Log examples

Illustrative examples; these are not captured server logs.

```text
PdhOpenQuery failed status=0xc0000bc6
```

## Short description

Cannot open Windows performance query.

## Trigger condition

Opening the CPU-load or paging-file-usage performance query fails.

## Explanation

The current metric call returns zero instead of a measured value. Initialization can be retried on a later call. This diagnostic is Windows-only.

## Possible causes

- Opening the CPU-load or paging-file-usage performance query fails.

## What to check

1. Inspect the Windows performance-counter status and whether system metrics remain zero.
