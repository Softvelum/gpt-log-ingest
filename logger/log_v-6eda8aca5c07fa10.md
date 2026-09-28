# Cache fragment directory already exists

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-6eda8aca5c07fa10

## Log pattern

```text
dir='%s' already exists
```

## Parameters

- Argument 1 — directory: Fragment storage directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dir='/var/cache/nimble/movie.fr' already exists
```

## Short description

Cache fragment directory already exists.

## Trigger condition

The target fragment directory already exists.

## Explanation

The cache continues with allocation of its fragment files in that directory.
