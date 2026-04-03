//! 首页状态管理与 Slint 视图映射。
//!
//! 后端接入时可以只关注两件事：
//! 1. 实现 [`HomePageAdapter`]，从串口、TCP、HTTP 或数据库读取首页数据。
//! 2. 在拿到最新业务数据后构造 [`HomePageState`]，再交给 `apply_home_page_state` 渲染到界面。

use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{
    App, HomeDeviceStatusColor, HomeDeviceStatusItem, HomeLogItem, HomeLogLevel,
    HomeRealtimeFieldItem,
};

/// 设备状态指示灯的颜色快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStatusColorSnapshot {
    Success,
    Warning,
    Danger,
}

/// 设备状态区单行数据。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatusSnapshot {
    pub label: String,
    pub indicator_color: DeviceStatusColorSnapshot,
}

/// 运行日志级别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevelSnapshot {
    Info,
    Success,
    Warning,
    Error,
}

/// 首页日志卡片中的一条日志记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogItemSnapshot {
    pub level: LogLevelSnapshot,
    pub timestamp: String,
    pub message: String,
}

/// 顶部重量卡片的主读数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightSnapshot {
    pub value: String,
    pub unit_label: String,
    pub unit: String,
}

/// 实时信息区左侧的动态字段。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealtimeFieldSnapshot {
    pub label: String,
    pub value: String,
    pub always_show_when_empty: bool,
}

/// 首页所需的完整状态快照。
///
/// 这个结构体是 Rust 侧和 Slint 首页之间的边界模型。
/// 后端只要组织出这份数据，就能完整驱动首页。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomePageState {
    pub weight: WeightSnapshot,
    pub current_step_index: i32,
    pub device_statuses: Vec<DeviceStatusSnapshot>,
    pub process_step_labels: Vec<String>,
    pub log_items: Vec<LogItemSnapshot>,
    pub realtime_fields: Vec<RealtimeFieldSnapshot>,
    pub gross_weight: String,
    pub tare_weight: String,
    pub net_weight: String,
}

/// 首页数据适配器。
///
/// 后续接真实后端时，建议把串口采集、接口调用、数据库读取等逻辑放到
/// 适配器实现里，而不是直接散落在 UI 初始化代码中。
pub trait HomePageAdapter {
    fn load_home_state(&self) -> HomePageState;

    fn refresh_home_state(&self) -> HomePageState {
        self.load_home_state()
    }

    fn select_process_step(&self, _step_index: i32) {}

    fn switch_monitor_panel(&self, _panel_index: i32) {}
}

/// 默认演示数据提供者。
#[derive(Debug, Default, Clone, Copy)]
pub struct DemoHomePageAdapter;

impl HomePageAdapter for DemoHomePageAdapter {
    fn load_home_state(&self) -> HomePageState {
        build_home_bootstrap_state()
    }
}

/// 构造首页初始展示数据。
///
/// 当后端尚未接入时，界面会先使用这里的演示数据完成渲染。
pub fn build_home_bootstrap_state() -> HomePageState {
    HomePageState {
        weight: WeightSnapshot {
            value: "0".to_string(),
            unit_label: "单位".to_string(),
            unit: "KG".to_string(),
        },
        current_step_index: 0,
        device_statuses: vec![
            DeviceStatusSnapshot {
                label: "连接".to_string(),
                indicator_color: DeviceStatusColorSnapshot::Success,
            },
            DeviceStatusSnapshot {
                label: "通讯".to_string(),
                indicator_color: DeviceStatusColorSnapshot::Warning,
            },
            DeviceStatusSnapshot {
                label: "稳定".to_string(),
                indicator_color: DeviceStatusColorSnapshot::Danger,
            },
        ],
        process_step_labels: vec![
            "实时上磅".to_string(),
            "识别车牌".to_string(),
            "称量中".to_string(),
            "完成下磅".to_string(),
        ],
        log_items: vec![
            LogItemSnapshot {
                level: LogLevelSnapshot::Warning,
                timestamp: "2026-04-03 09:20:12".to_string(),
                message: "地磅首页已切换到新的布局骨架，等待后端日志接入。".to_string(),
            },
            LogItemSnapshot {
                level: LogLevelSnapshot::Success,
                timestamp: "2026-04-03 09:20:36".to_string(),
                message: "设备状态卡片初始化完成，当前使用前端预置数据。".to_string(),
            },
            LogItemSnapshot {
                level: LogLevelSnapshot::Info,
                timestamp: "2026-04-03 09:21:08".to_string(),
                message: "监控区域与实时数据区域暂时保留占位，后续再接业务模块。".to_string(),
            },
            LogItemSnapshot {
                level: LogLevelSnapshot::Error,
                timestamp: "2026-04-03 09:21:40".to_string(),
                message: "当前错误日志仅用于演示颜色效果，未接入真实异常流。".to_string(),
            },
        ],
        realtime_fields: vec![
            RealtimeFieldSnapshot {
                label: "车牌号".to_string(),
                value: "苏D66278".to_string(),
                always_show_when_empty: true,
            },
            RealtimeFieldSnapshot {
                label: "货物名称".to_string(),
                value: "混凝土".to_string(),
                always_show_when_empty: true,
            },
            RealtimeFieldSnapshot {
                label: "收货单位".to_string(),
                value: "江苏海邦建设".to_string(),
                always_show_when_empty: true,
            },
            RealtimeFieldSnapshot {
                label: "过磅单位".to_string(),
                value: "海邦地磅一号".to_string(),
                always_show_when_empty: false,
            },
            RealtimeFieldSnapshot {
                label: "时间".to_string(),
                value: "2026-04-03 09:21:40".to_string(),
                always_show_when_empty: true,
            },
        ],
        gross_weight: "0".to_string(),
        tare_weight: "0".to_string(),
        net_weight: "0".to_string(),
    }
}

/// 使用默认演示适配器初始化首页。
pub fn initialize_home_page(app: &App) {
    initialize_home_page_with(app, &DemoHomePageAdapter);
}

/// 使用自定义适配器初始化首页。
///
/// 真实后端接入时，通常由这里加载第一屏数据。
pub fn initialize_home_page_with(app: &App, adapter: &dyn HomePageAdapter) {
    let bootstrap_state = adapter.load_home_state();
    apply_home_page_state(app, &bootstrap_state);
}

/// 绑定默认演示回调。
pub fn bind_home_callbacks(app: &App) {
    bind_home_callbacks_with(app, Rc::new(DemoHomePageAdapter));
}

/// 绑定首页 UI 回调到适配器。
///
/// 这里统一处理刷新、步骤切换和监控面板切换等事件。
pub fn bind_home_callbacks_with(app: &App, adapter: Rc<dyn HomePageAdapter>) {
    let app_weak = app.as_weak();

    // 预留首页刷新入口，后续接后端时在这里拉取最新数据。
    let refresh_adapter = adapter.clone();
    app.on_request_refresh_home_data(move || {
        if let Some(app) = app_weak.upgrade() {
            let bootstrap_state = refresh_adapter.refresh_home_state();
            apply_home_page_state(&app, &bootstrap_state);
        }
    });

    // 预留流程步骤切换入口，当前只保留函数定义，不处理业务状态流转。
    let step_adapter = adapter.clone();
    app.on_request_select_process_step(move |step_index| {
        step_adapter.select_process_step(step_index);
    });

    // 预留监控面板切换入口，后续在这里切换摄像头或识别视图。
    app.on_request_switch_monitor_panel(move |panel_index| {
        adapter.switch_monitor_panel(panel_index);
    });
}

/// 把 Rust 侧状态快照同步到 Slint 首页属性。
///
/// 这是首页渲染的唯一入口，方便后面从硬件线程或后端事件里集中更新 UI。
pub fn apply_home_page_state(app: &App, state: &HomePageState) {
    app.set_weight_value(state.weight.value.clone().into());
    app.set_weight_unit_label(state.weight.unit_label.clone().into());
    app.set_weight_unit(state.weight.unit.clone().into());
    app.set_current_step_index(state.current_step_index);

    // 设备状态区。
    let device_statuses = state
        .device_statuses
        .iter()
        .cloned()
        .map(|status| HomeDeviceStatusItem {
            label: status.label.into(),
            indicator_color: to_slint_device_status_color(status.indicator_color),
        })
        .collect::<Vec<_>>();
    app.set_device_statuses(ModelRc::new(VecModel::from(device_statuses)));

    // 右上角流程步骤区。
    let process_step_labels = state
        .process_step_labels
        .iter()
        .cloned()
        .map(Into::into)
        .collect::<Vec<_>>();
    app.set_process_step_labels(ModelRc::new(VecModel::from(process_step_labels)));

    // 运行日志区。
    let log_items = state
        .log_items
        .iter()
        .cloned()
        .map(|log| HomeLogItem {
            level: to_slint_log_level(log.level),
            timestamp: log.timestamp.into(),
            message: log.message.into(),
        })
        .collect::<Vec<_>>();
    app.set_log_items(ModelRc::new(VecModel::from(log_items)));

    // 左下角实时信息区。空值字段在这里统一过滤，避免 Slint 层做复杂判断。
    let realtime_fields = state
        .realtime_fields
        .iter()
        .filter(|field| field.always_show_when_empty || !field.value.trim().is_empty())
        .cloned()
        .map(|field| HomeRealtimeFieldItem {
            label: field.label.into(),
            value: field.value.into(),
            always_show_when_empty: field.always_show_when_empty,
        })
        .collect::<Vec<_>>();
    app.set_realtime_fields(ModelRc::new(VecModel::from(realtime_fields)));

    // 固定重量区。空字符串统一按 0 处理，便于界面稳定显示。
    app.set_gross_weight(normalize_weight(&state.gross_weight));
    app.set_tare_weight(normalize_weight(&state.tare_weight));
    app.set_net_weight(normalize_weight(&state.net_weight));
}

/// 把 Rust 日志级别映射到 Slint 枚举。
fn to_slint_log_level(level: LogLevelSnapshot) -> HomeLogLevel {
    match level {
        LogLevelSnapshot::Info => HomeLogLevel::Info,
        LogLevelSnapshot::Success => HomeLogLevel::Success,
        LogLevelSnapshot::Warning => HomeLogLevel::Warning,
        LogLevelSnapshot::Error => HomeLogLevel::Error,
    }
}

/// 把 Rust 设备状态颜色映射到 Slint 枚举。
fn to_slint_device_status_color(color: DeviceStatusColorSnapshot) -> HomeDeviceStatusColor {
    match color {
        DeviceStatusColorSnapshot::Success => HomeDeviceStatusColor::Success,
        DeviceStatusColorSnapshot::Warning => HomeDeviceStatusColor::Warning,
        DeviceStatusColorSnapshot::Danger => HomeDeviceStatusColor::Danger,
    }
}

/// 规范化重量字符串。
///
/// 真实业务里有些重量在某个阶段还没有采集到，此时统一显示为 `0`。
fn normalize_weight(value: &str) -> SharedString {
    if value.trim().is_empty() {
        "0".into()
    } else {
        value.into()
    }
}
