# KLV metadata packet disappeared

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-29e65ff7acc823dc

## Log pattern

```text
updateAvailable klv metadata doesn't exists, pid=%d, stream=%d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.
- Argument 2 — metadata\_index: Internal zero-based KLV metadata stream index, not another packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable klv metadata doesn't exists, pid=400, stream=0
```

## Short description

KLV metadata packet disappeared.

## Trigger condition

KLV forwarding is enabled and an existing forwarded KLV packet identifier is absent from the new available KLV set.

## Explanation

Its internal metadata stream index is returned to the unused set and the packet-to-index mapping is removed. Later new KLV packets may reuse that index.
