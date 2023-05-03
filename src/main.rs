extern crate gstreamer as gst;
use anyhow::{anyhow, Result};
use gstreamer::{prelude::*, ElementFactory, Pipeline};
// use std::convert::From;

fn create_pipeline(uri: &str) -> Result<Pipeline> {
    // Initialize GStreamer
    gst::init()?;

    // Create a new pipeline
    let pipeline = gst::Pipeline::new(None);

    // Create a uridecodebin element and set the URI to the video file
    let uridecodebin = ElementFactory::make("uridecodebin", None)?;
    uridecodebin.set_property("uri", &uri)?;

    // Add the uridecodebin element to the pipeline
    pipeline.add(&uridecodebin)?;

    // Create an autovideosink element for displaying the video
    let videosink = ElementFactory::make("autovideosink", None)?;

    // Add the videosink element to the pipeline
    pipeline.add(&videosink)?;

    // Link the uridecodebin and videosink elements together
    uridecodebin.link(&videosink)?;

    // Return the pipeline
    Ok(pipeline)
}

fn main() -> Result<()> {
    let uri = "file:///C:/Users/abdiu/Videos/myvidtrial";
    let pipeline = create_pipeline(uri)?;
    pipeline.set_state(gst::State::Playing)?;

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter() {
        match msg.view() {
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error()
                );
                eprintln!("Debugging information: {:?}", err.get_debug());
                return Err(anyhow!("Error playing video"));
            }
            gst::MessageView::Eos(..) => {
                println!("End-Of-Stream reached.");
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;
    Ok(())
}

// fn main() {
//     println!("Hello World!")
// }
