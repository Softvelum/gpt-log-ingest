# HTTP/2 remote DASH dependency updated

Product: Nimble Streamer
Component: HTTP/2 and HTTP/3 request handling
Logging function: log_d
Log level: debug
Documentation ID: log_d-dd3b6abcb8264269

## Log pattern

```text
[H2C::PRSDR] sst=wait_vod_items key='%s'
```

## Parameters

- Argument 1 — Cache key: Requested cache-item key, text; identifies the origin resource.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[H2C::PRSDR] sst=wait_vod_items key='https://origin.example/movie.mp4'
```

## Short description

HTTP/2 remote DASH dependency updated.

## Trigger condition

A completed cache item is being processed for a pending remote SMIL-based DASH request.

## Explanation

The key is checked against required items, its retained reference is released, and it is removed from the dependency list. The request keeps waiting for other items or resumes SMIL processing once all are present. An unexpected key instead produces an internal-error reply.
