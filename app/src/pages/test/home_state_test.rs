use crate::pages::home_state::*;
use crate::App;
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
        log_items: vec![LogItemSnapshot {
            level: LogLevelSnapshot::Info,
            timestamp: "2026-04-03 10:00:00".to_string(),
            message: "测试日志 1".to_string(),
        }],
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
                label: "时间".to_string(),
                value: "2026-04-03 10:00:08".to_string(),
                always_show_when_empty: true,
            },
            RealtimeFieldSnapshot {
                label: "发卡号".to_string(),
                value: "".to_string(),
                always_show_when_empty: false,
            },
        ],
        recent_weight_records: vec![
            "2026-04-03 10:00:08 | 苏D66278 | 混凝土 | 毛重 1280 | 皮重 0 | 净重 1280".to_string(),
            "2026-04-03 09:58:24 | 苏D66373 | 混凝土 | 毛重 2400 | 皮重 1450 | 净重 950".to_string(),
        ],
        gross_weight: weight_value.to_string(),
        tare_weight: "".to_string(),
        net_weight: weight_value.to_string(),
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
fn initialize_home_page_preserves_slint_defaults() {
    ensure_slint_test_backend();
    let app = App::new().expect("app should construct in tests");

    assert_eq!(app.get_weight_value().to_string(), "866");
    assert_eq!(app.get_log_items().row_count(), 4);

    initialize_home_page(&app);

    assert_eq!(app.get_weight_value().to_string(), "866");
    assert_eq!(app.get_log_items().row_count(), 4);
    assert_eq!(app.get_process_step_labels().row_count(), 4);
}

#[test]
fn main_and_pages_module_point_to_home_state_module() {
    let main_rs = read_project_file("src/main.rs");
    let pages_mod = read_project_file("src/pages/mod.rs");
    let test_mod = read_project_file("src/pages/test/mod.rs");

    assert!(main_rs.contains("mod pages;"));
    assert!(main_rs.contains("use pages::home_state::{bind_home_callbacks, initialize_home_page};"));
    assert!(pages_mod.contains("pub mod home_state;"));
    assert!(pages_mod.contains("#[cfg(test)]"));
    assert!(pages_mod.contains("mod test;"));
    assert!(test_mod.contains("mod home_state_test;"));
}

#[test]
fn home_state_exposes_adapter_contract_for_future_device_integration() {
    let source = read_project_file("src/pages/home_state.rs");
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
    assert!(
        !production_source.contains("build_home_bootstrap_state"),
        "home_state.rs should no longer define demo bootstrap values in Rust once defaults move to Slint"
    );
}

#[test]
fn home_state_tests_are_split_into_separate_module_file() {
    let pages_mod = read_project_file("src/pages/mod.rs");
    let test_mod = read_project_file("src/pages/test/mod.rs");
    let tests = read_project_file("src/pages/test/home_state_test.rs");

    assert!(pages_mod.contains("#[cfg(test)]"));
    assert!(pages_mod.contains("mod test;"));
    assert!(test_mod.contains("mod home_state_test;"));
    assert!(tests.contains("fn initialize_home_page_preserves_slint_defaults()"));
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
fn realtime_panel_slint_contracts_are_declared() {
    let models = read_project_file("dashboard/pages/home/models.slint");
    let app = read_project_file("dashboard/app.slint");
    let page = read_project_file("dashboard/pages/home/page.slint");

    assert!(models.contains("export struct HomeRealtimeFieldItem"));
    assert!(models.contains("always_show_when_empty: bool"));

    assert!(app.contains("in-out property <[HomeRealtimeFieldItem]> realtime_fields"));
    assert!(app.contains("in-out property <[string]> recent_weight_records"));
    assert!(app.contains("in-out property <string> gross_weight"));
    assert!(app.contains("in-out property <string> tare_weight"));
    assert!(app.contains("in-out property <string> net_weight"));

    assert!(page.contains("in property <[HomeRealtimeFieldItem]> realtime_fields"));
    assert!(page.contains("in property <[string]> recent_weight_records"));
    assert!(page.contains("in property <string> gross_weight"));
    assert!(page.contains("in property <string> tare_weight"));
    assert!(page.contains("in property <string> net_weight"));
}

#[test]
fn slint_defaults_are_declared_for_runtime_and_preview() {
    let models = read_project_file("dashboard/pages/home/models.slint");
    let app = read_project_file("dashboard/app.slint");
    let page = read_project_file("dashboard/pages/home/page.slint");

    assert!(models.contains("export global HomePageDefaults"));
    assert!(models.contains("out property <string> weight_value: \"866\""));
    assert!(models.contains("out property <[HomeLogItem]> log_items: ["));
    assert!(models.contains("out property <[HomeRealtimeFieldItem]> realtime_fields: ["));

    assert!(app.contains("HomePageDefaults.weight_value"));
    assert!(app.contains("HomePageDefaults.log_items"));
    assert!(page.contains("HomePageDefaults.weight_value"));
    assert!(page.contains("HomePageDefaults.log_items"));
}

#[test]
fn realtime_panel_initialize_home_page_with_applies_state() {
    ensure_slint_test_backend();
    let app = App::new().expect("app should construct in tests");
    let adapter = MockHomePageAdapter::new(build_test_home_state("1280"));

    initialize_home_page_with(&app, &adapter);

    assert_eq!(app.get_realtime_fields().row_count(), 4);
    assert_eq!(
        app.get_realtime_fields()
            .row_data(0)
            .expect("first realtime field should exist")
            .label
            .to_string(),
        "车牌号"
    );
    assert_eq!(app.get_gross_weight().to_string(), "1280");
    assert_eq!(app.get_tare_weight().to_string(), "0");
    assert_eq!(app.get_net_weight().to_string(), "1280");
    assert_eq!(app.get_recent_weight_records().row_count(), 2);
    assert_eq!(
        app.get_recent_weight_records()
            .row_data(0)
            .expect("first recent record should exist")
            .to_string(),
        "2026-04-03 10:00:08 | 苏D66278 | 混凝土 | 毛重 1280 | 皮重 0 | 净重 1280"
    );
    assert_eq!(app.get_weight_value().to_string(), "1280");
    assert_eq!(app.get_weight_unit_label().to_string(), "单位");
    assert_eq!(app.get_weight_unit().to_string(), "KG");
    assert_eq!(app.get_current_step_index(), 2);
    assert_eq!(app.get_device_statuses().row_count(), 2);
    assert_eq!(app.get_process_step_labels().row_count(), 3);
    assert_eq!(app.get_log_items().row_count(), 1);
    assert_eq!(
        app.get_log_items()
            .row_data(0)
            .expect("first log item should exist")
            .message
            .to_string(),
        "测试日志 1"
    );
}

#[test]
fn realtime_panel_field_row_component_is_declared() {
    let row = read_project_file("dashboard/pages/home/components/realtime-field-row.slint");
    let exports = read_project_file("dashboard/pages/home/components/page.slint");

    assert!(row.contains("export component RealtimeFieldRow"));
    assert!(row.contains("read-only: true"));
    assert!(row.contains("UInput"));
    assert!(exports.contains("export { RealtimeFieldRow }"));
}

#[test]
fn realtime_panel_info_card_replaces_left_placeholder() {
    let card = read_project_file("dashboard/pages/home/components/realtime-info-card.slint");
    let exports = read_project_file("dashboard/pages/home/components/page.slint");
    let page = read_project_file("dashboard/pages/home/page.slint");

    assert!(card.contains("export component RealtimeInfoCard"));
    assert!(card.contains("text: \"最近称重记录\""));
    assert!(card.contains("for field in root.realtime_fields"));
    assert!(card.contains("for record[index] in root.recent_weight_records"));
    assert!(card.contains("field.always_show_when_empty || field.value != \"\""));
    assert!(card.contains("label: \"毛重\""));
    assert!(card.contains("label: \"皮重\""));
    assert!(card.contains("label: \"净重\""));

    assert!(exports.contains("export { RealtimeInfoCard }"));
    assert!(page.contains("RealtimeInfoCard {"));
    assert!(!page.contains("title: \"实时数据区域\""));
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
    assert_eq!(app.get_log_items().row_count(), 1);
    assert_eq!(app.get_realtime_fields().row_count(), 4);
}
