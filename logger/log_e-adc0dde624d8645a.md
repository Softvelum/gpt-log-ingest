# DRM key-server URL is missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-adc0dde624d8645a

## Log pattern

```text
keyserver must be provided
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
keyserver must be provided
```

## Short description

DRM key-server URL is missing.

## Trigger condition

A provider requiring an explicit key server reaches group validation with an empty URL.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- A provider requiring an explicit key server reaches group validation with an empty URL.

## What to check

1. Provide the key-server URL for the selected protection type.

## Recommended action

Provide the key-server URL for the selected protection type.
