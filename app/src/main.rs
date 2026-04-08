#![cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]

mod pages;

use pages::home_state::{bind_home_callbacks, initialize_home_page};
use router::init_router;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 1) 启动 tokio runtime（后台线程池）
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    // 2) server 后台跑（不要阻塞 UI）
    rt.spawn(async {
        if let Err(err) = init_router().await {
            eprintln!("server startup failed: {err}");
            // 一般不建议在这里 std::process::exit(1)，会把整个 UI 一起干掉
        }
    });

    // 3) UI 必须在主线程跑（run 会阻塞）
    let main_window = App::new()?;
    // 初始化首页默认展示数据，并绑定后续接后端时要用到的空回调。
    initialize_home_page(&main_window);
    bind_home_callbacks(&main_window);
    main_window.run()
}
