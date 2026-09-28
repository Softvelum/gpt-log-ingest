# DVR recording settings are being synchronized

Product: Nimble Streamer
Component: DVR
Logging function: log_d
Log level: debug
Documentation ID: log_d-b5a753dede408f7a

## Log pattern

```text
DvrMainThread::SyncSettings
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
DvrMainThread::SyncSettings
```

## Short description

DVR recording settings are being synchronized.

## Trigger condition

Changed DVR settings or stream buffers are accepted after pending context work completes.

## Explanation

Archives and writer contexts are created, updated, moved, or removed to match current settings.

## Possible causes

- Changed DVR settings or stream buffers are accepted after pending context work completes.
