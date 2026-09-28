# DRM HTTP request completed

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-27c0820c09fd3dff

## Log pattern

```text
DrmManager::OnRequestProcessed url='%s' status=%d body_len=%d
```

## Parameters

- Argument 1 — Key-server URL: Text giving the configured or requested key-server URL.
- Argument 2 — HTTP status: Integer HTTP response status.
- Argument 3 — Body bytes: Integer response-body size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DrmManager::OnRequestProcessed url='https://keys.example/key' status=200 body_len=16
```

## Short description

DRM HTTP request completed.

## Trigger condition

The HTTP request processor delivers a completed DRM response.

## Explanation

Managed provider responses are dispatched by request ID; older raw-key responses are validated for HTTP 200 and a 16-byte body.

## Possible causes

- The HTTP request processor delivers a completed DRM response.
