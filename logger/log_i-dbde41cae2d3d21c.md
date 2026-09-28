# GeoIP database versions compared

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_i
Log level: info
Documentation ID: log_i-dbde41cae2d3d21c

## Log pattern

```text
geoip db available version='%s' current version='%s'
```

## Parameters

- Argument 1 — available: Trimmed version text returned by the panel.
- Argument 2 — current: Currently recorded GeoIP database version.

## Log examples

Illustrative examples; these are not captured server logs.

```text
geoip db available version='20260928' current version='20260901'
```

## Short description

GeoIP database versions compared.

## Trigger condition

The remote version request succeeds and its whitespace is removed.

## Explanation

Matching versions end the update check. Different strings cause an archive download attempt; this is an equality check rather than a semantic version ordering.
