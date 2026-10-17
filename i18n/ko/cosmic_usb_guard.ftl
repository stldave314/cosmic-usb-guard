# SPDX-License-Identifier: GPL-3.0-or-later
# Korean (한국어) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB 가드
app-description = 어떤 USB 장치가 연결될 수 있는지 검토하고 제어합니다

## Generic actions

allow = 허용
block = 차단
reject = 거부
revoke = 철회
forget = 규칙 삭제
details = 세부 정보
dismiss = 닫기
allow-once = 이번만 허용
always-allow = 항상 허용
refresh = 새로 고침
copy = 복사
copy-command = 명령 복사
open-app = USB 가드 열기
quit = 종료

## Navigation

page-devices = 장치
page-thunderbolt = Thunderbolt
page-history = 기록
page-status = 상태
page-settings = 설정

## Device list

devices-heading = 연결된 장치
devices-none = USB 장치 없음
devices-none-description = 연결된 것이 없거나, USBGuard 서비스가 이 시스템의 USB 버스를 볼 수 없습니다.
devices-hidden = 표시 설정으로 { $count }개 장치가 숨겨짐
devices-pending = { $count }개 장치가 결정을 기다리는 중
devices-remembered = 연결되지 않음
devices-remembered-description = 상시 규칙은 있지만 연결되어 있지 않은 장치입니다. 여기서 규칙을 변경하거나 삭제할 수 있습니다.
device-internal = 내장 장치
device-internal-toggle = 내장 장치
device-internal-description = 이 컴퓨터의 일부입니다. 목록에서 숨겨지고 다시 묻지 않습니다. 이것이 허용을 의미하지는 않습니다. 작동해야 한다면 별도로 허용하세요.
device-internal-no-hash = 이 장치는 디스크립터 해시를 보고하지 않으므로 내장 장치로 표시할 수 없습니다.

## Device state

state-allowed = 허용됨
state-blocked = 차단됨
state-rejected = 거부됨
state-unknown = 알 수 없음
state-pending = 결정 대기 중
state-disconnected = 연결되지 않음

## Standing policy rules

standing-allow = 항상 허용
standing-block = 항상 차단
standing-reject = 항상 거부
standing-other = 상시 규칙: { $target }
bound-port-toggle = USB 포트에 고정됨
bound-port-description = 장치가 포트 { $port }에 연결되어 있을 때만 규칙이 적용됩니다. 다른 곳에서는 차단되고 다시 묻습니다.
bound-port-description-unbound = 규칙이 장치가 지금 연결된 포트에서만 적용되도록 합니다. 이 장치의 신원을 흉내 내는 장치도 그 포트에 연결되어야 합니다.
bound-port-unplugged = 규칙을 연결된 포트에 고정하려면 장치를 연결하세요.

## Device fields

field-name = 이름
field-usb-id = USB ID
field-serial = 일련번호
field-port = 포트
field-hash = 디스크립터 해시
field-interfaces = 인터페이스
field-connection = 연결 방식
field-status = 상태
field-none = 보고되지 않음

## Decision prompt

prompt-heading = 이 장치를 허용하시겠습니까?
prompt-description = { $name }이(가) 방금 연결되었습니다. 결정하실 때까지 차단된 상태로 유지됩니다.
notify-new-device = 새 USB 장치가 연결됨
notify-auto-blocked = USB 장치가 거부됨
notify-auto-blocked-body = { $name }이(가) 상시 규칙에 의해 거부되어 사용할 수 없습니다. 변경하려면 USB 가드를 여세요.
notify-manage = 장치 관리
notify-thunderbolt = Thunderbolt 장치 대기 중
notify-thunderbolt-body = { $name }이(가) 연결되었지만 허용하기 전까지 컴퓨터 메모리에 접근할 수 없습니다.
remember-decision = 이 결정 기억하기
bind-to-port = 이 USB 포트에 고정 ({ $port })
bind-to-port-description = 이 포트에서만 적용됩니다. 다른 포트에 연결하면 장치가 차단되고 다시 묻습니다.
prompt-wrong-port = 규칙은 포트 { $bound }에 고정되어 있지만 지금은 포트 { $port }에 연결되어 있습니다.

## Warnings

warning-input-capable = 이 장치는 키보드처럼 동작하여 사용자를 대신해 입력할 수 있습니다.
warning-storage = 이 장치는 저장소를 제공합니다.
warning-network = 이 장치는 네트워크 어댑터를 제공하며, 트래픽 경로를 바꿀 수 있습니다.
warning-standing-conflict = 상시 규칙이 다음 연결 시 이를 “{ $target }”(으)로 되돌립니다.
warning-overriding-rule = 정책 앞쪽에 있는 더 넓은 규칙도 이 장치와 일치하며 먼저 검사되므로, 이 장치 자신의 규칙 대신 그 규칙이 결정할 수 있습니다: { $rule }
warning-no-hash = USBGuard 서비스가 이 장치의 디스크립터 해시를 보고하지 않아, 영구 규칙을 이 장치에만 고정할 수 없습니다.

## Status and health

status-ok = 이 시스템은 보호되고 있습니다
status-warning = 보호에 문제가 있습니다
status-critical = 이 시스템은 보호되지 않습니다
status-disconnected = USBGuard 서비스에 연결되지 않음
status-disconnected-description = { $reason }
status-checking = 확인 중…

check-daemon-running = USBGuard 서비스가 실행 중입니다
check-daemon-enabled = USBGuard 서비스가 부팅 시 시작됩니다
check-dbus-running = USBGuard D-Bus 인터페이스가 실행 중입니다
check-dbus-enabled = USBGuard D-Bus 인터페이스가 부팅 시 시작됩니다
check-ipc-reachable = 데몬이 요청에 응답함
check-ipc-permission = 정책을 결정할 권한이 있음
check-decisions-reversible = 영구적인 결정을 되돌릴 권한이 있음
check-inserted-policy = 새 장치는 결정을 기다림
check-policy-not-empty = 장치 정책이 구성되어 있음
check-thunderbolt-security = Thunderbolt 장치는 연결되기 전에 허용해야 합니다
check-thunderbolt-dma = 메모리가 Thunderbolt 장치로부터 보호됩니다
check-thunderbolt-service = Thunderbolt 서비스(bolt)가 실행 중입니다
check-thunderbolt-permission = Thunderbolt 장치를 허용할 수 있습니다
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt 및 USB4

check-observed = 관측값: { $value }
observed-unknown = 알 수 없음
observed-no-bus = 시스템 버스에 연결되지 않음
observed-responding = 응답 중
observed-granted = 허용됨
observed-password = 관리자 비밀번호 필요
observed-refused = 거부됨
observed-polkit-unavailable = polkit을 사용할 수 없음
observed-rules = 정책의 규칙 수: { $count }
observed-unreadable = 읽을 수 없음
observed-running = 실행 중
observed-not-installed = 설치되지 않음
observed-stopped = 설치되었지만 실행 중이 아님
observed-on = 켜짐
observed-off = 꺼짐
observed-not-reported = 보고되지 않음
remedy-heading = 다음을 실행하여 해결하세요:
remedy-explanation = 명령을 복사해 터미널에 붙여넣고 Enter 키를 누르세요. 비밀번호를 묻고, 성공하면 아무것도 출력하지 않습니다. 그런 다음 새로 고침을 눌러 적용되었는지 확인하세요.
remedy-policy-warning = 먼저 읽어 보세요: 지금 연결된 모든 장치가 허용됩니다. 실행하기 전에 알아볼 수 없는 장치는 모두 분리하세요.
steps-heading = 펌웨어 설정에서 해결하세요:
steps-firmware-only = 이 설정은 컴퓨터 펌웨어에 저장되어 있어 어떤 명령으로도 바꿀 수 없습니다.
steps-restart = 작업을 저장하고 컴퓨터를 다시 시작하세요.
steps-open-setup = 컴퓨터가 시작되는 동안 펌웨어 설정을 여는 키를 누르세요. 보통 화면에 표시되며, 흔한 키는 F1 또는 Enter(Lenovo), F2(Dell, ASUS, Acer 및 대부분), F10(HP), Del(데스크톱 보드)입니다.
steps-tb-find = Thunderbolt 또는 USB4 설정을 찾으세요. 보통 Security, Config 또는 Advanced 아래에 있습니다.
steps-tb-set = 보안 수준을 "User Authorization" 또는 "Secure Connect"로 설정하세요. "No Security"와 "Legacy"는 피하세요.
steps-dma-set = "Kernel DMA Protection" 또는 이것이 의존하는 옵션을 켜세요: Intel은 "VT-d", AMD는 "AMD-Vi" 또는 "IOMMU". 보통 Security, Virtualization 또는 Advanced 아래에 있습니다.
steps-save = 변경 사항을 저장하고 나가서 컴퓨터가 시작되도록 하세요. 그런 다음 이 페이지에서 새로 고침을 누르세요.
steps-session-only = polkit은 컴퓨터 앞에 있는 사람, 즉 활성 로컬 세션만 USBGuard를 사용하도록 허용합니다. SSH나 원격 데스크톱 세션, 사용자 전환으로 벗어난 세션은 거부됩니다.
steps-session-heading = 해결 방법:
steps-session-local = 컴퓨터 앞의 세션에서 USB Guard를 실행하세요.
steps-session-switch = 사용자를 전환했다면 이 세션으로 돌아오세요.
steps-session-refresh = 이 페이지에서 새로 고침을 누르세요.

## History

history-heading = 결정 기록
history-empty = 아직 기록이 없습니다
history-empty-description = 장치 이벤트와 그에 대한 결정이 여기에 표시됩니다.
history-clear = 기록 지우기
history-entries = { $count }개 항목
history-filter-all = 모든 이벤트
history-filter-decisions = 결정만

event-inserted = 연결됨
event-removed = 분리됨
event-updated = 변경됨
event-allowed = 허용됨
event-blocked = 차단됨
event-rejected = 거부됨
event-revoked = 철회됨
event-service-up = USBGuard 서비스를 사용할 수 있게 됨
event-service-down = USBGuard 서비스를 사용할 수 없게 됨
event-health-problem = 구성 문제

actor-user = 사용자에 의해
actor-policy = USBGuard 서비스 규칙에 의해
actor-external = 이 앱 외부에서
actor-system = 자동

## Hooks

hook-heading = 연결 시 프로그램 실행
hook-description = 이 장치가 허용된 뒤에만 실행됩니다. 차단된 장치는 아무것도 실행하지 않습니다.
hook-program = 프로그램
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = 인자
hook-arguments-placeholder = 한 줄에 하나씩
hook-label = 이름
hook-label-placeholder = 백업
hook-enabled = 사용함
hook-save = 동작 저장
hook-remove = 동작 삭제
hook-none = 지정된 프로그램 없음
hook-problem-not-set = 실행할 프로그램을 선택하세요.
hook-problem-not-absolute = 슬래시로 시작하는 전체 경로를 입력하세요.
hook-problem-missing = 해당 파일이 존재하지 않습니다.
hook-problem-not-executable = 해당 파일은 실행 가능하지 않습니다. chmod +x 를 실행하세요.
hook-variables = 프로그램은 장치 정보를 환경 변수로 전달받습니다: { $names }.

## Settings

setting-prompt-on-insert = 새 장치에 대해 묻기
setting-prompt-on-insert-description = 상시 규칙이 없는 장치가 연결되면 결정 창을 표시합니다.
setting-notify-on-insert = 데스크톱 알림 보내기
setting-notify-on-insert-description = 패널이 숨겨져 있어도 놓치지 않도록 알림도 함께 보냅니다.
setting-auto-open-popup = 창을 자동으로 열기
setting-auto-open-popup-description = 장치에 대한 결정이 필요할 때 USB 가드가 스스로 창을 엽니다.
setting-default-permanent = 기본적으로 결정 기억하기
setting-default-permanent-description = 결정 창에서 “이 결정 기억하기”를 미리 선택해 둡니다.
setting-show-hardwired = 납땜된 장치 표시
setting-show-hardwired-description = USBGuard 서비스가 내장으로 보고하는, 분리할 수 없는 장치도 포함합니다.
setting-show-root-hubs = 루트 허브 표시
setting-show-root-hubs-description = USB 포트가 연결된 호스트 컨트롤러를 포함합니다.
setting-show-internal = 내장 장치 표시
setting-show-internal-description = 이 컴퓨터의 일부로 표시해 둔 장치를 포함합니다.
setting-show-disconnected = 연결되지 않은 장치 표시
setting-show-disconnected-description = 상시 규칙이 있지만 연결되지 않은 장치를 함께 보여 주어, 장치 없이도 결정을 바꿀 수 있게 합니다.
setting-warn-input-capable = 키보드가 될 수 있는 장치 강조
setting-warn-input-capable-description = 키 입력을 주입할 수 있는 장치를 눈에 띄게 표시합니다.
setting-journal-enabled = 결정 기록 보관
setting-journal-enabled-description = 장치 이벤트와 결정을 { $path }에 기록합니다.
setting-warn-on-health-problems = 구성 문제 경고
setting-warn-on-health-problems-description = 이 시스템이 완전히 보호되지 않을 때 알림을 표시합니다.
setting-notify-on-auto-block = 장치가 거부되면 알리기
setting-notify-on-auto-block-description = 상시 규칙이 묻지 않고 장치를 거부할 때 알립니다. 아무 설명 없이 작동하지 않는 상황을 막기 위함입니다.
setting-show-tray-icon = 상태 아이콘 표시
setting-show-tray-icon-description = 시스템 트레이에 방패 아이콘을 표시합니다. 끄더라도 장치 감시는 계속됩니다.
setting-autostart = 로그인 시 자동 시작
setting-autostart-description = USB 가드를 세션에 추가하여, 무언가를 연결하기 전부터 감시하도록 합니다.
setting-start-minimized = 창을 열지 않고 시작
setting-start-minimized-description = 로그인 시 상태 아이콘만 표시합니다.
setting-run-in-background = 창을 닫아도 계속 실행
setting-run-in-background-description = 창을 닫아도 USB 가드는 계속 감시합니다. 이 설정을 끄면 창을 닫을 때 종료되며, 확인 창도 멈춥니다.

section-behaviour = 동작
section-display = 표시
section-startup = 시작 및 상태 아이콘
section-privacy = 기록

## Errors

error-service-unavailable = USBGuard 서비스가 실행 중이 아닙니다
error-permission-denied = 허용되지 않음
error-autostart = 자동 시작 설정을 변경할 수 없습니다: { $message }
error-no-tray = 이 데스크톱에는 시스템 트레이가 없어 상태 아이콘을 표시할 수 없었습니다. 대신 USB 가드 창을 열었습니다.
error-cannot-remove-rule = 상시 규칙을 삭제하려면 관리자 승인이 필요하지만 승인되지 않았습니다. 상태 페이지의 “영구적인 결정을 되돌릴 권한이 있음”을 참고하세요.
error-thunderbolt-permission = Thunderbolt 장치를 승인할 권한이 없습니다. 상태 페이지의 "Thunderbolt 장치를 허용할 수 있습니다" 아래에 해결 방법이 있습니다.
error-bolt-unavailable = Thunderbolt 서비스(bolt)가 실행 중이 아닙니다. 상태 페이지에서 시작하는 방법을 볼 수 있습니다.
error-thunderbolt = Thunderbolt 서비스가 오류를 보고했습니다: { $message }

## About

repository = 저장소
support = 문제 신고
version = 버전 { $version }

## Thunderbolt page

tb-heading = Thunderbolt 및 USB4
tb-security = 보안 수준
tb-level-none = 없음: 모든 장치가 묻지 않고 연결됩니다
tb-level-user = 사용자 승인: 새 장치는 모두 허용해야 합니다
tb-level-secure = 보안 연결: 새 장치는 모두 허용해야 하며, 기억된 장치는 키로 신원을 증명합니다
tb-level-dponly = DisplayPort 전용: 어떤 장치도 PCIe 접근 권한을 얻지 못합니다
tb-level-usbonly = USB 전용: 어떤 장치도 PCIe 접근 권한을 얻지 못합니다
tb-level-nopcie = PCIe 비활성화: 어떤 장치도 PCIe 접근 권한을 얻지 못합니다
tb-dma = DMA 보호
tb-dma-on = 켜짐: 장치는 할당되지 않은 메모리를 읽을 수 없습니다
tb-dma-off = 꺼짐: 허용된 장치가 모든 메모리를 읽을 수 있습니다
tb-auto-off = bolt에서 자동 승인이 꺼져 있어 기억된 장치도 매번 허용을 기다립니다.
tb-never-asks = 이 보안 수준에서는 허용할 것이 없습니다. 장치에 대해 묻지 않습니다.
tb-has-problems = Thunderbolt 설정에 문제가 있습니다.
tb-open-status = 해결 방법 보기
tb-prompt-heading = 이 Thunderbolt 장치를 허용할까요?
tb-prompt-description = { $name }이(가) 연결되었습니다. 허용하기 전까지 컴퓨터 메모리나 PCIe 버스에 접근할 수 없습니다.
tb-devices = 장치
tb-no-devices = 연결된 Thunderbolt 또는 USB4 장치가 없습니다.
tb-remembered = 기억됨
tb-not-remembered = 기억 안 됨
tb-status-waiting = 허용 대기 중
tb-status-authorized = 허용됨
tb-status-error = 승인 실패
tb-status-working = 연결 중…
tb-forget = 잊기
tb-forget-note = 연결된 Thunderbolt 장치는 분리될 때까지 접근 권한을 유지하며, 그 전에 회수할 방법은 없습니다. 장치 잊기는 다음 연결 시 적용됩니다.
