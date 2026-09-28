# DVR JPEG thumbnail stored

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-f46ecf47b44a430c

## Log pattern

```text
added thumbnail seg str='%s/%s' f='%s' offset=%llu size=%u t=%d p=%u st=%llu dbof=%u
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Storage file: Text naming the recorded media storage file.
- Argument 4 — File offset: Unsigned 64-bit byte offset within the media storage file.
- Argument 5 — Segment bytes: Unsigned segment size in bytes.
- Argument 6 — Segment type: Integer category: 0 video initialization, 1 audio initialization, 2 video, 3 audio, 4 shared data file, 5 JPEG, 6 DVB subtitles, 7 KLV metadata, 8 WebVTT.
- Argument 7 — Current archive period: Unsigned current archive period, which may differ from the thumbnail result period.
- Argument 8 — Media start: Unsigned 64-bit media start timestamp in the segment track timescale.
- Argument 9 — Database offset: Unsigned byte offset of the segment record in its database.

## Log examples

Illustrative examples; these are not captured server logs.

```text
added thumbnail seg str='live/camera' f='/data/dvr/live/camera/1700000000000.dat' offset=1024 size=65536 t=2 p=1 st=90000 dbof=512
```

## Short description

DVR JPEG thumbnail stored.

## Trigger condition

The JPEG bytes and segment record are written and the thumbnail is linked to its video segment.

## Explanation

The video segment is marked as having a thumbnail.

## Possible causes

- The JPEG bytes and segment record are written and the thumbnail is linked to its video segment.
