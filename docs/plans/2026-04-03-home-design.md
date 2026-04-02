# 称重首页设计文档

**目标**

为 Slint 前端新增称重软件首页骨架，先完成页面布局、组件拆分和 Rust 侧参数/函数定义，暂不接入后端业务逻辑。

**页面结构**

- 页面采用单屏布局，不出现滚动条。
- 主体分为两层：
  - 顶部状态区：固定高度，三张等宽卡片。
  - 底部业务区：占满剩余高度，左侧大卡片跨两列，右侧大卡片占一列。
- 顶部三张卡片分别为：
  - 地磅状态卡片：显示重量、单位和三项设备状态。
  - 运行日志卡片：显示彩色日志，可滚动但不显示滚动条。
  - 实时状态步骤卡片：显示当前地磅流程步骤。
- 底部两张大卡片暂时只保留占位区域：
  - 左侧：实时数据区域。
  - 右侧：监控区域。

**组件拆分**

- `app/dashboard/pages/home/page.slint`
  - 首页总布局，只负责组合各个子组件。
- `app/dashboard/pages/home/components/weight-status-card.slint`
  - 顶部左侧地磅状态卡片。
- `app/dashboard/pages/home/components/log-card.slint`
  - 顶部中部运行日志卡片。
- `app/dashboard/pages/home/components/process-step-card.slint`
  - 顶部右侧实时步骤卡片。
- `app/dashboard/pages/home/components/panel-placeholder-card.slint`
  - 底部占位卡片。
- `app/dashboard/pages/home/models.slint`
  - 首页公用枚举和结构体。

**数据定义**

- 地磅状态：
  - `weight_value`
  - `weight_unit`
  - `device_statuses`
- 运行日志：
  - `HomeLogItem`
  - `HomeLogLevel`
- 流程步骤：
  - `step_labels`
  - `current_step_index`

**Rust 脚手架**

- 新增 `app/src/home_state.rs`
  - 定义首页默认数据结构。
  - 提供首页初始化函数。
  - 提供后续联动所需的空回调绑定函数。
- `main.rs`
  - 引入首页状态模块。
  - 创建窗口后调用首页初始化函数。

**验证方式**

- `cargo test -p app`
- `cargo check -p app`

