# Home Realtime Panel Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the homepage left-bottom placeholder with a read-only realtime weighing panel that shows dynamic `label/value` fields on the left and fixed `毛重 / 皮重 / 净重` values on the right.

**Architecture:** Extend the existing homepage Slint model layer with realtime-field types and page properties, then map those values from `home_state.rs` through the generated `App` API. Build the UI as two focused components: a reusable read-only field row and a `RealtimeInfoCard` that filters dynamic fields inside the component while always showing the three fixed weight fields.

**Tech Stack:** Rust 2021, Slint 1.15, sleek-ui `UCard`/`UInput`, Cargo test/check/clippy

---

## File Structure

- Modify: `app/dashboard/pages/home/models.slint`
  - Add the shared Slint struct for realtime dynamic fields.
- Modify: `app/dashboard/app.slint`
  - Add top-level properties for realtime fields and fixed weights, then pass them into `HomePage`.
- Modify: `app/dashboard/pages/home/page.slint`
  - Add homepage input properties and replace the left-bottom placeholder with `RealtimeInfoCard`.
- Modify: `app/dashboard/pages/home/components/page.slint`
  - Export the new realtime components.
- Create: `app/dashboard/pages/home/components/realtime-field-row.slint`
  - Reusable read-only `label + UInput` row.
- Create: `app/dashboard/pages/home/components/realtime-info-card.slint`
  - Main realtime panel card, including dynamic-field filtering and fixed weight presentation.
- Modify: `app/src/home_state.rs`
  - Extend the Rust snapshot model, sample bootstrap data, and Slint property mapping. Add regression tests for Slint contracts and runtime state wiring.

### Task 1: Declare The Shared Slint Realtime Contracts

**Files:**
- Modify: `app/src/home_state.rs`
- Modify: `app/dashboard/pages/home/models.slint`
- Modify: `app/dashboard/app.slint`
- Modify: `app/dashboard/pages/home/page.slint`

- [ ] **Step 1: Write the failing source-contract test**

Add this test near the existing file-content assertions in `app/src/home_state.rs`:

```rust
#[test]
fn realtime_panel_slint_contracts_are_declared() {
    let models = read_project_file("dashboard/pages/home/models.slint");
    let app = read_project_file("dashboard/app.slint");
    let page = read_project_file("dashboard/pages/home/page.slint");

    assert!(models.contains("export struct HomeRealtimeFieldItem"));
    assert!(models.contains("always_show_when_empty: bool"));

    assert!(app.contains("in-out property <[HomeRealtimeFieldItem]> realtime_fields"));
    assert!(app.contains("in-out property <string> gross_weight"));
    assert!(app.contains("in-out property <string> tare_weight"));
    assert!(app.contains("in-out property <string> net_weight"));

    assert!(page.contains("in property <[HomeRealtimeFieldItem]> realtime_fields"));
    assert!(page.contains("in property <string> gross_weight"));
    assert!(page.contains("in property <string> tare_weight"));
    assert!(page.contains("in property <string> net_weight"));
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p app home_state::tests::realtime_panel_slint_contracts_are_declared -- --nocapture`

Expected: FAIL because `HomeRealtimeFieldItem` and the new homepage properties do not exist yet.

- [ ] **Step 3: Add the minimal Slint model and property declarations**

Update `app/dashboard/pages/home/models.slint`:

```slint
export struct HomeRealtimeFieldItem {
    label: string,
    value: string,
    always_show_when_empty: bool,
}
```

Update the imports at the top of `app/dashboard/app.slint`:

```slint
import {
    HomeDeviceStatusColor,
    HomeDeviceStatusItem,
    HomeLogItem,
    HomeLogLevel,
    HomeRealtimeFieldItem,
} from "pages/home/models.slint";
```

Add these properties in `app/dashboard/app.slint` below `log_items`:

```slint
in-out property <[HomeRealtimeFieldItem]> realtime_fields: [];
in-out property <string> gross_weight: "0";
in-out property <string> tare_weight: "0";
in-out property <string> net_weight: "0";
```

Pass them into `HomePage`:

```slint
realtime_fields <=> root.realtime_fields;
gross_weight <=> root.gross_weight;
tare_weight <=> root.tare_weight;
net_weight <=> root.net_weight;
```

Update the imports at the top of `app/dashboard/pages/home/page.slint`:

```slint
import { HomeDeviceStatusItem, HomeLogItem, HomeRealtimeFieldItem } from "models.slint";
```

Add these `HomePage` properties below `log_items`:

```slint
in property <[HomeRealtimeFieldItem]> realtime_fields: [];
in property <string> gross_weight: "0";
in property <string> tare_weight: "0";
in property <string> net_weight: "0";
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test -p app home_state::tests::realtime_panel_slint_contracts_are_declared -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add app/src/home_state.rs app/dashboard/pages/home/models.slint app/dashboard/app.slint app/dashboard/pages/home/page.slint
git commit -m "feat(home): declare realtime panel slint contracts"
```

### Task 2: Extend Rust Homepage State For The Realtime Panel

**Files:**
- Modify: `app/src/home_state.rs`

- [ ] **Step 1: Write the failing runtime tests**

In `app/src/home_state.rs`, add these tests after the existing adapter tests:

```rust
#[test]
fn build_home_bootstrap_state_includes_realtime_panel_defaults() {
    let state = build_home_bootstrap_state();

    assert!(!state.realtime_fields.is_empty());
    assert_eq!(state.realtime_fields[0].label, "车牌号");
    assert_eq!(state.gross_weight, "0");
    assert_eq!(state.tare_weight, "0");
    assert_eq!(state.net_weight, "0");
}

#[test]
fn initialize_home_page_with_applies_realtime_panel_state() {
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
}
```

Update the existing `build_test_home_state()` helper target shape to this:

```rust
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
                label: "时间".to_string(),
                value: "2026-04-03 10:00:08".to_string(),
                always_show_when_empty: true,
            },
        ],
        gross_weight: weight_value.to_string(),
        tare_weight: "".to_string(),
        net_weight: weight_value.to_string(),
    }
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test -p app realtime_panel -- --nocapture`

Expected: FAIL with compile errors because `HomePageState`, `MockHomePageAdapter`, and `apply_home_page_state()` do not know about realtime fields or fixed weights yet.

- [ ] **Step 3: Implement the Rust snapshot model and Slint mapping**

Extend the imports in `app/src/home_state.rs`:

```rust
use crate::{
    App, HomeDeviceStatusColor, HomeDeviceStatusItem, HomeLogItem, HomeLogLevel,
    HomeRealtimeFieldItem,
};
```

Add the new snapshot type and `HomePageState` fields:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealtimeFieldSnapshot {
    pub label: String,
    pub value: String,
    pub always_show_when_empty: bool,
}

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
```

Seed `build_home_bootstrap_state()` with concrete sample data:

```rust
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
        value: "2026-04-03 09:21:40".to_string(),
        always_show_when_empty: true,
    },
],
gross_weight: "0".to_string(),
tare_weight: "0".to_string(),
net_weight: "0".to_string(),
```

Add a normalization helper:

```rust
fn normalize_weight(value: &str) -> slint::SharedString {
    if value.trim().is_empty() {
        "0".into()
    } else {
        value.into()
    }
}
```

Map the new fields inside `apply_home_page_state()`:

```rust
let realtime_fields = state
    .realtime_fields
    .iter()
    .cloned()
    .map(|field| HomeRealtimeFieldItem {
        label: field.label.into(),
        value: field.value.into(),
        always_show_when_empty: field.always_show_when_empty,
    })
    .collect::<Vec<_>>();
app.set_realtime_fields(ModelRc::new(VecModel::from(realtime_fields)));

app.set_gross_weight(normalize_weight(&state.gross_weight));
app.set_tare_weight(normalize_weight(&state.tare_weight));
app.set_net_weight(normalize_weight(&state.net_weight));
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test -p app realtime_panel -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add app/src/home_state.rs
git commit -m "feat(home): add realtime panel state mapping"
```

### Task 3: Build The Reusable Read-Only Realtime Field Row

**Files:**
- Modify: `app/src/home_state.rs`
- Create: `app/dashboard/pages/home/components/realtime-field-row.slint`
- Modify: `app/dashboard/pages/home/components/page.slint`

- [ ] **Step 1: Write the failing component-contract test**

Add this test in `app/src/home_state.rs`:

```rust
#[test]
fn realtime_field_row_component_is_declared() {
    let row = read_project_file("dashboard/pages/home/components/realtime-field-row.slint");
    let exports = read_project_file("dashboard/pages/home/components/page.slint");

    assert!(row.contains("export component RealtimeFieldRow"));
    assert!(row.contains("read-only: true"));
    assert!(row.contains("UInput"));
    assert!(exports.contains("export { RealtimeFieldRow }"));
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p app home_state::tests::realtime_field_row_component_is_declared -- --nocapture`

Expected: FAIL because `realtime-field-row.slint` does not exist yet.

- [ ] **Step 3: Implement the row component and export it**

Create `app/dashboard/pages/home/components/realtime-field-row.slint`:

```slint
import { UInput, UText } from "@sleek-ui/widgets.slint";
import { UAppTheme } from "@sleek-ui/app-theme.slint";

export component RealtimeFieldRow inherits Rectangle {
    in property <string> label: "";
    in property <string> value: "";
    in property <length> label_width: 92px;
    in property <length> field_height: 40px;

    min-height: root.field_height;
    background: transparent;

    HorizontalLayout {
        spacing: 12px;

        UText {
            width: root.label_width;
            text: root.label;
            color: UAppTheme.text-secondary;
            vertical-alignment: center;
            font-size: 14px;
        }

        UInput {
            horizontal-stretch: 1;
            height: root.field_height;
            text: root.value;
            read-only: true;
        }
    }
}
```

Update `app/dashboard/pages/home/components/page.slint`:

```slint
export { WeightStatusCard } from "weight-status-card.slint";
export { LogCard } from "log-card.slint";
export { ProcessStepCard } from "process-step-card.slint";
export { PanelPlaceholderCard } from "panel-placeholder-card.slint";
export { RealtimeFieldRow } from "realtime-field-row.slint";
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test -p app home_state::tests::realtime_field_row_component_is_declared -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add app/src/home_state.rs app/dashboard/pages/home/components/realtime-field-row.slint app/dashboard/pages/home/components/page.slint
git commit -m "feat(home): add realtime field row component"
```

### Task 4: Build And Wire The RealtimeInfoCard

**Files:**
- Modify: `app/src/home_state.rs`
- Create: `app/dashboard/pages/home/components/realtime-info-card.slint`
- Modify: `app/dashboard/pages/home/components/page.slint`
- Modify: `app/dashboard/pages/home/page.slint`

- [ ] **Step 1: Write the failing integration contract test**

Add this test in `app/src/home_state.rs`:

```rust
#[test]
fn realtime_info_card_replaces_the_left_placeholder() {
    let card = read_project_file("dashboard/pages/home/components/realtime-info-card.slint");
    let exports = read_project_file("dashboard/pages/home/components/page.slint");
    let page = read_project_file("dashboard/pages/home/page.slint");

    assert!(card.contains("export component RealtimeInfoCard"));
    assert!(card.contains("text: \"称重信息\""));
    assert!(card.contains("for field in root.realtime_fields"));
    assert!(card.contains("field.always_show_when_empty || field.value != \"\""));
    assert!(card.contains("label: \"毛重\""));
    assert!(card.contains("label: \"皮重\""));
    assert!(card.contains("label: \"净重\""));

    assert!(exports.contains("export { RealtimeInfoCard }"));
    assert!(page.contains("RealtimeInfoCard {"));
    assert!(!page.contains("title: \"实时数据区域\""));
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p app home_state::tests::realtime_info_card_replaces_the_left_placeholder -- --nocapture`

Expected: FAIL because `realtime-info-card.slint` does not exist and the left placeholder is still present.

- [ ] **Step 3: Implement `RealtimeInfoCard` and replace the placeholder**

Create `app/dashboard/pages/home/components/realtime-info-card.slint`:

```slint
import { UCard, UText, UTitle } from "@sleek-ui/widgets.slint";
import { UAppTheme } from "@sleek-ui/app-theme.slint";
import { HomeRealtimeFieldItem } from "../models.slint";
import { RealtimeFieldRow } from "realtime-field-row.slint";

export component RealtimeInfoCard inherits UCard {
    in property <[HomeRealtimeFieldItem]> realtime_fields: [];
    in property <string> gross_weight: "0";
    in property <string> tare_weight: "0";
    in property <string> net_weight: "0";

    VerticalLayout {
        padding: 20px;
        spacing: 16px;

        HorizontalLayout {
            alignment: space-between;

            VerticalLayout {
                spacing: 6px;

                UTitle {
                    text: "称重信息";
                    level: 5;
                }

                UText {
                    text: "展示最新一条实时称重记录。";
                    color: UAppTheme.text-secondary;
                    wrap: word-wrap;
                }
            }
        }

        HorizontalLayout {
            spacing: 18px;

            Rectangle {
                horizontal-stretch: 3;
                background: transparent;

                VerticalLayout {
                    spacing: 12px;

                    for field in root.realtime_fields:
                    if field.always_show_when_empty || field.value != "": RealtimeFieldRow {
                        label: field.label;
                        value: field.value;
                    }
                }
            }

            Rectangle {
                horizontal-stretch: 2;
                background: transparent;

                VerticalLayout {
                    spacing: 12px;

                    RealtimeFieldRow { label: "毛重"; value: root.gross_weight; }
                    RealtimeFieldRow { label: "皮重"; value: root.tare_weight; }
                    RealtimeFieldRow { label: "净重"; value: root.net_weight; }
                }
            }
        }
    }
}
```

Update `app/dashboard/pages/home/components/page.slint`:

```slint
export { WeightStatusCard } from "weight-status-card.slint";
export { LogCard } from "log-card.slint";
export { ProcessStepCard } from "process-step-card.slint";
export { PanelPlaceholderCard } from "panel-placeholder-card.slint";
export { RealtimeFieldRow } from "realtime-field-row.slint";
export { RealtimeInfoCard } from "realtime-info-card.slint";
```

Update the imports in `app/dashboard/pages/home/page.slint`:

```slint
import {
    LogCard,
    PanelPlaceholderCard,
    ProcessStepCard,
    RealtimeInfoCard,
    WeightStatusCard,
} from "components/page.slint";
```

Replace the left-bottom `PanelPlaceholderCard` block in `app/dashboard/pages/home/page.slint` with:

```slint
RealtimeInfoCard {
    x: root.page_padding;
    y: root.bottom_row_y;
    width: root.bottom_left_width;
    height: root.bottom_card_height;
    realtime_fields: root.realtime_fields;
    gross_weight: root.gross_weight;
    tare_weight: root.tare_weight;
    net_weight: root.net_weight;
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test -p app home_state::tests::realtime_info_card_replaces_the_left_placeholder -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add app/src/home_state.rs app/dashboard/pages/home/components/realtime-info-card.slint app/dashboard/pages/home/components/page.slint app/dashboard/pages/home/page.slint
git commit -m "feat(home): add realtime info card"
```

### Task 5: Full Verification And Cleanup

**Files:**
- Modify: `app/src/home_state.rs`
- Modify: `app/dashboard/app.slint`
- Modify: `app/dashboard/pages/home/models.slint`
- Modify: `app/dashboard/pages/home/page.slint`
- Modify: `app/dashboard/pages/home/components/page.slint`
- Create: `app/dashboard/pages/home/components/realtime-field-row.slint`
- Create: `app/dashboard/pages/home/components/realtime-info-card.slint`

- [ ] **Step 1: Run the focused homepage tests**

Run: `cargo test -p app home_state -- --nocapture`

Expected: PASS with the homepage state and Slint contract tests green.

- [ ] **Step 2: Run compile verification**

Run: `cargo check -p app`

Expected: PASS

- [ ] **Step 3: Run lint verification**

Run: `cargo clippy -p app --all-targets --all-features`

Expected: PASS with no new warnings in `app`.

- [ ] **Step 4: Review the final diff**

Run: `git diff -- app/dashboard/app.slint app/dashboard/pages/home/models.slint app/dashboard/pages/home/page.slint app/dashboard/pages/home/components/page.slint app/dashboard/pages/home/components/realtime-field-row.slint app/dashboard/pages/home/components/realtime-info-card.slint app/src/home_state.rs`

Expected: Only the realtime-panel files and state wiring appear.

- [ ] **Step 5: Commit**

```bash
git add app/dashboard/app.slint app/dashboard/pages/home/models.slint app/dashboard/pages/home/page.slint app/dashboard/pages/home/components/page.slint app/dashboard/pages/home/components/realtime-field-row.slint app/dashboard/pages/home/components/realtime-info-card.slint app/src/home_state.rs
git commit -m "feat(home): implement realtime weighing panel"
```
