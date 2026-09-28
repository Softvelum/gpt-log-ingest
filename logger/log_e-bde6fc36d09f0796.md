# Remote fragment returned an unsuccessful HTTP status

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-bde6fc36d09f0796

## Log pattern

```text
key='%s' f=%llu-%llu request failed status=%hu
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — from: Inclusive starting byte offset.
- Argument 3 — to: Exclusive ending byte offset.
- Argument 4 — status: HTTP response status code.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key='https://media.example/movie.mp4' f=0-1048576 request failed status=404
```

## Short description

Remote fragment returned an unsuccessful HTTP status.

## Trigger condition

An established fragmented item receives a response status other than 200 or 206.

## Explanation

The requested fragment is marked failed, the held item reference and active request are released, and the callback returns without starting the next queued request.

## Possible causes

- The origin did not return a successful media response.

## What to check

1. Check the origin status and requested byte range.

## Recommended action

Correct origin availability or access for the requested media.
