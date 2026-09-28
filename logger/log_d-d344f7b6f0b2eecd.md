# Origin body download progressed

Product: Nimble Streamer
Component: Background origin download
Logging function: log_d
Log level: debug
Documentation ID: log_d-d344f7b6f0b2eecd

## Log pattern

```text
received %lld (total %llu of %llu) body bytes from %s s=%d
```

## Parameters

- Argument 1 — received: Bytes read in this operation.
- Argument 2 — total: Body bytes received by this downloader.
- Argument 3 — expected: Expected body bytes for this download.
- Argument 4 — peer: Peer role label.
- Argument 5 — socket: Origin socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
received 512 (total 1024 of 4096) body bytes from origin s=12
```

## Short description

Origin body download progressed.

## Trigger condition

A body read returns positive data into the cache buffer.

## Explanation

The received total is updated before cache size is increased. Completion marks the item ready only when its stored size equals capacity; otherwise it is canceled.
