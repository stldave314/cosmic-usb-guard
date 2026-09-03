# SPDX-License-Identifier: GPL-3.0-or-later
# Japanese (日本語) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB ガード
app-description = どの USB デバイスの接続を許可するかを確認・制御します

## Generic actions

allow = 許可
block = ブロック
reject = 拒否
revoke = 取り消す
forget = ルールを削除
details = 詳細
dismiss = 閉じる
refresh = 更新
copy = コピー
open-app = USB ガードを開く
quit = 終了

## Navigation

page-devices = デバイス
page-history = 履歴
page-status = 状態
page-settings = 設定

## Device list

devices-heading = 接続中のデバイス
devices-none = USB デバイスがありません
devices-none-description = 何も接続されていないか、USBGuard がこのシステムの USB バスを認識できていません。
devices-hidden = 表示設定により { $count } 台のデバイスを非表示中
devices-pending = { $count } 台のデバイスが判断を待っています
devices-remembered = 未接続
devices-remembered-description = 常設ルールはあるが接続されていないデバイスです。ここでルールを変更または削除できます。
device-internal = 内蔵デバイス
device-internal-toggle = 内蔵デバイス
device-internal-description = このマシンの一部です。一覧から隠され、確認も行われません。これは許可を意味しません。動作させたい場合は別途許可してください。
device-internal-no-hash = このデバイスはディスクリプターハッシュを報告しないため、内蔵デバイスとして登録できません。

## Device state

state-allowed = 許可済み
state-blocked = ブロック済み
state-rejected = 拒否済み
state-unknown = 不明
state-pending = 判断待ち
state-disconnected = 未接続

## Standing policy rules

standing-allow = 常に許可
standing-block = 常にブロック
standing-reject = 常に拒否
standing-other = 常設ルール: { $target }

## Device fields

field-name = 名前
field-usb-id = USB ID
field-serial = シリアル番号
field-port = ポート
field-hash = ディスクリプターハッシュ
field-interfaces = インターフェース
field-connection = 接続方法
field-status = 状態
field-none = 報告なし

## Decision prompt

prompt-heading = このデバイスを許可しますか？
prompt-description = { $name } が接続されました。判断するまでブロックされたままです。
notify-new-device = 新しい USB デバイスが接続されました
notify-auto-blocked = USB デバイスを拒否しました
notify-auto-blocked-body = { $name } は常設ルールにより拒否され、利用できません。変更するには USB ガードを開いてください。
notify-manage = デバイスを管理
remember-decision = この判断を記憶する

## Warnings

warning-input-capable = このデバイスはキーボードとして振る舞い、あなたの代わりに入力できます。
warning-storage = このデバイスはストレージを公開します。
warning-network = このデバイスはネットワークアダプターを提供し、通信の経路を変更できます。
warning-standing-conflict = 常設ルールにより、次回接続時に「{ $target }」へ戻されます。
warning-no-hash = USBGuard はこのデバイスのディスクリプターハッシュを報告しなかったため、恒久ルールをこの個体に限定して結び付けることはできません。

## Status and health

status-ok = USBGuard がこのシステムを保護しています
status-warning = USBGuard は動作していますが問題があります
status-critical = このシステムは保護されていません
status-disconnected = USBGuard に接続していません
status-disconnected-description = { $reason }
status-checking = 確認中…

check-daemon-running = USBGuard サービスが動作中
check-daemon-enabled = USBGuard サービスが起動時に開始
check-dbus-running = USBGuard の D-Bus インターフェースが動作中
check-dbus-enabled = USBGuard の D-Bus インターフェースが起動時に開始
check-ipc-reachable = デーモンが要求に応答している
check-ipc-permission = ポリシーを変更する権限がある
check-decisions-reversible = 恒久的な判断を取り消す権限がある
check-inserted-policy = 新しいデバイスは判断を待つ
check-policy-not-empty = デバイスポリシーが設定されている

check-observed = 実測値: { $value }
remedy-heading = 次のコマンドで修正できます:

## History

history-heading = 判断の履歴
history-empty = まだ記録がありません
history-empty-description = デバイスのイベントと、それに対して下した判断がここに表示されます。
history-clear = 履歴を消去
history-entries = { $count } 件
history-filter-all = すべてのイベント
history-filter-decisions = 判断のみ

event-inserted = 接続
event-removed = 取り外し
event-updated = 変更
event-allowed = 許可
event-blocked = ブロック
event-rejected = 拒否
event-revoked = 取り消し
event-service-up = USBGuard が利用可能になりました
event-service-down = USBGuard が利用できなくなりました
event-health-problem = 設定の問題

actor-user = あなたによる
actor-policy = USBGuard ポリシーによる
actor-external = このアプリ以外から
actor-system = 自動

## Hooks

hook-heading = 接続時にプログラムを実行
hook-description = このデバイスが許可された後にのみ実行されます。ブロックされたデバイスは何も実行しません。
hook-program = プログラム
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = 引数
hook-arguments-placeholder = 1 行に 1 つ
hook-label = 名前
hook-label-placeholder = バックアップ
hook-enabled = 有効
hook-save = アクションを保存
hook-remove = アクションを削除
hook-none = プログラムが未設定
hook-problem-not-set = 実行するプログラムを選んでください。
hook-problem-not-absolute = スラッシュで始まる絶対パスを入力してください。
hook-problem-missing = そのファイルは存在しません。
hook-problem-not-executable = そのファイルは実行可能ではありません。chmod +x を実行してください。
hook-variables = プログラムにはデバイス情報が環境変数として渡されます: { $names }。

## Settings

setting-prompt-on-insert = 新しいデバイスについて確認する
setting-prompt-on-insert-description = 常設ルールのないデバイスが接続されたときに確認画面を表示します。
setting-notify-on-insert = デスクトップ通知を送る
setting-notify-on-insert-description = パネルが隠れていても確認を見逃さないよう、通知も行います。
setting-auto-open-popup = ウィンドウを自動的に開く
setting-auto-open-popup-description = デバイスの判断が必要なとき、USB ガードが自らウィンドウを開きます。
setting-default-permanent = 判断を既定で記憶する
setting-default-permanent-description = 確認画面で「この判断を記憶する」をあらかじめオンにします。
setting-show-hardwired = はんだ付けされたデバイスを表示
setting-show-hardwired-description = USBGuard が固定接続と報告する、取り外せないデバイスも含めます。
setting-show-root-hubs = ルートハブを表示
setting-show-root-hubs-description = USB ポートがぶら下がっているホストコントローラーも含めます。
setting-show-internal = 内蔵デバイスを表示
setting-show-internal-description = このマシンの一部として登録したデバイスも含めます。
setting-show-disconnected = 未接続のデバイスを表示
setting-show-disconnected-description = 常設ルールはあるが接続されていないデバイスを一覧に含め、接続しなくても判断を変更できるようにします。
setting-warn-input-capable = キーボードになり得るデバイスを強調
setting-warn-input-capable-description = キー入力を注入できるデバイスを目立たせます。
setting-journal-enabled = 判断の履歴を残す
setting-journal-enabled-description = デバイスのイベントと判断を { $path } に記録します。
setting-warn-on-health-problems = 設定の問題を警告する
setting-warn-on-health-problems-description = USBGuard がこのシステムを保護する設定になっていないときに警告を表示します。
setting-notify-on-auto-block = デバイスが拒否されたら知らせる
setting-notify-on-auto-block-description = 常設ルールが確認なしにデバイスを拒否したときに通知します。黙って動かないままにしないためです。
setting-show-tray-icon = 状態アイコンを表示
setting-show-tray-icon-description = システムトレイに盾のアイコンを表示します。オフにしてもデバイスの監視は止まりません。
setting-autostart = ログイン時に自動起動
setting-autostart-description = USB ガードをセッションに追加し、何かを接続する前から監視させます。
setting-start-minimized = ウィンドウを開かずに起動
setting-start-minimized-description = ログイン時は状態アイコンのみを表示します。
setting-run-in-background = ウィンドウを閉じても動作を続ける
setting-run-in-background-description = ウィンドウを閉じても USB ガードは監視を続けます。オフにすると閉じたときに終了し、確認も止まります。

section-behaviour = 動作
section-display = 表示
section-startup = 起動と状態アイコン
section-privacy = 履歴

## Errors

error-service-unavailable = USBGuard が動作していません
error-permission-denied = 許可されていません
error-autostart = 自動起動の設定を変更できませんでした: { $message }
error-no-tray = このデスクトップにはシステムトレイがないため、状態アイコンを表示できませんでした。代わりに USB ガードのウィンドウを開きました。
error-cannot-remove-rule = 常設ルールの削除には管理者の承認が必要ですが、承認されませんでした。状態ページの「恒久的な判断を取り消す権限がある」をご覧ください。

## About

repository = リポジトリ
support = 問題を報告
version = バージョン { $version }
