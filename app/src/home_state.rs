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
pub struct HomeBootstrapState {
    pub weight_value: String,
    pub weight_unit_label: String,
    pub weight_unit: String,
    pub current_step_index: i32,
    pub device_statuses: Vec<DeviceStatusSnapshot>,
    pub process_step_labels: Vec<String>,
    pub log_items: Vec<LogItemSnapshot>,
}

pub fn build_home_bootstrap_state() -> HomeBootstrapState {
    HomeBootstrapState {
        weight_value: "0".to_string(),
        weight_unit_label: "单位".to_string(),
        weight_unit: "KG".to_string(),
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
    let bootstrap_state = build_home_bootstrap_state();
    apply_home_bootstrap_state(app, &bootstrap_state);
}

pub fn bind_home_callbacks(app: &App) {
    let app_weak = app.as_weak();

    // 预留首页刷新入口，后续接后端时在这里拉取最新数据。
    app.on_request_refresh_home_data(move || {
        if let Some(app) = app_weak.upgrade() {
            let bootstrap_state = build_home_bootstrap_state();
            apply_home_bootstrap_state(&app, &bootstrap_state);
        }
    });

    // 预留流程步骤切换入口，当前只保留函数定义，不处理业务状态流转。
    app.on_request_select_process_step(|_step_index| {});

    // 预留监控面板切换入口，后续在这里切换摄像头或识别视图。
    app.on_request_switch_monitor_panel(|_panel_index| {});
}

fn apply_home_bootstrap_state(app: &App, state: &HomeBootstrapState) {
    app.set_weight_value(state.weight_value.clone().into());
    app.set_weight_unit_label(state.weight_unit_label.clone().into());
    app.set_weight_unit(state.weight_unit.clone().into());
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

    #[test]
    fn build_home_bootstrap_state_uses_weighing_defaults() {
        let state = build_home_bootstrap_state();

        assert_eq!(state.weight_value, "0");
        assert_eq!(state.weight_unit_label, "单位");
        assert_eq!(state.weight_unit, "KG");
        assert_eq!(state.current_step_index, 0);
        assert_eq!(state.device_statuses.len(), 3);
        assert_eq!(
            state.device_statuses[0].indicator_color,
            DeviceStatusColorSnapshot::Success
        );
        assert_eq!(state.process_step_labels.len(), 4);
        assert!(!state.log_items.is_empty());
    }
}
