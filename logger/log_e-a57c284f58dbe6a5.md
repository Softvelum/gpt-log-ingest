# VCAS key response has the wrong JSON type

Product: Nimble Streamer
Component: VCAS DRM
Logging function: log_e
Log level: error
Documentation ID: log_e-a57c284f58dbe6a5

## Log pattern

```text
error processing keyResponse json='%s'
```

## Parameters

- Argument 1 — Response body: Text containing the VCAS response.

## Log examples

Illustrative examples; these are not captured server logs.

```text
error processing keyResponse json='[]'
```

## Short description

VCAS key response has the wrong JSON type.

## Trigger condition

The VCAS key response parses as a JSON value other than an object.

## Explanation

Response parsing returns failure.

## Possible causes

- The VCAS key response parses as a JSON value other than an object.

## What to check

1. Return a JSON object from the VCAS key endpoint.

## Recommended action

Return a JSON object from the VCAS key endpoint.
