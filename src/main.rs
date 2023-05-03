extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

use gst::prelude::*;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create a new pipeline
    let pipeline = gst::Pipeline::new(None);

    // Create a new file source element
    let source = gst::ElementFactory::make("filesrc", None).unwrap();
    source.set_property("location", &"../vid.mp4").unwrap();
    pipeline.add(&source).unwrap();

    // Create a new video decoder element
    let decoder = gst::ElementFactory::make("decodebin", None).unwrap();
    pipeline.add(&decoder).unwrap();

    // Create a new video sink element
    let sink = gst::ElementFactory::make("autovideosink", None).unwrap();
    pipeline.add(&sink).unwrap();
    sink.sync_state_with_parent().unwrap();

    // Link the elements together
    source.link(&decoder).unwrap();
    let sinkpad = sink.get_request_pad("sink").unwrap();
    let decoder_sinkpad = decoder.get_static_pad("sink").unwrap();
    decoder_sinkpad.link(&sinkpad).unwrap();

    // Start the pipeline
    pipeline.set_state(gst::State::Playing).unwrap();

    // Wait for the pipeline to finish
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter() {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                println!("Error: {}", err.get_error());
                break;
            }
            _ => (),
        }
    }

    // Clean up the pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}
