# HTTP/3 decoder supplied an unrecognized header

Product: Nimble Streamer
Component: HTTP/3 header decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-12ee5baa1ba77bd5

## Log pattern

```text
lsquic error. hset_prepare_decode called with invalid hdr
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
lsquic error. hset_prepare_decode called with invalid hdr
```

## Short description

HTTP/3 decoder supplied an unrecognized header.

## Trigger condition

The decoder asks to resize an existing header but its header pointer cannot be found in the active header set, including a full search.

## Explanation

The decode-preparation callback returns no header and does not resize a buffer.

## Possible causes

- The decoder and active header-set ownership disagree about the header being decoded.

## What to check

1. Check matching HTTP/3 library/server versions and adjacent header-decoding failures.

## Recommended action

Investigate repeated invalid header callbacks as an integration or lifetime defect.
