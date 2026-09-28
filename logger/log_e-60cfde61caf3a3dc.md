# SLDP Play selects no media

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-60cfde61caf3a3dc

## Log pattern

```text
viewer issues invalid command in wait_for_play state
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
viewer issues invalid command in wait_for_play state
```

## Short description

SLDP Play selects no media.

## Trigger condition

Play processing recognizes the command but no playback selections exist afterward.

## Explanation

The request is rejected and the caller closes the playback connection.

## Possible causes

- Play processing recognizes the command but no playback selections exist afterward.

## What to check

1. Check the streams array and whether it contains usable stream entries.

## Recommended action

Include at least one valid media selection.
