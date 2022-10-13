use std::fs::create_dir_all;

use anyhow::Error;
use gstreamer::{
  prelude::Cast,
  traits::{ElementExt, GstObjectExt},
  StructureRef,
};

use crate::model::smov::Smov;

impl Smov {
  pub fn to_hls(self: &Self) -> Result<(), Error> {
    let path = crate::app::APP.lock().clone().conf.tidy_folder;
    let media_folder = path.join(self.realname.clone());

    let media = media_folder
      .join(format!("{}.{}", self.realname, self.extension))
      .clone();

    let ts_path = media_folder.join("hls").clone();

    create_dir_all(&ts_path).expect("创建HLS文件夹错误");

    let commond = format!("filesrc location={} name=file ! qtdemux  name=demux  ! queue ! progressreport update-freq=1 silent=true ! h264parse  ! mpegtsmux name=mux ! hlssink  playlist-length=0 target-duration=5 max-files=99999999 playlist-location={}/playlist.m3u8 location={}/fragment%03d.ts name=sink demux. ! queue ! aacparse ! mux. " , media.display(),ts_path.display(),ts_path.display()).replace("\\", "/");

    gstreamer::init().unwrap();

    let pipeline = gstreamer::parse_launch(&commond)
      .unwrap()
      .downcast::<gstreamer::Pipeline>()
      .unwrap();

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let bus = pipeline
      .bus()
      .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
      use gstreamer::MessageView;
      match msg.view() {
        MessageView::Eos(..) => {
          println!("EOS");
          break;
        }
        MessageView::Error(err) => {
          pipeline.set_state(gstreamer::State::Null).unwrap();
          eprintln!(
            "Got error from {}: {} ({})",
            msg
              .src()
              .map(|s| String::from(s.path_string()))
              .unwrap_or_else(|| "None".into()),
            err.error(),
            err.debug().unwrap_or_else(|| "".into()),
          );
          break;
        }
        MessageView::Element(ele) => {
          let structure = ele.structure().unwrap();
          let percent = structure.get::<i32>("percent").unwrap();
        }
        _ => {}
      }
    }

    pipeline.set_state(gstreamer::State::Null).unwrap();

    Ok(())
  }
}
