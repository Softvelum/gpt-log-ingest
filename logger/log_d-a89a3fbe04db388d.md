# Panel stream-check request prepared

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_d
Log level: debug
Documentation ID: log_d-a89a3fbe04db388d

## Log pattern

```text
prepared stream checker packet request:
%s
```

## Parameters

- Argument 1 — request: Complete HTTP request headers and JSON body.

## Log examples

An example is omitted to avoid reproducing the authenticated panel request structure.

## Short description

Panel stream-check request prepared.

## Trigger condition

A regular or immediate checker has a panel connection and has built its task-list request.

## Explanation

The request is logged before sending. Its endpoint selects regular or immediate tasks; this message does not prove successful exchange or valid task data.
