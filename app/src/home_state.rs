use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, VecModel};

use crate::{App, HomeDeviceStatusColor, HomeDeviceStatusItem, HomeLogItem, HomeLogLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStatusColorSnapshot {
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatusSnapshot {
    pub label: String,
    pub indicator_color: DeviceStatusColorSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevelSnapshot {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogItemSnapshot {
    pub level: LogLevelSnapshot,
    pub timestamp: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightSnapshot {
    pub value: String,
    pub unit_label: String,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomePageState {
    pub weight: WeightSnapshot,
    pub current_step_index: i32,
    pub device_statuses: Vec<DeviceStatusSnapshot>,
    pub process_step_labels: Vec<String>,
    pub log_items: Vec<LogItemSnapshot>,
}

pub trait HomePageAdapter {
    fn load_home_state(&self) -> HomePageState;

    fn refresh_home_state(&self) -> HomePageState {
        self.load_home_state()
    }

    fn select_process_step(&self, _step_index: i32) {}

    fn switch_monitor_panel(&self, _panel_index: i32) {}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DemoHomePageAdapter;

impl HomePageAdapter for DemoHomePageAdapter {
    fn load_home_state(&self) -> HomePageState {
        build_home_bootstrap_state()
    }
}

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
    }
}

pub fn initialize_home_page(app: &App) {
    initialize_home_page_with(app, &DemoHomePageAdapter);
}

pub fn initialize_home_page_with(app: &App, adapter: &dyn HomePageAdapter) {
    let bootstrap_state = adapter.load_home_state();
    apply_home_page_state(app, &bootstrap_state);
}

pub fn bind_home_callbacks(app: &App) {
    bind_home_callbacks_with(app, Rc::new(DemoHomePageAdapter));
}

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

pub fn apply_home_page_state(app: &App, state: &HomePageState) {
    app.set_weight_value(state.weight.value.clone().into());
    app.set_weight_unit_label(state.weight.unit_label.clone().into());
    app.set_weight_unit(state.weight.unit.clone().into());
    app.set_current_step_index(state.current_step_index);

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

    let process_step_labels = state
        .process_step_labels
        .iter()
        .cloned()
        .map(Into::into)
        .collect::<Vec<_>>();
    app.set_process_step_labels(ModelRc::new(VecModel::from(process_step_labels)));

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
}

fn to_slint_log_level(level: LogLevelSnapshot) -> HomeLogLevel {
    match level {
        LogLevelSnapshot::Info => HomeLogLevel::Info,
        LogLevelSnapshot::Success => HomeLogLevel::Success,
        LogLevelSnapshot::Warning => HomeLogLevel::Warning,
        LogLevelSnapshot::Error => HomeLogLevel::Error,
    }
}

fn to_slint_device_status_color(color: DeviceStatusColorSnapshot) -> HomeDeviceStatusColor {
    match color {
        DeviceStatusColorSnapshot::Success => HomeDeviceStatusColor::Success,
        DeviceStatusColorSnapshot::Warning => HomeDeviceStatusColor::Warning,
        DeviceStatusColorSnapshot::Danger => HomeDeviceStatusColor::Danger,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::Model;
    use std::{
        cell::{Cell, RefCell},
        fs,
        path::PathBuf,
        rc::Rc,
    };

    fn ensure_slint_test_backend() {
        i_slint_backend_testing::init_no_event_loop();
    }

    fn read_project_file(relative_path: &str) -> String {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
        fs::read_to_string(path).expect("project file should be readable during tests")
    }

    fn build_test_home_state(weight_value: &str) -> HomePageState {
        HomePageState {
            weight: WeightSnapshot {
                value: weight_value.to_string(),
                unit_label: "单位".to_string(),
                unit: "KG".to_string(),
            },
            current_step_index: 2,
            device_statuses: vec![
                DeviceStatusSnapshot {
                    label: "连接".to_string(),
                    indicator_color: DeviceStatusColorSnapshot::Success,
                },
                DeviceStatusSnapshot {
                    label: "稳定".to_string(),
                    indicator_color: DeviceStatusColorSnapshot::Warning,
                },
            ],
            process_step_labels: vec!["上磅".to_string(), "称量".to_string(), "完成".to_string()],
            log_items: vec![
                LogItemSnapshot {
                    level: LogLevelSnapshot::Info,
                    timestamp: "2026-04-03 10:00:00".to_string(),
                    message: "测试日志 1".to_string(),
                },
                LogItemSnapshot {
                    level: LogLevelSnapshot::Success,
                    timestamp: "2026-04-03 10:00:08".to_string(),
                    message: "测试日志 2".to_string(),
                },
            ],
        }
    }

    #[derive(Clone)]
    struct MockHomePageAdapter {
        state: HomePageState,
        refresh_count: Rc<Cell<usize>>,
        selected_steps: Rc<RefCell<Vec<i32>>>,
        switched_panels: Rc<RefCell<Vec<i32>>>,
    }

    impl MockHomePageAdapter {
        fn new(state: HomePageState) -> Self {
            Self {
                state,
                refresh_count: Rc::new(Cell::new(0)),
                selected_steps: Rc::new(RefCell::new(Vec::new())),
                switched_panels: Rc::new(RefCell::new(Vec::new())),
            }
        }
    }

    impl HomePageAdapter for MockHomePageAdapter {
        fn load_home_state(&self) -> HomePageState {
            self.state.clone()
        }

        fn refresh_home_state(&self) -> HomePageState {
            self.refresh_count.set(self.refresh_count.get() + 1);
            self.state.clone()
        }

        fn select_process_step(&self, step_index: i32) {
            self.selected_steps.borrow_mut().push(step_index);
        }

        fn switch_monitor_panel(&self, panel_index: i32) {
            self.switched_panels.borrow_mut().push(panel_index);
        }
    }

    #[test]
    fn build_home_bootstrap_state_uses_weighing_defaults() {
        let state = build_home_bootstrap_state();

        assert_eq!(state.weight.value, "0");
        assert_eq!(state.weight.unit_label, "单位");
        assert_eq!(state.weight.unit, "KG");
        assert_eq!(state.current_step_index, 0);
        assert_eq!(state.device_statuses.len(), 3);
        assert_eq!(
            state.device_statuses[0].indicator_color,
            DeviceStatusColorSnapshot::Success
        );
        assert_eq!(state.process_step_labels.len(), 4);
        assert!(!state.log_items.is_empty());
    }

    #[test]
    fn home_state_exposes_adapter_contract_for_future_device_integration() {
        let source = read_project_file("src/home_state.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("home_state.rs should contain production code before the test module");

        assert!(
            production_source.contains("pub trait HomePageAdapter"),
            "home_state.rs should expose a HomePageAdapter trait for future hardware and device integration"
        );
        assert!(
            production_source.contains("pub fn initialize_home_page_with"),
            "home_state.rs should expose initialize_home_page_with for custom homepage state providers"
        );
        assert!(
            production_source.contains("pub fn bind_home_callbacks_with"),
            "home_state.rs should expose bind_home_callbacks_with for custom callback adapters"
        );
        assert!(
            production_source.contains("pub fn apply_home_page_state"),
            "home_state.rs should expose apply_home_page_state so backend updates can drive the UI directly"
        );
    }

    #[test]
    fn log_card_declares_vertical_scroll_content_contract() {
        let source = read_project_file("dashboard/pages/home/components/log-card.slint");

        assert!(
            source.contains("log_content := VerticalLayout"),
            "log-card.slint should render logs inside a dedicated vertical layout instead of stacking repeated items directly on the Flickable root"
        );
        assert!(
            source.contains("content-height: log_content.preferred-height"),
            "log-card.slint should bind the Flickable viewport height to the log content height for vertical scrolling"
        );
    }

    #[test]
    fn initialize_home_page_with_applies_custom_adapter_state() {
        ensure_slint_test_backend();
        let app = App::new().expect("app should construct in tests");
        let adapter = MockHomePageAdapter::new(build_test_home_state("1280"));

        initialize_home_page_with(&app, &adapter);

        assert_eq!(app.get_weight_value().to_string(), "1280");
        assert_eq!(app.get_weight_unit_label().to_string(), "单位");
        assert_eq!(app.get_weight_unit().to_string(), "KG");
        assert_eq!(app.get_current_step_index(), 2);
        assert_eq!(app.get_device_statuses().row_count(), 2);
        assert_eq!(app.get_process_step_labels().row_count(), 3);
        assert_eq!(app.get_log_items().row_count(), 2);
        assert_eq!(
            app.get_log_items()
                .row_data(1)
                .expect("second log item should exist")
                .message
                .to_string(),
            "测试日志 2"
        );
    }

    #[test]
    fn bind_home_callbacks_with_forwards_refresh_and_ui_actions() {
        ensure_slint_test_backend();
        let app = App::new().expect("app should construct in tests");
        let adapter = Rc::new(MockHomePageAdapter::new(build_test_home_state("2048")));

        bind_home_callbacks_with(&app, adapter.clone());

        app.invoke_request_select_process_step(3);
        app.invoke_request_switch_monitor_panel(1);
        app.invoke_request_refresh_home_data();

        assert_eq!(adapter.refresh_count.get(), 1);
        assert_eq!(adapter.selected_steps.borrow().as_slice(), &[3]);
        assert_eq!(adapter.switched_panels.borrow().as_slice(), &[1]);
        assert_eq!(app.get_weight_value().to_string(), "2048");
        assert_eq!(app.get_log_items().row_count(), 2);
    }
}
