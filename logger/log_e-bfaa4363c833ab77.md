# SRT sender statistics omitted from panel request

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-bfaa4363c833ab77

## Log pattern

```text
SRT sender stats size limit exceeded size=%d
```

## Parameters

- Argument 1 — serialized\_bytes: Byte length of the oversized SRT statistics string.

The source supplies a string size of type size_t to %d, which expects int. The printed number is not a portable reliable size.

## Log examples

No example is given because the size argument does not match its printf conversion.

## Short description

SRT sender statistics omitted from panel request.

## Trigger condition

SRT statistics were requested during a full sync, and the serialized sender data exceeds 204,800 bytes.

## Explanation

Only this SRT statistics string is cleared before the panel body is assembled. Other statistics and synchronization continue. This path exists in builds with SRT support.

## Possible causes

- Serialized SRT sender statistics exceed the 200 KiB limit.

## What to check

1. Check the number and size of SRT statistics records.
