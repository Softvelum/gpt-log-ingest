# Remote item request queue is empty

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_v
Log level: verbose
Documentation ID: log_v-4abb15f4ec4a9dec

## Log pattern

```text
no pending requests for item key='%s'
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no pending requests for item key='https://media.example/movie.mp4'
```

## Short description

Remote item request queue is empty.

## Trigger condition

A successful GET completion removes its active request and finds no pending requests.

## Explanation

The callback returns without scheduling another range for this item.
