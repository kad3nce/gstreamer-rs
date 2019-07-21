// This example demonstrates how events can be created and sent to the pipeline.
// What this example does is scheduling a timeout on the main loop, and
// sending an EOS message on the bus from there - telling the pipeline
// to shut down. Once that event is processed by everything, the EOS message
// is going to be sent and we catch that one to shut down everything.

// GStreamer's bus is an abstraction layer above an arbitrary main loop.
// This makes sure that GStreamer can be used in conjunction with any existing
// other framework (GUI frameworks, mostly) that operate their own main loops.
// Main idea behind the bus is the simplification between the application and
// GStreamer, because GStreamer is heavily threaded underneath.

// Any thread can post messages to the bus, which is essentially a thread-safe
// queue of messages to process. When a new message was sent to the bus, it
// will wake up the main loop implementation underneath it (which will then
// process the pending messages from the main loop thread).

// An application itself can post messages to the bus aswell.
// This makes it possible, e.g., to schedule an arbitrary piece of code
// to run in the main loop thread - avoiding potential threading issues.

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

extern crate gstreamer_pbutils as gst_pbutils;
use gst_pbutils::prelude::*;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

use std::env;
use std::error::Error as StdError;
#[cfg(feature = "v1_10")]
use std::sync::{Arc, Mutex};

extern crate failure;
use failure::Error;

#[macro_use]
extern crate failure_derive;

#[path = "../examples-common.rs"]
mod examples_common;

fn configure_encodebin(encodebin: &gst::Element) -> Result<(), Error> {
    // To tell the encodebin what we want it to produce, we create an EncodingProfile
    // https://gstreamer.freedesktop.org/data/doc/gstreamer/head/gst-plugins-base-libs/html/GstEncodingProfile.html
    // This profile consists of information about the contained audio and video formats
    // as well as the container format we want everything to be combined into.

    // Every audiostream piped into the encodebin should be encoded using vorbis.
    let audio_profile = gst_pbutils::EncodingAudioProfileBuilder::new()
        // .format(&gst::Caps::new_simple("audio/mpeg,mpegversion=4", &[]))
        .format(&gst::Caps::from_string("audio/mpeg,mpegversion=4,rate=48000,channel=2,bitrate=320").unwrap())
        .presence(0)
        .build().unwrap();

    // Every videostream piped into the encodebin should be encoded using theora.
    let video_profile = gst_pbutils::EncodingVideoProfileBuilder::new()
        // .format(&gst::Caps::from_string("video/x-h264,width=1920,height=1080,framerate=30000/1001").unwrap())
        // WORKS IN VLC, NOT IN QT
        .format(&gst::Caps::from_string("video/x-h264,width=1280,height=720,framerate=30000/1001,bitrate=10000,bframes=2,key-int-max=60,pass=pass1,preset=veryfast").unwrap())
        // .format(&gst::Caps::from_string("video/x-h264,width=1280,height=720,framerate=30000/1001,bitrate=10000,bframes=2,key-int-max=60,pass=pass1,preset=veryfast").unwrap())
        // .format(&gst::Caps::new_simple("video/x-h264", &[]))
        .presence(0)
        .build().unwrap();

    // All streams are then finally combined into a matroska container.
    let container_profile = gst_pbutils::EncodingContainerProfileBuilder::new()
        .name("container")
        // .format(&gst::Caps::new_simple("video/x-matroska", &[]))
        .format(&gst::Caps::from_string("video/quicktime,variant=iso").unwrap())
        .add_profile(&(video_profile))
        .add_profile(&(audio_profile))
        .build().unwrap();

    // Finally, apply the EncodingProfile onto our encodebin element.
    encodebin
        .set_property("profile", &container_profile)
        .expect("set profile property failed");

    Ok(())
}

fn example_main() {
    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    let args: Vec<_> = env::args().collect();
    // let uri: &str;
    let output_file: &str;

    if args.len() == 2 {
        // uri = args[1].as_ref();
        output_file = args[1].as_ref();
    } else {
        println!("Usage: encodebin output_file");
        std::process::exit(-1)
    };

    // // This creates a pipeline by parsing the gst-launch pipeline syntax.
    // let pipeline = gst::parse_launch("audiotestsrc ! fakesink").unwrap();


    let pipeline = gst::Pipeline::new(None);
    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let encodebin =
        gst::ElementFactory::make("encodebin", None).ok_or(MissingElement("encodebin")).unwrap();

    let sink = gst::ElementFactory::make("filesink", None).ok_or(MissingElement("filesink")).unwrap();
    sink.set_property("location", &output_file)
        .expect("setting location property failed");

    // Configure the encodebin.
    // Here we tell the bin what format we expect it to create at its output.
    configure_encodebin(&encodebin).unwrap();

    pipeline
        .add_many(&[&encodebin, &sink])
        .expect("failed to add elements to pipeline");
    // It is clear from the start, that encodebin has only one src pad, so we can
    // directly link it to our filesink without problems.
    // The caps of encodebin's src-pad are set after we configured the encoding-profile.
    // (But filesink doesn't really care about the caps at its input anyway)
    gst::Element::link_many(&[&encodebin, &sink]).unwrap();

    // VIDEO
    let video_src =
        gst::ElementFactory::make("v4l2src", None).ok_or(MissingElement("v4l2src")).unwrap();
    let video_queue =
        gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue")).unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert", None)
        .ok_or(MissingElement("videoconvert")).unwrap();
    let video_scale = gst::ElementFactory::make("videoscale", None)
        .ok_or(MissingElement("videoscale")).unwrap();

    let video_elements = &[&video_src, &video_queue, &video_convert, &video_scale];
    pipeline
        .add_many(video_elements)
        .expect("failed to add video elements to pipeline");
    gst::Element::link_many(video_elements).unwrap();

    // Request a sink pad from our encodebin, that can handle a raw videostream.
    // The encodebin will then automatically create an internal pipeline, that encodes
    // the audio stream in the format we specified in the EncodingProfile.
    let enc_video_sink_pad = encodebin
        .get_request_pad("video_%u")
        .expect("Could not get video pad from encodebin");
    let video_src_pad = video_scale
        .get_static_pad("src")
        .expect("videoscale has no srcpad");
    video_src_pad.link(&enc_video_sink_pad).unwrap();

    // AUDIO
    let audio_src =
        gst::ElementFactory::make("alsasrc", None).ok_or(MissingElement("alsasrc")).unwrap();
    let audio_queue =
        gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue")).unwrap();
    let audio_convert = gst::ElementFactory::make("audioconvert", None)
        .ok_or(MissingElement("audioconvert")).unwrap();
    let audio_resample = gst::ElementFactory::make("audioresample", None)
        .ok_or(MissingElement("audioresample")).unwrap();

    let audio_elements = &[&audio_src, &audio_queue, &audio_convert, &audio_resample];
    pipeline
        .add_many(audio_elements)
        .expect("failed to add audio elements to pipeline");
    gst::Element::link_many(audio_elements).unwrap();

    // Request a sink pad from our encodebin, that can handle a raw audiostream.
    // The encodebin will then automatically create an internal pipeline, that encodes
    // the audio stream in the format we specified in the EncodingProfile.
    let enc_audio_sink_pad = encodebin
        .get_request_pad("audio_%u")
        .expect("Could not get audio pad from encodebin");
    let audio_src_pad = audio_resample
        .get_static_pad("src")
        .expect("resample has no srcpad");
    audio_src_pad.link(&enc_audio_sink_pad).unwrap();

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop. This closure will be executed
    // in an interval of 5 seconds. The return value of the handler function
    // determines whether the handler still wants to be called:
    // - glib::Continue(false) - stop calling this handler, remove timeout
    // - glib::Continue(true) - continue calling this handler
    glib::timeout_add_seconds(5, move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(false),
        };

        println!("sending eos");

        // We create an EndOfStream event here, that tells all elements to drain
        // their internal buffers to their following elements, essentially draining the
        // whole pipeline (front to back). It ensuring that no data is left unhandled and potentially
        // headers were rewritten (e.g. when using something like an MP4 or Matroska muxer).
        // The EOS event is handled directly from this very thread until the first
        // queue element is reached during pipeline-traversal, where it is then queued
        // up and later handled from the queue's streaming thread for the elements
        // following that queue.
        // Once all sinks are done handling the EOS event (and all buffers that were before the
        // EOS event in the pipeline already), the pipeline would post an EOS message on the bus,
        // essentially telling the application that the pipeline is completely drained.
        let ev = gst::Event::new_eos().build();
        pipeline.send_event(ev);

        // Remove this handler, the pipeline will shutdown anyway, now that we
        // sent the EOS event.
        glib::Continue(false)
    });

    //bus.add_signal_watch();
    //bus.connect_message(move |_, msg| {
    let main_loop_clone = main_loop.clone();
    // This sets the bus's signal handler (don't be mislead by the "add", there can only be one).
    // Every message from the bus is passed through this function. Its returnvalue determines
    // whether the handler wants to be called again. If glib::Continue(false) is returned, the
    // handler is removed and will never be called again. The mainloop still runs though.
    bus.add_watch(move |_, msg| {
        use gst::MessageView;
        println!("received a message");

        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                // An EndOfStream event was sent to the pipeline, so we tell our main loop
                // to stop execution here.
                main_loop.quit()
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        // Tell the mainloop to continue executing this callback.
        glib::Continue(true)
    });

    // Operate GStreamer's bus, facilliating GLib's mainloop here.
    // This function call will block until you tell the mainloop to quit
    // (see above for how to do this).
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    // Remove the watch function from the bus.
    // Again: There can always only be one watch function.
    // Thus we don't have to tell him which function to remove.
    bus.remove_watch().unwrap();
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
