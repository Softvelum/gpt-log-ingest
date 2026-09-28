# SLDP expects a Play command

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-4ad5e346ee6642fe

## Log pattern

```text
wrong command in wait_for_play state '%s'
```

## Parameters

- Argument 1 — Command name: Decoded command field from the received object.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong command in wait_for_play state 'Cancel'
```

## Short description

SLDP expects a Play command.

## Trigger condition

The Play parser sees a command value other than Play.

## Explanation

Command-field processing stops; the parser subsequently rejects the request as an invalid Play command.

## Possible causes

- The Play parser sees a command value other than Play.

## What to check

1. Check the command name and case in the initial player request.

## Recommended action

Start playback with the supported Play command.
