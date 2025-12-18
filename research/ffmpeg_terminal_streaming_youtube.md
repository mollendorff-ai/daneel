# FFmpeg Terminal Streaming to YouTube/RTMP Research

## Overview
Research on capturing terminal output and streaming to YouTube Live using ffmpeg on macOS (Mac mini - kveldulf).

## 1. Terminal/Screen Capture on macOS

### List Available Devices
```bash
ffmpeg -f avfoundation -list_devices true -i ""
```

Output shows:
- AVFoundation video devices: `[0] FaceTime HD Camera`, `[1] Capture screen 0`
- AVFoundation audio devices: `[0] Built-in Microphone`

### Basic Screen Capture
```bash
# Capture screen 0 with audio device 0
ffmpeg -f avfoundation -r 30 -i "1:0" output.mkv

# Capture screen without audio
ffmpeg -f avfoundation -i "1" -vcodec libx264 -r 10 -tune zerolatency -b:v 500k -bufsize 300k -f mpegts udp://localhost:1234
```

### Important Note
- macOS requires Screen Recording permissions in System Preferences > Privacy & Security > Screen Recording
- By default, ffmpeg on macOS (using avfoundation) does NOT capture the mouse cursor (unlike x11grab on Linux)

## 2. Streaming to YouTube Live via RTMP

### YouTube RTMP Endpoint
```
rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

### Basic YouTube Streaming Command
```bash
# Simple test stream
ffmpeg -re -i input.mp4 \
  -c:v libx264 -c:a aac \
  -f flv rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

### Screen Capture to YouTube (720p, 30fps)
```bash
ffmpeg -f avfoundation \
  -framerate 30 \
  -video_size 1280x720 \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -b:v 2500k \
  -maxrate 2500k \
  -bufsize 5000k \
  -pix_fmt yuv420p \
  -g 60 \
  -c:a aac \
  -b:a 128k \
  -ar 44100 \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

### Screen Capture to YouTube (1080p, 30fps)
```bash
ffmpeg -f avfoundation \
  -framerate 30 \
  -video_size 1920x1080 \
  -i "1:0" \
  -c:v libx264 \
  -preset veryfast \
  -tune zerolatency \
  -b:v 4500k \
  -maxrate 4500k \
  -bufsize 9000k \
  -pix_fmt yuv420p \
  -profile:v main \
  -g 120 \
  -x264opts "nal-hrd=cbr:no-scenecut" \
  -c:a aac \
  -b:a 128k \
  -ar 44100 \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

## 3. Headless Streaming Setup

### Key Considerations for Headless
- No GUI required - runs purely from command line
- Can be automated with cron jobs or systemd
- Ideal for server/Mac mini setups
- Use `-re` flag for real-time playback when streaming files
- Use `-stream_loop -1` for infinite looping

### Headless Screen Capture Example
```bash
# Capture and stream screen in background
ffmpeg -f avfoundation \
  -framerate 30 \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -b:v 3000k \
  -maxrate 3000k \
  -bufsize 6000k \
  -pix_fmt yuv420p \
  -c:a aac \
  -b:a 128k \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY \
  > /tmp/ffmpeg-stream.log 2>&1 &
```

### Loop Pre-recorded Content (24/7 Stream)
```bash
# Infinite loop of video file
ffmpeg -re \
  -stream_loop -1 \
  -i /path/to/dashboard.mp4 \
  -c:v libx264 \
  -preset veryfast \
  -b:v 3000k \
  -maxrate 3000k \
  -bufsize 6000k \
  -c:a aac \
  -b:a 128k \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

### RTSP Camera Restream
```bash
# Restream from IP camera to YouTube
ffmpeg -rtsp_transport tcp \
  -i rtsp://camera_ip:554/stream \
  -preset ultrafast \
  -vcodec libx264 \
  -ar 44100 \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

## 4. Terminal Dashboard Specific

### Capture Terminal Window
For capturing a specific terminal window running a dashboard (like htop, btop, etc):

```bash
# Full screen capture (captures entire screen including terminal)
ffmpeg -f avfoundation \
  -framerate 15 \
  -video_size 1920x1080 \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -b:v 2000k \
  -maxrate 2000k \
  -bufsize 4000k \
  -pix_fmt yuv420p \
  -c:a aac \
  -b:a 128k \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

### Lower Framerate for Terminal (saves bandwidth)
```bash
# 10fps is often sufficient for terminal dashboards
ffmpeg -f avfoundation \
  -framerate 10 \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -b:v 1500k \
  -maxrate 1500k \
  -bufsize 3000k \
  -pix_fmt yuv420p \
  -c:a aac \
  -b:a 128k \
  -f flv \
  rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY
```

## 5. Automation with Cron

### Create Stream Script
```bash
#!/bin/bash
# /usr/local/bin/start-youtube-stream.sh

YOUTUBE_KEY="your_stream_key_here"
RTMP_URL="rtmp://a.rtmp.youtube.com/live2/${YOUTUBE_KEY}"

ffmpeg -f avfoundation \
  -framerate 15 \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -b:v 2000k \
  -maxrate 2000k \
  -bufsize 4000k \
  -pix_fmt yuv420p \
  -c:a aac \
  -b:a 128k \
  -f flv \
  "${RTMP_URL}" \
  >> /var/log/youtube-stream.log 2>&1
```

### Schedule with Cron
```bash
# Start stream every day at 8 AM
0 8 * * * /usr/local/bin/start-youtube-stream.sh
```

## 6. Key Parameters Explained

### Video Encoding
- `-c:v libx264` - H.264 video codec (widely supported)
- `-preset ultrafast` - Fastest encoding (lower CPU usage, larger file size)
- `-preset veryfast` - Balanced encoding
- `-tune zerolatency` - Optimize for live streaming (reduces buffering)
- `-b:v 2000k` - Target video bitrate (2 Mbps)
- `-maxrate 2000k` - Maximum bitrate
- `-bufsize 4000k` - Buffer size (usually 2x maxrate)
- `-pix_fmt yuv420p` - Pixel format (required for YouTube)
- `-g 60` - GOP size (keyframe interval, typically 2x framerate)

### Audio Encoding
- `-c:a aac` - AAC audio codec
- `-b:a 128k` - Audio bitrate (128 kbps)
- `-ar 44100` - Audio sample rate (44.1 kHz)

### Streaming
- `-f flv` - Output format (Flash Video for RTMP)
- `-re` - Read input at native frame rate (for file inputs)

## 7. YouTube Recommended Settings

### 720p @ 30fps
- Video Bitrate: 1500-4000 kbps
- Resolution: 1280x720
- Framerate: 30
- Audio Bitrate: 128 kbps

### 1080p @ 30fps
- Video Bitrate: 3000-6000 kbps
- Resolution: 1920x1080
- Framerate: 30
- Audio Bitrate: 128 kbps

## 8. Troubleshooting

### No Audio Stream
YouTube may reject streams without audio. Add silent audio if needed:
```bash
-f lavfi -i anullsrc=channel_layout=stereo:sample_rate=44100 -c:a aac -b:a 128k
```

### Connection Issues
- Test with: `ffplay rtmp://a.rtmp.youtube.com/live2/YOUR_KEY`
- Check firewall: RTMP uses port 1935 (TCP)
- Use verbose logging: `-loglevel debug`

### High CPU Usage
- Lower resolution: `-video_size 1280x720`
- Lower framerate: `-framerate 15`
- Faster preset: `-preset ultrafast`
- Consider hardware encoding if available

### Monitor Stream Health
```bash
# Basic stream test
ffmpeg -i rtmp://a.rtmp.youtube.com/live2/YOUR_KEY -t 5 -f null - 2>&1 | grep "frame="
```

## 9. Alternative: Using SRT (Modern Alternative to RTMP)

SRT offers better error recovery and encryption:
```bash
ffmpeg -f avfoundation \
  -i "1:0" \
  -c:v libx264 \
  -preset ultrafast \
  -f mpegts \
  "srt://destination:port?passphrase=yourkey"
```

Note: YouTube primarily uses RTMP, but SRT is increasingly popular for professional broadcasting.

## Resources
- [ffmpeg Mac M1 screen recording](https://gist.github.com/jbaranski/f61c50cc41ed7ef37cf389301d9c3347)
- [RTMP Streaming Tutorial - OTTVerse](https://ottverse.com/rtmp-streaming-using-ffmpeg-tutorial/)
- [Headless FFmpeg Streaming - AWS](https://dev.to/aws/broadcasting-to-an-amazon-ivs-live-stream-in-headless-mode-with-ffmpeg-2i73)
- [FFmpeg Full Screen Streaming Examples](https://gist.github.com/tomasinouk/8415acb4e2f86d54fcb9)
- [FFmpeg Live Streaming Guide 2025 - Dacast](https://www.dacast.com/blog/how-to-broadcast-live-stream-using-ffmpeg/)
- [Stream video to YouTube via ffmpeg](https://gist.github.com/olasd/9841772)
