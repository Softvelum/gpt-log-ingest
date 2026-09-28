# Chunked external response is complete

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_d
Log level: debug
Documentation ID: log_d-bfa1f032ea0c7996

## Log pattern

```text
chunked response is ready url='%s' size=%zu range=%llu-%llu:
%.*s
```

## Parameters

- Argument 1 — URL: Original external request URL, retained even when following a redirect.
- Argument 2 — body bytes: Collected decoded body bytes.
- Argument 3 — from: Requested starting byte offset.
- Argument 4 — to: Requested exclusive ending byte offset.
- Argument 5 — header length: Maximum number of header bytes requested for printing.
- Argument 6 — headers: Response header text, limited by the supplied precision.

The header precision is a platform-sized byte count passed where the format requires an int.

## Log examples

No portable example is provided because the precision argument has the wrong type.

## Short description

Chunked external response is complete.

## Trigger condition

The chunked response parser reaches its final chunk.

## Explanation

The collected body is attached to the parser and the response status is checked before notifying the requester.
