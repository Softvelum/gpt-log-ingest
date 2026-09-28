# Widevine response payload decoded

Product: Nimble Streamer
Component: Widevine DRM
Logging function: log_d
Log level: debug
Documentation ID: log_d-4fbdbb1f4361ff5c

## Log pattern

```text
Widevine decoded response='%s'
```

## Parameters

- Argument 1 — Decoded response: Text obtained by base64-decoding the response payload.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Widevine decoded response='{"status":"OK","tracks":[]}'
```

## Short description

Widevine response payload decoded.

## Trigger condition

The outer Widevine response contains a string payload which has been base64-decoded.

## Explanation

The decoded text is logged before its JSON structure and key fields are validated.

## Possible causes

- The outer Widevine response contains a string payload which has been base64-decoded.
