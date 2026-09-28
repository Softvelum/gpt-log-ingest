# Incompatible disk cache cleaning loop finished

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-3c1c893ca0c7f5eb

## Log pattern

```text
Cache cleaned
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Cache cleaned
```

## Short description

Incompatible disk cache cleaning loop finished.

## Trigger condition

The compatibility cleanup has finished traversing cache directories.

## Explanation

The loop attempted to remove regular files, but this completion message does not establish that every filesystem deletion succeeded.
