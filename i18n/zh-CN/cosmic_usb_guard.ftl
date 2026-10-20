# SPDX-License-Identifier: GPL-3.0-or-later
# Chinese, Simplified (简体中文) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB 卫士
app-description = 查看并控制哪些 USB 设备可以连接

## Generic actions

allow = 允许
block = 阻止
reject = 拒绝
revoke = 撤销
forget = 忘记规则
details = 详细信息
dismiss = 忽略
allow-once = 仅允许一次
always-allow = 始终允许
refresh = 刷新
copy = 复制
copy-command = 复制命令
open-app = 打开 USB 卫士
quit = 退出

## Navigation

page-devices = 设备
page-thunderbolt = 雷电
page-history = 历史
page-status = 状态
page-settings = 设置

## Device list

devices-heading = 已连接的设备
devices-none = 没有 USB 设备
devices-none-description = 没有连接任何设备，或者 USBGuard 服务无法看到此系统的 USB 总线。
devices-hidden = 显示设置隐藏了 { $count } 个设备
devices-pending = 有 { $count } 个设备正在等待处理
devices-remembered = 未连接
devices-remembered-description = 有常驻规则但未插入的设备。可在此更改或删除规则。
device-internal = 内置设备
device-internal-toggle = 内置设备
device-internal-description = 属于本机的一部分。将从列表中隐藏且不再询问。这并不会授权它——如果需要它正常工作，请同时允许它。
device-internal-no-hash = 此设备未报告描述符哈希，因此无法标记为内置设备。

## Device state

state-allowed = 已允许
state-blocked = 已阻止
state-rejected = 已拒绝
state-unknown = 未知
state-pending = 等待处理
state-disconnected = 未连接

## Standing policy rules

standing-allow = 始终允许
standing-block = 始终阻止
standing-reject = 始终拒绝
standing-other = 常驻规则：{ $target }
bound-port-toggle = 绑定到 USB 端口
bound-port-description = 仅当设备插在端口 { $port } 时规则才生效。插在其他地方时会被阻止，并再次询问您。
bound-port-description-unbound = 让规则仅在设备当前所插的端口生效。冒充此设备身份的设备也必须插在该端口。
bound-port-unplugged = 插入设备后才能将其规则绑定到所在端口。

## Device fields

field-name = 名称
field-usb-id = USB ID
field-serial = 序列号
field-port = 端口
field-hash = 描述符哈希
field-interfaces = 接口
field-connection = 连接方式
field-status = 状态
field-none = 未报告

## Decision prompt

prompt-heading = 允许此设备吗？
prompt-description = { $name } 刚刚接入。在您做出决定之前它将保持阻止状态。
notify-new-device = 接入了新的 USB 设备
notify-auto-blocked = USB 设备被拒绝
notify-auto-blocked-body = { $name } 被一条常驻规则拒绝，当前不可用。打开 USB 卫士即可更改。
notify-manage = 管理设备
notify-thunderbolt = 雷电设备正在等待
notify-thunderbolt-body = { $name } 已连接，但在您允许之前无法访问计算机内存。
remember-decision = 记住此决定
bind-to-port = 绑定到此 USB 端口（{ $port }）
bind-to-port-description = 仅在此端口生效。插入其他任何端口时，设备会被阻止，并再次询问您。
prompt-wrong-port = 它的规则绑定到端口 { $bound }，但现在插在端口 { $port }。

## Warnings

warning-input-capable = 此设备可以充当键盘并代替您输入。
warning-storage = 此设备提供存储功能。
warning-network = 此设备提供网络适配器，可能会改道您的网络流量。
warning-standing-conflict = 下次连接时，一条常驻规则会将其改回“{ $target }”。
warning-overriding-rule = 策略中更靠前的一条更宽泛的规则也匹配此设备，并且会先被检查，因此它可能代替此设备自己的规则作出决定：{ $rule }
warning-no-hash = USBGuard 服务未报告此设备的描述符哈希，因此无法将永久规则专门绑定到它。

## Status and health

status-ok = 此系统受到保护
status-warning = 保护存在问题
status-critical = 此系统未受保护
status-disconnected = 未连接到 USBGuard 服务
status-disconnected-description = { $reason }
status-checking = 正在检查…

check-daemon-running = USBGuard 服务正在运行
check-daemon-enabled = USBGuard 服务开机启动
check-dbus-running = USBGuard D-Bus 接口正在运行
check-dbus-enabled = USBGuard D-Bus 接口开机启动
check-ipc-reachable = 守护进程能够响应请求
check-ipc-permission = 您可以做出策略决定
check-decisions-reversible = 您可以撤销永久决定
check-inserted-policy = 新设备会等待处理
check-policy-not-empty = 已配置设备策略
check-thunderbolt-security = 雷电设备在连接前必须获得允许
check-thunderbolt-dma = 内存受到保护，不受雷电设备访问
check-thunderbolt-service = 雷电服务（bolt）正在运行
check-thunderbolt-permission = 您可以允许雷电设备
status-group-usbguard = USB
status-group-thunderbolt = 雷电和 USB4

check-observed = 实测：{ $value }
observed-unknown = 未知
observed-no-bus = 未连接到系统总线
observed-responding = 有响应
observed-granted = 已授予
observed-password = 需要管理员密码
observed-asks-password = 会要求输入密码，这没有问题
observed-refused = 被拒绝
observed-polkit-unavailable = polkit 不可用
observed-rules = 策略中的规则数：{ $count }
observed-unreadable = 无法读取
observed-running = 正在运行
observed-not-installed = 未安装
observed-stopped = 已安装但未运行
observed-on = 开启
observed-off = 关闭
observed-not-reported = 未报告
remedy-heading = 运行以下命令修复：
remedy-explanation = 复制该命令，粘贴到终端中并按回车。它会要求输入密码，成功时不会输出任何内容。之后点击“刷新”确认是否生效。
remedy-policy-warning = 请先阅读：这会允许当前插入的所有设备。运行前请拔下所有您不认识的设备。
steps-heading = 在固件设置中修复：
steps-firmware-only = 此设置保存在计算机固件中，因此任何命令都无法更改。
steps-restart = 保存您的工作并重新启动计算机。
steps-open-setup = 开机时按下进入固件设置的按键。该按键通常会显示在屏幕上；常见的有 F1 或 Enter（联想）、F2（戴尔、华硕、宏碁及大多数其他品牌）、F10（惠普）和 Del（台式机主板）。
steps-tb-find = 找到雷电或 USB4 设置，通常位于 Security、Config 或 Advanced 下。
steps-tb-set = 将安全级别设为“User Authorization”或“Secure Connect”。避免使用“No Security”和“Legacy”。
steps-dma-set = 开启“Kernel DMA Protection”，或它所依赖的选项：Intel 上为“VT-d”，AMD 上为“AMD-Vi”或“IOMMU”。通常位于 Security、Virtualization 或 Advanced 下。
steps-save = 保存更改、退出并让计算机启动。然后在本页点击“刷新”。
steps-session-only = polkit 只允许坐在电脑前的人，也就是活动的本地会话，使用 USBGuard。通过 SSH 或远程桌面登录的会话，以及已切换离开的会话都会被拒绝。
steps-session-heading = 修复方法：
steps-session-local = 请在电脑前的会话中运行 USB Guard。
steps-session-switch = 如果您切换过用户，请切换回此会话。
steps-session-refresh = 在本页点击“刷新”。

## History

history-heading = 决定历史
history-empty = 尚无记录
history-empty-description = 设备事件以及对它们所做的决定将显示在此处。
history-clear = 清除历史
history-entries = { $count } 条记录
history-filter-all = 全部事件
history-filter-decisions = 仅决定

event-inserted = 已连接
event-removed = 已断开
event-updated = 已更改
event-allowed = 已允许
event-blocked = 已阻止
event-rejected = 已拒绝
event-revoked = 已撤销
event-service-up = USBGuard 服务已可用
event-service-down = USBGuard 服务已不可用
event-health-problem = 配置问题

actor-user = 由您
actor-policy = 由 USBGuard 服务的规则
actor-external = 在本应用之外
actor-system = 自动

## Hooks

hook-heading = 连接时运行程序
hook-description = 仅在此设备被允许之后才会运行。被阻止的设备绝不会运行任何程序。
hook-program = 程序
hook-program-placeholder = /home/你/bin/backup.sh
hook-arguments = 参数
hook-arguments-placeholder = 每行一个
hook-label = 名称
hook-label-placeholder = 备份
hook-enabled = 已启用
hook-save = 保存动作
hook-remove = 删除动作
hook-none = 未设置程序
hook-problem-not-set = 请选择要运行的程序。
hook-problem-not-absolute = 请输入以斜杠开头的完整路径。
hook-problem-missing = 该文件不存在。
hook-problem-not-executable = 该文件不可执行。请运行：chmod +x
hook-variables = 程序会通过环境变量收到设备信息：{ $names }。

## Settings

setting-prompt-on-insert = 询问新设备
setting-prompt-on-insert-description = 当接入没有常驻规则的设备时显示决定提示。
setting-notify-on-insert = 发送桌面通知
setting-notify-on-insert-description = 同时发送通知，以免面板隐藏时错过提示。
setting-auto-open-popup = 自动打开窗口
setting-auto-open-popup-description = 当设备需要做出决定时，让 USB 卫士自行打开窗口。
setting-default-permanent = 默认记住决定
setting-default-permanent-description = 在提示中预先勾选“记住此决定”。
setting-show-hardwired = 显示焊接式设备
setting-show-hardwired-description = 包括 USBGuard 服务报告为内置、无法拔出的设备。
setting-show-root-hubs = 显示根集线器
setting-show-root-hubs-description = 包含 USB 端口所挂接的主机控制器。
setting-show-internal = 显示内置设备
setting-show-internal-description = 包含您标记为本机组成部分的设备。
setting-show-disconnected = 显示未连接的设备
setting-show-disconnected-description = 列出有常驻规则但未插入的设备，这样无需插入即可更改决定。
setting-warn-input-capable = 突出显示可充当键盘的设备
setting-warn-input-capable-description = 标出可能注入按键的设备。
setting-journal-enabled = 保留决定历史
setting-journal-enabled-description = 将设备事件和决定记录到 { $path }。
setting-warn-on-health-problems = 对配置问题发出警告
setting-warn-on-health-problems-description = 当此系统未受到完全保护时显示提醒。
setting-notify-on-auto-block = 设备被拒绝时通知我
setting-notify-on-auto-block-description = 当常驻规则未经询问就拒绝设备时发出通知，以免它只是悄无声息地无法工作。
setting-show-tray-icon = 显示状态图标
setting-show-tray-icon-description = 在系统托盘中显示盾牌图标。关闭它并不会让本应用停止监视设备。
setting-autostart = 登录时自动启动
setting-autostart-description = 将 USB 卫士加入会话，让它在您插入任何设备之前就开始监视。
setting-start-minimized = 启动时不打开窗口
setting-start-minimized-description = 登录时仅显示状态图标。
setting-run-in-background = 关闭窗口后继续运行
setting-run-in-background-description = 关闭窗口后 USB 卫士仍在监视。关闭此选项后，关窗即退出，提示也会随之停止。

section-behaviour = 行为
section-display = 显示
section-startup = 启动与状态图标
section-privacy = 历史

## Errors

error-service-unavailable = USBGuard 服务未运行
error-permission-denied = 不允许
error-autostart = 无法更改自动启动设置：{ $message }
error-no-tray = 此桌面没有系统托盘，因此无法显示状态图标。USB 卫士改为打开了窗口。
error-cannot-remove-rule = 删除常驻规则需要管理员授权，但未获得授权。请参阅状态页面中的“您可以撤销永久决定”。
error-thunderbolt-permission = 您无法在此会话中授权雷电设备。状态页面的“您可以允许雷电设备”下说明了原因。
error-bolt-unavailable = 雷电服务（bolt）未运行。状态页面会说明如何启动它。
error-thunderbolt = 雷电服务报告了错误：{ $message }

## About

repository = 代码仓库
support = 报告问题
version = 版本 { $version }

## Thunderbolt page

tb-heading = 雷电和 USB4
tb-security = 安全级别
tb-level-none = 无：每个设备都会在不询问的情况下连接
tb-level-user = 用户授权：每个新设备都必须获得允许
tb-level-secure = 安全连接：每个新设备都必须获得允许，已记住的设备用密钥证明身份
tb-level-dponly = 仅 DisplayPort：任何设备都无法获得 PCIe 访问权限
tb-level-usbonly = 仅 USB：任何设备都无法获得 PCIe 访问权限
tb-level-nopcie = 已禁用 PCIe：任何设备都无法获得 PCIe 访问权限
tb-dma = DMA 保护
tb-dma-on = 开启：设备无法读取未分配给它的内存
tb-dma-off = 关闭：获准的设备可以读取任何内存
tb-auto-off = bolt 中的自动授权已关闭，因此已记住的设备每次也要等待允许。
tb-never-asks = 在此安全级别下无需允许任何设备：系统从不询问。
tb-has-problems = 雷电设置存在问题。
tb-open-status = 显示修复方法
tb-prompt-heading = 允许此雷电设备吗？
tb-prompt-description = { $name } 已连接。在您允许之前，它无法访问计算机内存或 PCIe 总线。
tb-devices = 设备
tb-no-devices = 尚未连接任何雷电或 USB4 设备。
tb-remembered = 已记住
tb-not-remembered = 未记住
tb-status-waiting = 等待允许
tb-status-authorized = 已允许
tb-status-error = 授权失败
tb-status-working = 正在连接…
tb-forget = 忘记
tb-forget-note = 已连接的雷电设备在拔出之前一直保留访问权限，无法提前收回。忘记设备会在其下次连接时生效。
