// This example demonstrates the use of the encodebin element.
// The example takes an arbitrary URI as input, which it will try to decode
// and finally reencode using the encodebin element.
// For more information about how the decodebin element works, have a look at
// the decodebin-example.
// Since we tell the encodebin what format we want to get out of it from the start,
// it provides the correct caps and we can link it before starting the pipeline.
// After the decodebin has found all streams and we piped them into the encodebin,
// the operated pipeline looks as follows:

//                  /-{queue}-{audioconvert}-{audioresample}-\
// {uridecodebin} -|                                          {encodebin}-{filesink}
//                  \-{queue}-{videoconvert}-{videoscale}----/

#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_pbutils as gst_pbutils;
use gst_pbutils::prelude::*;

extern crate ctrlc;

#[cfg_attr(feature = "v1_10", macro_use)]
extern crate glib;

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

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

#[derive(Debug, Fail)]
#[fail(
    display = "Received error from {}: {} (debug: {:?})",
    src, error, debug
)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

#[cfg(feature = "v1_10")]
#[derive(Clone, Debug)]
struct ErrorValue(Arc<Mutex<Option<Error>>>);

#[cfg(feature = "v1_10")]
impl glib::subclass::boxed::BoxedType for ErrorValue {
    const NAME: &'static str = "ErrorValue";

    glib_boxed_type!();
}

#[cfg(feature = "v1_10")]
glib_boxed_derive_traits!(ErrorValue);

fn configure_encodebin(encodebin: &gst::Element) -> Result<(), Error> {
    // To tell the encodebin what we want it to produce, we create an EncodingProfile
    // https://gstreamer.freedesktop.org/data/doc/gstreamer/head/gst-plugins-base-libs/html/GstEncodingProfile.html
    // This profile consists of information about the contained audio and video formats
    // as well as the container format we want everything to be combined into.

    // Every audiostream piped into the encodebin should be encoded using vorbis.
    // let audio_profile = gst_pbutils::EncodingAudioProfileBuilder::new()
    //     // .format(&gst::Caps::new_simple("audio/mpeg,mpegversion=4", &[]))
    //     .format(&gst::Caps::from_string("audio/mpeg,mpegversion=4,rate=48000,channel=2,bitrate=320").unwrap())
    //     .presence(0)
    //     .build()?;

    // Every videostream piped into the encodebin should be encoded using theora.
    let video_profile = gst_pbutils::EncodingVideoProfileBuilder::new()
        // .format(&gst::Caps::from_string("video/x-h264,width=1920,height=1080,framerate=30000/1001").unwrap())
        // WORKS IN VLC, NOT IN QT
        .format(&gst::Caps::from_string("video/x-h264,width=1280,height=720,framerate=30000/1001,bitrate=10000,bframes=2,key-int-max=60,pass=pass1,preset=veryfast").unwrap())
        // .format(&gst::Caps::from_string("video/x-h264,width=1280,height=720,framerate=30000/1001,bitrate=10000,bframes=2,key-int-max=60,pass=pass1,preset=veryfast").unwrap())
        // .format(&gst::Caps::new_simple("video/x-h264", &[]))
        .presence(0)
        .build()?;

    // All streams are then finally combined into a matroska container.
    let container_profile = gst_pbutils::EncodingContainerProfileBuilder::new()
        .name("container")
        // .format(&gst::Caps::new_simple("video/x-matroska", &[]))
        .format(&gst::Caps::from_string("video/quicktime,variant=iso").unwrap())
        .add_profile(&(video_profile))
        // .add_profile(&(audio_profile))
        .build()?;

    // Finally, apply the EncodingProfile onto our encodebin element.
    encodebin
        .set_property("profile", &container_profile)
        .expect("set profile property failed");

    Ok(())
}

fn main() -> Result<(), Error> {
    gst::init()?;

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

    let pipeline = gst::Pipeline::new(None);
    let encodebin =
        gst::ElementFactory::make("encodebin", None).ok_or(MissingElement("encodebin"))?;

    let sink = gst::ElementFactory::make("filesink", None).ok_or(MissingElement("filesink"))?;
    sink.set_property("location", &output_file)
        .expect("setting location property failed");

    // Configure the encodebin.
    // Here we tell the bin what format we expect it to create at its output.
    configure_encodebin(&encodebin)?;

    pipeline
        .add_many(&[&encodebin, &sink])
        .expect("failed to add elements to pipeline");
    // It is clear from the start, that encodebin has only one src pad, so we can
    // directly link it to our filesink without problems.
    // The caps of encodebin's src-pad are set after we configured the encoding-profile.
    // (But filesink doesn't really care about the caps at its input anyway)
    gst::Element::link_many(&[&encodebin, &sink])?;

    // VIDEO
    let video_src =
        gst::ElementFactory::make("v4l2src", None).ok_or(MissingElement("v4l2src"))?;
    // let video_src =
    //     gst::ElementFactory::make("videotestsrc", None).ok_or(MissingElement("videotestsrc"))?;
    let video_convert = gst::ElementFactory::make("videoconvert", None)
        .ok_or(MissingElement("videoconvert"))?;
    let video_scale = gst::ElementFactory::make("videoscale", None)
        .ok_or(MissingElement("videoscale"))?;
    let video_queue =
        gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue"))?;

    let encoder = gst::ElementFactory::make("x264enc", None).unwrap();
    let mux = gst::ElementFactory::make("mp4mux", None).unwrap();

    // let video_elements = &[&video_src, &video_queue, &video_convert, &video_scale, &encoder, &mux, &sink];
    let video_elements = &[&video_src, &video_queue, &video_convert, &video_scale];
    pipeline
        .add_many(video_elements)
        .expect("failed to add video elements to pipeline");
    gst::Element::link_many(video_elements)?;

    // Request a sink pad from our encodebin, that can handle a raw videostream.
    // The encodebin will then automatically create an internal pipeline, that encodes
    // the audio stream in the format we specified in the EncodingProfile.
    let enc_video_sink_pad = encodebin
        .get_request_pad("video_%u")
        .expect("Could not get video pad from encodebin");
    let video_src_pad = video_scale
        .get_static_pad("src")
        .expect("videoqueue has no srcpad");
    video_src_pad.link(&enc_video_sink_pad)?;

    // AUDIO
    // let audio_src =
    //     gst::ElementFactory::make("alsasrc", None).ok_or(MissingElement("alsasrc"))?;
    // let audio_queue =
    //     gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue"))?;
    // let audio_convert = gst::ElementFactory::make("audioconvert", None)
    //     .ok_or(MissingElement("audioconvert"))?;
    // let audio_resample = gst::ElementFactory::make("audioresample", None)
    //     .ok_or(MissingElement("audioresample"))?;

    // let audio_elements = &[&audio_src, &audio_queue, &audio_convert, &audio_resample];
    // pipeline
    //     .add_many(audio_elements)
    //     .expect("failed to add audio elements to pipeline");
    // gst::Element::link_many(audio_elements)?;

    // Request a sink pad from our encodebin, that can handle a raw audiostream.
    // The encodebin will then automatically create an internal pipeline, that encodes
    // the audio stream in the format we specified in the EncodingProfile.
    // let enc_audio_sink_pad = encodebin
    //     .get_request_pad("audio_%u")
    //     .expect("Could not get audio pad from encodebin");
    // let audio_src_pad = audio_resample
    //     .get_static_pad("src")
    //     .expect("resample has no srcpad");
    // audio_src_pad.link(&enc_audio_sink_pad)?;


    let timeout_pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop. This closure will be executed
    // in an interval of 5 seconds. The return value of the handler function
    // determines whether the handler still wants to be called:
    // - glib::Continue(false) - stop calling this handler, remove timeout
    // - glib::Continue(true) - continue calling this handler
    glib::timeout_add_seconds(5, move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match timeout_pipeline_weak.upgrade() {
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

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    let main_loop_clone = main_loop.clone();
    let bus_watch_pipeline_weak = pipeline.downgrade();

    // for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
    bus.add_watch(move |_, msg| {
        println!("Received a message on the bus");
        use gst::MessageView;

        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("Received Eos message on bus. Quitting main loop.");
                main_loop.quit();
            }
            MessageView::Error(err) => {
                let pipeline = match bus_watch_pipeline_weak.upgrade() {
                    Some(pipeline) => pipeline,
                    None => return glib::Continue(false),
                };
                pipeline
                    .set_state(gst::State::Null)
                    .expect("Unable to set the pipeline to the `Null` state");

                #[cfg(feature = "v1_10")]
                {
                    match err.get_details() {
                        Some(details) if details.get_name() == "error-details" => details
                            .get::<&ErrorValue>("error")
                            .cloned()
                            .and_then(|v| v.0.lock().unwrap().take())
                            .map(Result::Err)
                            .expect("error-details message without actual error"),
                        _ => Err(ErrorMessage {
                            src: msg
                                .get_src()
                                .map(|s| String::from(s.get_path_string()))
                                .unwrap_or_else(|| String::from("None")),
                            error: err.get_error().description().into(),
                            debug: Some(err.get_debug().unwrap().to_string()),
                            cause: err.get_error(),
                        }
                        .into()),
                    }?;
                }
                // #[cfg(not(feature = "v1_10"))]
                // {
                //     Err(ErrorMessage {
                //         src: msg
                //             .get_src()
                //             .map(|s| String::from(s.get_path_string()))
                //             .unwrap_or_else(|| String::from("None")),
                //         error: err.get_error().description().into(),
                //         debug: Some(err.get_debug().unwrap().to_string()),
                //         cause: err.get_error(),
                //     })
                //         .expect("Could not generator error.");
                // }
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.get_src().map(|s| s.get_path_string()),
                    s.get_old(),
                    s.get_current(),
                    s.get_pending()
                );
            }
            _ => (),
        }
        // Tell the mainloop to continue executing this callback.
        glib::Continue(true)
    });

    // Operate GStreamer's bus, facilliating GLib's mainloop here.
    // This function call will block until you tell the mainloop to quit
    // (see above for how to do this).
    main_loop.run();

    println!("Main loop returned. Setting pipeline state to Null");
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    // Remove the watch function from the bus.
    // Again: There can always only be one watch function.
    // Thus we don't have to tell him which function to remove.
    bus.remove_watch().unwrap();

    // pipeline.set_state(gst::State::Null)?;

    Ok(())
}
