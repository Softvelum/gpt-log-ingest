# GeoIP update check started

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_i
Log level: info
Documentation ID: log_i-8aaa72a6cf4cdc05

## Log pattern

```text
checking for GeoIP datebase update
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
checking for GeoIP datebase update
```

## Short description

GeoIP update check started.

## Trigger condition

The update thread begins its scheduled version check.

## Explanation

It connects to the configured panel and requests the available database version. Connection or request failures can end the attempt before any version comparison.
