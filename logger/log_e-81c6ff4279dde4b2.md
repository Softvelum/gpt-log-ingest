# Remote response has no active cache request

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-81c6ff4279dde4b2

## Log pattern

```text
MetaCacheManager::OnRequestsProcessed invalid url='%s'
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
MetaCacheManager::OnRequestsProcessed invalid url='https://media.example/movie.mp4'
```

## Short description

Remote response has no active cache request.

## Trigger condition

The completion callback cannot find an active request for the response URL.

## Explanation

The callback returns without updating a cache item or pending queue.

## Possible causes

- The active request record is absent when completion arrives.

## What to check

1. Compare request submission, failure, and completion for this URL.

## Recommended action

Investigate unexpected or duplicate completion callbacks.
