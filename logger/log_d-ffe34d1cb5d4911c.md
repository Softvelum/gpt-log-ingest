# NDI receiver created and ready to capture

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-ffe34d1cb5d4911c

## Log pattern

```text
ndi receiver started, ndi_name=%s
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ndi receiver started, ndi_name=HOST (Camera)
```

## Short description

NDI receiver created and ready to capture.

## Trigger condition

The NDI runtime successfully creates a receiver for the selected full source name.

## Explanation

The input clears its inactivity count and enters the receive state. Actual frame arrival is established by subsequent capture, not by this message.

## Possible causes

- Successful NDI source selection and receiver creation.

## Related log messages

- [NDI receiver stopped after its source disappeared](log_d-b7527187e830eab3.md)
- [NDI receiver creation failed](log_e-8b9ad4f006218a42.md)
