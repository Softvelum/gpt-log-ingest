# Playlist response is not a JSON object

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-d2bb07fefc86741d

## Log pattern

```text
failed to process response from server playlist handler body='%s'
```

## Parameters

- Argument 1 — Response body: Text containing the rejected response.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process response from server playlist handler body='invalid JSON'
```

## Short description

Playlist response is not a JSON object.

## Trigger condition

The main task response or a block-definition body cannot be parsed as a JSON object.

## Explanation

A main response update is abandoned; an invalid block body adds no blocks.

## Possible causes

- The main task response or a block-definition body cannot be parsed as a JSON object.

## What to check

1. Return a JSON object containing the expected Tasks or Blocks collection.

## Recommended action

Return a JSON object containing the expected Tasks or Blocks collection.
