# Managed task output read

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_d
Log level: debug
Documentation ID: log_d-0ca683b777ec22cf

## Log pattern

```text
pid=%d fd=%d wrote %zd bytes:
%.*s
```

## Parameters

- Argument 1 — Process: Integer task process identifier.
- Argument 2 — Output descriptor: Integer descriptor used by Nimble to read the task output.
- Argument 3 — Read result: Signed byte count returned by the read operation; a negative value indicates failure.
- Argument 4 — Text precision: The read result supplied as the text precision, which printf expects as an int.
- Argument 5 — Output text: The raw output buffer displayed up to the supplied precision.

The precision argument is supplied as the signed read-result type rather than an explicit int. Its compatibility is platform dependent; a negative read result also fails to establish valid bounded buffer contents.

## Log examples

A reliable example is omitted because the precision type is not guaranteed compatible and failed reads are not excluded.

## Short description

Managed task output read.

## Trigger condition

A readable-output event causes one read from the task pipe. Available only on Linux builds.

## Explanation

The result and a bounded text view of the buffer are logged before bytes are appended to the recent-output buffer. Negative read results are not checked before this log.
