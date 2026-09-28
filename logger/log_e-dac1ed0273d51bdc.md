# SMIL document has no valid root element

Product: Nimble Streamer
Component: SMIL media playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-dac1ed0273d51bdc

## Log pattern

```text
invalid SMIL file=%s
```

## Parameters

- Argument 1 — SMIL filename: Text filename passed to the XML reader. It is a local SMIL path or the local cache-fragment filename for a downloaded SMIL document.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid SMIL file=/media/movie.smil
```

## Short description

SMIL document has no valid root element.

## Trigger condition

XML reading succeeds, but the document has no root element or the root element name is not exactly lowercase smil.

## Explanation

HLS playlist generation, including fragmented-MP4 HLS, returns failure and the HTTP caller prepares HTTP 500. DASH input discovery returns an empty item list, so its HTTP caller prepares HTTP 404. No requested playlist or manifest is generated from this document.

## Possible causes

- XML reading succeeds, but the document has no root element or the root element name is not exactly lowercase smil.

## What to check

1. Ensure the XML document has a root element named smil with the expected lowercase spelling; a different root is rejected even if the XML is otherwise well formed.

## Recommended action

Ensure the XML document has a root element named smil with the expected lowercase spelling; a different root is rejected even if the XML is otherwise well formed.

## Related log messages

- [SMIL playlist generation failed](log_v-e86ce78b4e392467.md)
