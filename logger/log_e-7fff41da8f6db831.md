# VCAS key download could not be submitted

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-7fff41da8f6db831

## Log pattern

```text
failed to request key url='%s'
```

## Parameters

- Argument 1 — Key-server URL: Text giving the configured or requested key-server URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to request key url='https://keys.example/key'
```

## Short description

VCAS key download could not be submitted.

## Trigger condition

A VCAS key needs its initial download or refresh and the HTTP processor refuses the request.

## Explanation

The key request timestamp is not advanced for this unsuccessful submission.

## Possible causes

- A VCAS key needs its initial download or refresh and the HTTP processor refuses the request.

## What to check

1. Check the VCAS key URL and HTTP request-processor errors.

## Recommended action

Check the VCAS key URL and HTTP request-processor errors.
