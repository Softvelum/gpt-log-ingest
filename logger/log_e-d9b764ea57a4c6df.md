# Key URL is no longer registered

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-d9b764ea57a4c6df

## Log pattern

```text
unknown key url='%s'
```

## Parameters

- Argument 1 — Key-server URL: Text giving the configured or requested key-server URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown key url='https://keys.example/key'
```

## Short description

Key URL is no longer registered.

## Trigger condition

A VCAS key response or failed key request refers to a URL absent from the key lookup.

## Explanation

The response or failure is ignored because no matching key can be updated.

## Possible causes

- A VCAS key response or failed key request refers to a URL absent from the key lookup.

## What to check

1. Check key removal and DRM settings changes while the request was pending.

## Recommended action

Check key removal and DRM settings changes while the request was pending.
