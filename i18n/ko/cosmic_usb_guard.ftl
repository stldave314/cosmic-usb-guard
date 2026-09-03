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
refresh = 새로 고침
copy = 복사
open-app = USB 가드 열기
quit = 종료

## Navigation

page-devices = 장치
page-history = 기록
page-status = 상태
page-settings = 설정

## Device list

devices-heading = 연결된 장치
devices-none = USB 장치 없음
devices-none-description = 연결된 장치가 없거나, USBGuard가 이 시스템의 USB 버스를 인식하지 못합니다.
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
remember-decision = 이 결정 기억하기

## Warnings

warning-input-capable = 이 장치는 키보드처럼 동작하여 사용자를 대신해 입력할 수 있습니다.
warning-storage = 이 장치는 저장소를 제공합니다.
warning-network = 이 장치는 네트워크 어댑터를 제공하며, 트래픽 경로를 바꿀 수 있습니다.
warning-standing-conflict = 상시 규칙이 다음 연결 시 이를 “{ $target }”(으)로 되돌립니다.
warning-no-hash = USBGuard가 이 장치의 디스크립터 해시를 보고하지 않아, 영구 규칙을 이 장치에만 한정해 고정할 수 없습니다.

## Status and health

status-ok = USBGuard가 이 시스템을 보호하고 있습니다
status-warning = USBGuard가 문제를 안고 실행 중입니다
status-critical = 이 시스템은 보호되지 않습니다
status-disconnected = USBGuard에 연결되지 않음
status-disconnected-description = { $reason }
status-checking = 확인 중…

check-daemon-running = USBGuard 서비스가 실행 중
check-daemon-enabled = USBGuard 서비스가 부팅 시 시작됨
check-dbus-running = USBGuard D-Bus 인터페이스가 실행 중
check-dbus-enabled = USBGuard D-Bus 인터페이스가 부팅 시 시작됨
check-ipc-reachable = 데몬이 요청에 응답함
check-ipc-permission = 정책을 결정할 권한이 있음
check-decisions-reversible = 영구적인 결정을 되돌릴 권한이 있음
check-inserted-policy = 새 장치는 결정을 기다림
check-policy-not-empty = 장치 정책이 구성되어 있음

check-observed = 관측값: { $value }
remedy-heading = 다음을 실행하여 해결하세요:

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
event-service-up = USBGuard를 사용할 수 있게 됨
event-service-down = USBGuard를 사용할 수 없게 됨
event-health-problem = 구성 문제

actor-user = 사용자에 의해
actor-policy = USBGuard 정책에 의해
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
setting-show-hardwired-description = USBGuard가 고정 연결로 보고하는, 분리할 수 없는 장치를 포함합니다.
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
setting-warn-on-health-problems-description = USBGuard가 이 시스템을 보호하도록 설정되어 있지 않을 때 경고를 표시합니다.
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

error-service-unavailable = USBGuard가 실행 중이 아닙니다
error-permission-denied = 허용되지 않음
error-autostart = 자동 시작 설정을 변경할 수 없습니다: { $message }
error-no-tray = 이 데스크톱에는 시스템 트레이가 없어 상태 아이콘을 표시할 수 없었습니다. 대신 USB 가드 창을 열었습니다.
error-cannot-remove-rule = 상시 규칙을 삭제하려면 관리자 승인이 필요하지만 승인되지 않았습니다. 상태 페이지의 “영구적인 결정을 되돌릴 권한이 있음”을 참고하세요.

## About

repository = 저장소
support = 문제 신고
version = 버전 { $version }
