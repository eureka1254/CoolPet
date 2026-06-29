# CoolPet

基于 Tauri 2 + React + Rust 的 AI 桌宠系统。

## 项目简介

CoolPet 是一个桌面透明悬浮 AI 桌宠，支持自定义 PNG 序列帧角色、多模型 AI 对话、喂食与状态系统。所有数据本地存储，不上传 API Key 或聊天记录。

## 功能介绍

- 桌面透明悬浮桌宠，支持拖拽、抛出、落地、睡眠、唤醒和待机动作
- 人物管理：创建、切换、编辑、删除人物，每个人物独立 prompt、记忆、聊天 session 和序列帧资源
- PNG 序列帧资源导入：支持自定义角色资源包
- AI 聊天：流式输出、打字机效果、停止生成、动作标签触发表情动作
- 多 Provider 支持：OpenAI、Gemini、DeepSeek、Zhipu GLM、Kimi、Qwen、Ollama、Claude 等
- 喂食系统：独立食物类型、库存、能量和好感变化
- 状态系统：能量、好感、每日聊天轮数、每日/每周结算
- 主题系统：内置主题和自定义颜色主题
- 本地优先：数据库和资源保存在本机 AppData

## 本地运行

### 环境要求

- Node.js
- pnpm
- Rust stable (MSVC toolchain, Windows)
- Visual Studio Build Tools (Desktop development with C++)

### 安装依赖

```bash
pnpm install
```

### 启动开发模式

```bash
pnpm tauri dev
```

如果 1420 端口被占用，请先关闭已有进程。

### 类型检查 / Lint / 测试

```bash
pnpm run typecheck
pnpm run lint
pnpm run test
```

## 技术栈

- **Frontend**: React 19, TypeScript, Vite, Tailwind CSS, Zustand
- **Desktop**: Tauri 2
- **Backend**: Rust, SQLite (rusqlite bundled), reqwest (rustls-tls)
- **Package manager**: pnpm

## 打包

```bash
pnpm tauri build
```

产物位于 `src-tauri/target/release/bundle/`：

- `CoolPet_0.1.0_x64-setup.exe` — NSIS 安装程序
- `CoolPet_0.1.0_x64_en-US.msi` — MSI 安装包

## 数据目录

用户数据保存在：

```text
%APPDATA%/com.chen.coolpet/bytepet-data/
```

主要内容：

- `bytepet.db` — SQLite 数据库
- `skins/` — 用户导入的角色序列帧资源
- `food_icons/` — 食物图标
- `logs/`、`backups/`、`exports/` — 日志、备份和导出

## 自定义角色资源包

角色序列帧资源包需包含以下动作目录：

```text
calm/ sleeping/ wake_up/ yawn/ sit/ sit_down/
happy/ cheer_up/ sad/ angry/ comfort/ thinking/
eat_food/ run_left/ run_right/ fly_up/ fall_down/
dizzy/ error/ box/ action1/
```

导入后资源复制到 AppData 的 `bytepet-data/skins/` 下。

## License

MIT License. See [LICENSE](./LICENSE) for details.
