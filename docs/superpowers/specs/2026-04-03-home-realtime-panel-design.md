# 首页实时数据区域设计

**目标**

在首页左下角业务区实现“最新一条实时称重记录”展示卡片，先完成 Slint 前端结构和 Rust 状态接口定义，不实现下方历史表格、查询条件和编辑交互。

**范围**

- 替换首页左下角占位卡片。
- 展示最新一条实时记录。
- 左侧使用动态字段列表。
- 右侧固定展示 `毛重 / 皮重 / 净重`。
- 使用只读表单风格控件展示内容。

**不在本次范围**

- 历史称重记录表格。
- 查询筛选区域。
- 用户编辑和提交。
- 设备实际接入逻辑。
- 监控区域改造。

**页面结构**

- 首页整体布局不变，仍然使用当前 `App -> HomePage -> 顶部三卡片 + 底部左右两区` 骨架。
- 左下角区域替换为新的 `RealtimeInfoCard` 组件。
- `RealtimeInfoCard` 顶部保留标题区，标题定为“称重信息”。
- 卡片主体分为左右两栏：
  - 左栏：动态信息区，常见展示 4 到 6 行。
  - 右栏：固定重量区，展示 `毛重 / 皮重 / 净重` 三行。
- 本次不做内部滚动，布局按单屏可读优先。

**左侧动态信息区**

- 左侧字段由后端按顺序下发，前端不写死业务字段名。
- 每一行包含：
  - 左侧 `label`
  - 右侧只读 `UInput`
- 典型字段包括但不限于：
  - `车牌号`
  - `发卡号`
  - `农户`
  - `货物名称`
  - `收货单位`
  - `过磅单位`
  - `时间`
- “车牌号 / 发卡号 / 农户”由后端决定实际标签文本，前端只负责渲染。

**左侧字段显示规则**

- 每个字段都带 `always_show_when_empty`。
- 当 `value` 为空且 `always_show_when_empty == false` 时，该行不渲染。
- 当 `value` 为空且 `always_show_when_empty == true` 时，该行保留，输入框显示空值。
- 字段过滤逻辑固定放在 `RealtimeInfoCard` 组件内部处理，不在页面布局层或 Rust 映射层分散判断。

**右侧固定重量区**

- 固定展示三项：
  - `毛重`
  - `皮重`
  - `净重`
- 每一行也采用 `label + 只读 UInput` 形式，保持和左栏一致的表单视觉。
- 没有称量值时统一显示 `0`。
- 右栏字段顺序固定，不跟随左侧动态字段变化。
- 视觉上应比左栏更稳、更容易扫读，突出称重核心数据。

**组件拆分**

- `app/dashboard/pages/home/components/realtime-info-card.slint`
  - 实时数据区域主卡片。
- `app/dashboard/pages/home/components/realtime-field-row.slint`
  - 通用的 `label + 只读输入框` 行组件。
- `app/dashboard/pages/home/components/page.slint`
  - 导出新的实时数据组件。
- `app/dashboard/pages/home/page.slint`
  - 将左下角占位组件替换成 `RealtimeInfoCard`。
- `app/dashboard/pages/home/models.slint`
  - 新增实时区域使用的结构体。

**Slint 数据定义**

- 新增 `HomeRealtimeFieldItem`
  - `label: string`
  - `value: string`
  - `always_show_when_empty: bool`
- 重量区先不定义数组结构，直接使用独立属性：
  - `gross_weight: string`
  - `tare_weight: string`
  - `net_weight: string`

这样做的原因是左侧字段顺序和类型受后端驱动，适合列表；右侧重量字段固定且业务含义强，拆成独立属性更直观，也更利于后续扩展颜色、状态或单位显示。

**页面属性调整**

- `app/dashboard/app.slint` 新增：
  - `realtime_fields`
  - `gross_weight`
  - `tare_weight`
  - `net_weight`
- `app/dashboard/pages/home/page.slint` 新增对应入参并透传给 `RealtimeInfoCard`。

**Rust 状态定义**

- 在 `app/src/home_state.rs` 中新增：
  - `RealtimeFieldSnapshot`
    - `label: String`
    - `value: String`
    - `always_show_when_empty: bool`
- 将首页状态扩展为：
  - `realtime_fields: Vec<RealtimeFieldSnapshot>`
  - `gross_weight: String`
  - `tare_weight: String`
  - `net_weight: String`

**Rust 接口约定**

- 沿用当前 `HomePageAdapter` 模式。
- `load_home_state()`
  - 返回首页首次加载时的最新实时记录。
- `refresh_home_state()`
  - 返回刷新后的最新实时记录。
- 后续硬件接入、后端接口接入或串口数据解析，都通过适配器将外部 DTO 映射成首页状态。
- `apply_home_page_state()`
  - 负责将实时字段列表和固定重量字段映射到 Slint 属性。

**状态兜底规则**

- `gross_weight / tare_weight / net_weight` 为空时，Rust 层兜底设置为 `"0"`。
- 左侧动态字段不在 Rust 层强制补默认字段名，只保留后端实际提供的顺序和标签。
- 如果后端需要“空值但保留一行”，必须显式将 `always_show_when_empty` 置为 `true`。

**交互约束**

- 本次所有输入框均为只读。
- 不提供键盘输入、下拉选择、保存按钮、清空按钮。
- 页面刷新动作只负责更新显示内容，不引入局部编辑状态。

**测试与验证**

- Rust 单元测试：
  - 默认首页状态包含实时区域示例数据。
  - 自定义 `HomePageAdapter` 能正确下发动态字段和固定重量。
  - 重量空值兜底为 `0`。
- 编译验证：
  - `cargo check -p app`
  - `cargo test -p app`
- 视觉验证：
  - 首页左下角可显示 4 到 6 行动态字段。
  - 左侧空字段在不同 `always_show_when_empty` 配置下表现正确。
  - 右侧三项重量始终可见且顺序正确。

**实施顺序**

1. 在 `models.slint` 和 `home_state.rs` 中补齐实时区域数据结构。
2. 实现通用字段行组件。
3. 实现 `RealtimeInfoCard`。
4. 替换首页左下角占位区并接入新属性。
5. 更新默认首页样例数据与测试。
6. 运行编译与测试验证。
