use anyhow::Error;

pub fn _mp42hls() -> Result<(), Error> {
  Ok(())
}

/*
尝试实现语句
gst-launch-1.0 filesrc location=IPX-215.mp4 !
qtdemux  name=demux  !
queue !
h264parse  !
mpegtsmux name=mux !
hlssink  playlist-length=0 target-duration=5 max-files=100 playlist-location="playlist.m3u8" location="fragment%03d.ts"   demux.  !
queue !
aacparse !
mux.

参考blog
https://blog.csdn.net/kk3909/article/details/120027553

*/
