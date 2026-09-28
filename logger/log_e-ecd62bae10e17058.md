# Zixi API response is not parseable JSON

Product: Nimble Streamer
Component: Zixi integration
Logging function: log_e
Log level: error
Documentation ID: log_e-ecd62bae10e17058

## Log pattern

```text
failed to parse Zixi API response='%s'
```

## Parameters

- Argument 1 — Response: Complete received API response body.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse Zixi API response='invalid-json'
```

## Short description

Zixi API response is not parseable JSON.

## Trigger condition

The received configuration, stream, output, settings, creation, or statistics response cannot be parsed as JSON.

## Explanation

The current response is not applied. Configuration parsing has already cleared the interface list; failed statistics parsing leaves previously stored remote addresses intact. Stream/output request chains return failure and the request processor moves on.

## Possible causes

- The received configuration, stream, output, settings, creation, or statistics response cannot be parsed as JSON.

## What to check

1. Inspect the response body and the configured local Zixi API endpoint.
