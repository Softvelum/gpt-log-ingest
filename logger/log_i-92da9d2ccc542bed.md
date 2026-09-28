# NVIDIA free-context cache released

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-92da9d2ccc542bed

## Log pattern

```text
nvenc context cache released
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
nvenc context cache released
```

## Short description

NVIDIA free-context cache released.

## Trigger condition

Manager release has visited all free cached contexts and requested their destruction.

## Explanation

Free cache lists are now empty; shared and still-used contexts are handled afterward.
