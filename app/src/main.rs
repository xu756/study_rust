#![cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]

use futures::StreamExt;
use gst::prelude::*;
use gst::MessageView;
use gst_app::AppSink;
use gst_video::VideoInfo;

slint::include_modules!();

fn build_pipeline(uri: &str) -> Result<(gst::Pipeline, AppSink), anyhow::Error> {
    // 例：RTSP
    // rtspsrc location=... latency=200 ! decodebin ! videoconvert ! video/x-raw,format=RGBA ! appsink name=sink
    //
    // 例：HTTP/HLS/文件，用 playbin 更省事，但 appsink 接法不同；这里给你统一用 decodebin 方式。
    //
    // 如果你要 HTTP MP4：souphttpsrc location=... ! qtdemux ! decodebin ...
    // 更通用做法：uridecodebin
    //
    // 推荐：uridecodebin
    let pipeline_str = format!(
        "uridecodebin uri=\"{uri}\" name=dec \
         dec. ! queue ! videoconvert ! videoscale ! video/x-raw,format=RGBA ! appsink name=sink sync=false max-buffers=2 drop=true"
    );

    let pipeline = gst::parse::launch(&pipeline_str)?
        .downcast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("not a pipeline"))?;

    let sink = pipeline
        .by_name("sink")
        .ok_or_else(|| anyhow::anyhow!("appsink not found"))?
        .downcast::<AppSink>()
        .map_err(|_| anyhow::anyhow!("element is not an appsink"))?;

    // 让 appsink 发 sample 信号（pull-sample 模式也可以，这里用 pull_sample）
    sink.set_property("emit-signals", &false);
    sink.set_property("sync", &false);

    Ok((pipeline, sink))
}

fn main() -> Result<(), slint::PlatformError> {
    // 1) tokio runtime（后台线程池）
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    rt.spawn(async {
        if let Err(err) = router::init_router().await {
            eprintln!("server startup failed: {err}");
        }
    });

    // 2) UI
    let app = App::new()?;

    // 你可以在这里把 uri 从配置/输入框拿，这里先写死演示：
    // RTSP: "rtsp://user:pass@ip/xxx"
    // 文件: "file:///home/you/video.mp4"
    // HTTP: "https://example.com/video.mp4"
    let uri = "rtsp://127.0.0.1:8554/live"; // 改成你的流地址

    // 3) 启动 GStreamer 播放线程（不要阻塞 UI）
    {
        let app_handle = app.as_weak();
        std::thread::spawn(move || {
            if let Err(e) = run_gst(uri, app_handle) {
                eprintln!("gstreamer error: {e:#}");
            }
        });
    }

    // 4) UI run（主线程阻塞）
    app.run()
}

fn run_gst(uri: &str, app_handle: slint::Weak<App>) -> Result<(), anyhow::Error> {
    gst::init()?;

    let (pipeline, sink) = build_pipeline(uri)?;

    // 用于解析帧宽高/格式
    let mut vinfo: Option<VideoInfo> = None;

    pipeline.set_state(gst::State::Playing)?;

    // bus 处理错误/结束
    let bus = pipeline.bus().unwrap();

    loop {
        // 1) 从 appsink 拉一帧
        if let Some(sample) = sink.try_pull_sample(gst::ClockTime::from_mseconds(30)) {
            let caps = sample.caps().ok_or_else(|| anyhow::anyhow!("no caps"))?;
            if vinfo.is_none() {
                vinfo = Some(VideoInfo::from_caps(&caps)?);
            }
            let info = vinfo.as_ref().unwrap();

            let buffer = sample
                .buffer()
                .ok_or_else(|| anyhow::anyhow!("no buffer"))?;
            let map = buffer.map_readable()?;
            let data = map.as_slice();

            // 1) 在 gst 线程里只准备“可 Send”的数据
            let width = info.width() as usize;
            let height = info.height() as usize;

            let expected = width * height * 4;
            if data.len() < expected {
                continue;
            }

            // 复制一份 RGBA 数据（Vec<u8> 是 Send）
            let rgba = data[..expected].to_vec();

            // 2) 把 Vec<u8> + 宽高 发到 UI 线程，在 UI 线程里创建 Slint Image
            let _ = slint::invoke_from_event_loop({
                let app_handle = app_handle.clone();
                move || {
                    if let Some(app) = app_handle.upgrade() {
                        let mut pb = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(
                            width as u32,
                            height as u32,
                        );
                        pb.make_mut_bytes().copy_from_slice(&rgba);
                        let image = slint::Image::from_rgba8(pb);

                        app.set_video_frame(image);
                        app.set_buffering_percent(100);
                    }
                }
            });
        }

        // 2) 同时读 bus 消息（错误/断流）
        while let Some(msg) = bus.pop_filtered(&[gst::MessageType::Error, gst::MessageType::Eos]) {
            match msg.view() {
                MessageView::Error(err) => {
                    eprintln!(
                        "gst error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    pipeline.set_state(gst::State::Null)?;
                    return Err(anyhow::anyhow!("gstreamer error"));
                }
                MessageView::Eos(..) => {
                    eprintln!("gst eos");
                    pipeline.set_state(gst::State::Null)?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
