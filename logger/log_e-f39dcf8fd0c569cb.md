# SLDP Play command is missing or invalid

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-f39dcf8fd0c569cb

## Log pattern

```text
viewer issues invalid play command in wait_for_play state
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
viewer issues invalid play command in wait_for_play state
```

## Short description

SLDP Play command is missing or invalid.

## Trigger condition

Play processing finishes without recognizing an object command field exactly equal to Play.

## Explanation

The request is rejected and the caller closes the playback connection.

## Possible causes

- Play processing finishes without recognizing an object command field exactly equal to Play.

## What to check

1. Check the top-level JSON object and its case-sensitive command field.

## Recommended action

Send a valid Play command object.
