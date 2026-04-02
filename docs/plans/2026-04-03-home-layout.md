# Home Layout Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为称重软件首页实现新的 Slint 页面骨架，并补上 Rust 侧首页参数和函数定义。

**Architecture:** 页面采用 `App -> HomePage -> 顶部三卡片 + 底部两大卡片` 的组件化结构。Rust 侧只负责初始化默认首页数据和绑定空回调，不承载业务逻辑。

**Tech Stack:** Rust 2021, Slint 1.15, sleek-ui

---

### Task 1: 补齐首页设计资源

**Files:**
- Create: `app/dashboard/pages/home/models.slint`
- Create: `app/dashboard/pages/home/components/page.slint`

**Step 1: 定义首页结构体和枚举**

- 添加设备状态结构体、日志结构体、日志级别枚举。

**Step 2: 建立组件导出入口**

- 在 `components/page.slint` 中统一导出首页子组件。

### Task 2: 实现首页 UI 组件

**Files:**
- Create: `app/dashboard/pages/home/components/weight-status-card.slint`
- Create: `app/dashboard/pages/home/components/log-card.slint`
- Create: `app/dashboard/pages/home/components/process-step-card.slint`
- Create: `app/dashboard/pages/home/components/panel-placeholder-card.slint`
- Modify: `app/dashboard/pages/home/page.slint`

**Step 1: 实现顶部三张卡片**

- 地磅状态卡片
- 日志卡片
- 流程步骤卡片

**Step 2: 实现底部两张占位卡片**

- 左侧实时数据占位
- 右侧监控占位

**Step 3: 组合首页布局**

- 第一行三列等宽
- 第二行左右 `2:1`
- 全页面单屏显示

### Task 3: 补齐应用入口

**Files:**
- Modify: `app/dashboard/app.slint`

**Step 1: 切换到新的首页入口**

- 移除旧 demo 页面绑定
- 改为新的首页属性和回调

### Task 4: 先写 Rust 测试，再补首页状态脚手架

**Files:**
- Create: `app/src/home_state.rs`
- Modify: `app/src/main.rs`

**Step 1: 写首页默认数据测试**

- 断言默认重量、单位、步骤和日志数量。

**Step 2: 运行测试确认失败**

Run: `cargo test -p app home_state -- --nocapture`

**Step 3: 实现最小首页状态和空回调**

- `HomeBootstrapState`
- `build_home_bootstrap_state`
- `initialize_home_page`
- `bind_home_callbacks`

**Step 4: 运行测试确认通过**

Run: `cargo test -p app home_state -- --nocapture`

### Task 5: 编译验证

**Files:**
- Modify: `app/dashboard/app.slint`
- Modify: `app/dashboard/pages/home/**`
- Modify: `app/src/**`

**Step 1: 运行前端编译检查**

Run: `cargo check -p app`

**Step 2: 修正编译问题并复验**

Run: `cargo test -p app`
