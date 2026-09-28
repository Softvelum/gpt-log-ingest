# Writable event while waiting for origin headers

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-67b1616b6d6cc6fc

## Log pattern

```text
kEventOut - kStateOriginRespHeader should get kEventIn
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
kEventOut - kStateOriginRespHeader should get kEventIn
```

## Short description

Writable event while waiting for origin headers.

## Trigger condition

A writable event arrives while an HTTP client or cache downloader expects origin response input.

## Explanation

The client path prepares HTTP 500 and the downloader path finishes the failed download.

## Possible causes

- A writable event arrives while an HTTP client or cache downloader expects origin response input.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
