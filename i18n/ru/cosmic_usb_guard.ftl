# SPDX-License-Identifier: GPL-3.0-or-later
# Russian (Русский) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Просматривайте и контролируйте, какие USB-устройства могут подключаться

## Generic actions

allow = Разрешить
block = Заблокировать
reject = Отклонить
revoke = Отозвать
forget = Забыть правило
details = Подробности
dismiss = Пропустить
refresh = Обновить
copy = Копировать
open-app = Открыть USB Guard
quit = Выйти

## Navigation

page-devices = Устройства
page-history = История
page-status = Состояние
page-settings = Настройки

## Device list

devices-heading = Подключённые устройства
devices-none = Нет USB-устройств
devices-none-description = Ничего не подключено, либо USBGuard не видит шину USB этой системы.
devices-hidden = Настройками отображения { $count ->
        [one] скрыто { $count } устройство
        [few] скрыто { $count } устройства
       *[many] скрыто { $count } устройств
    }
devices-pending = { $count ->
        [one] { $count } устройство ожидает
        [few] { $count } устройства ожидают
       *[many] { $count } устройств ожидают
    } решения
devices-remembered = Не подключены
devices-remembered-description = Устройства с постоянным правилом, которые сейчас не подключены. Правило можно изменить или удалить здесь.
device-internal = Внутреннее устройство
device-internal-toggle = Внутреннее устройство
device-internal-description = Часть этого компьютера. Скрывается из списка, и о нём больше не спрашивают. Это не разрешает его — разрешите его отдельно, если оно должно работать.
device-internal-no-hash = Это устройство не сообщает хеш дескриптора, поэтому его нельзя отметить как внутреннее.

## Device state

state-allowed = Разрешено
state-blocked = Заблокировано
state-rejected = Отклонено
state-unknown = Неизвестно
state-pending = Ожидает решения
state-disconnected = Не подключено

## Standing policy rules

standing-allow = Всегда разрешено
standing-block = Всегда блокируется
standing-reject = Всегда отклоняется
standing-other = Постоянное правило: { $target }

## Device fields

field-name = Название
field-usb-id = Идентификатор USB
field-serial = Серийный номер
field-port = Порт
field-hash = Хеш дескриптора
field-interfaces = Интерфейсы
field-connection = Подключение
field-status = Состояние
field-none = Не сообщено

## Decision prompt

prompt-heading = Разрешить это устройство?
prompt-description = { $name } только что подключено. Оно останется заблокированным, пока вы не примете решение.
notify-new-device = Подключено новое USB-устройство
notify-auto-blocked = USB-устройство отклонено
notify-auto-blocked-body = { $name } отклонено постоянным правилом и недоступно. Откройте USB Guard, чтобы это изменить.
notify-manage = Управление устройством
remember-decision = Запомнить это решение

## Warnings

warning-input-capable = Это устройство может выдавать себя за клавиатуру и вводить текст от вашего имени.
warning-storage = Это устройство предоставляет накопитель.
warning-network = Это устройство предоставляет сетевой адаптер, способный перенаправить ваш трафик.
warning-standing-conflict = Постоянное правило вернёт значение «{ $target }» при следующем подключении.
warning-no-hash = USBGuard не сообщил хеш дескриптора для этого устройства, поэтому постоянное правило нельзя привязать именно к нему.

## Status and health

status-ok = USBGuard защищает эту систему
status-warning = USBGuard работает с проблемами
status-critical = Эта система не защищена
status-disconnected = Нет подключения к USBGuard
status-disconnected-description = { $reason }
status-checking = Проверка…

check-daemon-running = Служба USBGuard работает
check-daemon-enabled = Служба USBGuard запускается при загрузке
check-dbus-running = Интерфейс D-Bus USBGuard работает
check-dbus-enabled = Интерфейс D-Bus USBGuard запускается при загрузке
check-ipc-reachable = Служба отвечает на запросы
check-ipc-permission = Вы можете принимать решения по политике
check-decisions-reversible = Вы можете отменить постоянное решение
check-inserted-policy = Новые устройства ожидают решения
check-policy-not-empty = Политика устройств настроена

check-observed = Наблюдается: { $value }
remedy-heading = Исправьте, выполнив:

## History

history-heading = История решений
history-empty = Пока ничего не записано
history-empty-description = Здесь появятся события устройств и принятые по ним решения.
history-clear = Очистить историю
history-entries = { $count ->
        [one] { $count } запись
        [few] { $count } записи
       *[many] { $count } записей
    }
history-filter-all = Все события
history-filter-decisions = Только решения

event-inserted = Подключено
event-removed = Отключено
event-updated = Изменено
event-allowed = Разрешено
event-blocked = Заблокировано
event-rejected = Отклонено
event-revoked = Отозвано
event-service-up = USBGuard стал доступен
event-service-down = USBGuard стал недоступен
event-health-problem = Проблема конфигурации

actor-user = вами
actor-policy = политикой USBGuard
actor-external = вне этого приложения
actor-system = автоматически

## Hooks

hook-heading = Запускать программу при подключении
hook-description = Запускается только после того, как это устройство разрешено. Заблокированное устройство никогда ничего не запускает.
hook-program = Программа
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = Аргументы
hook-arguments-placeholder = По одному в строке
hook-label = Название
hook-label-placeholder = Резервная копия
hook-enabled = Включено
hook-save = Сохранить действие
hook-remove = Удалить действие
hook-none = Программа не задана
hook-problem-not-set = Выберите программу для запуска.
hook-problem-not-absolute = Укажите полный путь, начинающийся с косой черты.
hook-problem-missing = Такого файла не существует.
hook-problem-not-executable = Этот файл не является исполняемым. Выполните: chmod +x
hook-variables = Программа получает сведения об устройстве в переменных окружения: { $names }.

## Settings

setting-prompt-on-insert = Спрашивать о новых устройствах
setting-prompt-on-insert-description = Показывать запрос решения при подключении устройства без постоянного правила.
setting-notify-on-insert = Отправлять уведомление рабочего стола
setting-notify-on-insert-description = Также уведомлять, чтобы запрос не был пропущен при скрытой панели.
setting-auto-open-popup = Открывать окно автоматически
setting-auto-open-popup-description = Позволить USB Guard самому открывать окно, когда устройству нужно решение.
setting-default-permanent = Запоминать решения по умолчанию
setting-default-permanent-description = Заранее отмечать «Запомнить это решение» в запросе.
setting-show-hardwired = Показывать распаянные устройства
setting-show-hardwired-description = Включать устройства, которые USBGuard считает несъёмными и которые нельзя отключить.
setting-show-root-hubs = Показывать корневые концентраторы
setting-show-root-hubs-description = Включать хост-контроллеры, к которым подключены порты USB.
setting-show-internal = Показывать внутренние устройства
setting-show-internal-description = Включать устройства, отмеченные вами как часть этого компьютера.
setting-show-disconnected = Показывать отключённые устройства
setting-show-disconnected-description = Перечислять устройства с постоянным правилом, которые не подключены, чтобы решение можно было изменить без них.
setting-warn-input-capable = Выделять устройства, способные быть клавиатурой
setting-warn-input-capable-description = Отмечать устройства, способные внедрять нажатия клавиш.
setting-journal-enabled = Вести историю решений
setting-journal-enabled-description = Записывать события и решения в { $path }.
setting-warn-on-health-problems = Предупреждать о проблемах конфигурации
setting-warn-on-health-problems-description = Показывать предупреждение, когда USBGuard не настроен на защиту этой системы.
setting-notify-on-auto-block = Сообщать, когда устройство отклонено
setting-notify-on-auto-block-description = Уведомлять, когда постоянное правило отклоняет устройство без вопроса, чтобы оно не просто молча не работало.
setting-show-tray-icon = Показывать значок состояния
setting-show-tray-icon-description = Отображать щит в системном лотке. Отключение не мешает приложению следить за устройствами.
setting-autostart = Запускать автоматически при входе
setting-autostart-description = Добавить USB Guard в сеанс, чтобы он следил ещё до того, как вы что-то подключите.
setting-start-minimized = Запускаться без открытия окна
setting-start-minimized-description = При входе показывать только значок состояния.
setting-run-in-background = Продолжать работу при закрытии окна
setting-run-in-background-description = Закрытие окна оставляет USB Guard следить за устройствами. Отключите это — и закрытие завершит приложение, а вместе с ним и запросы.

section-behaviour = Поведение
section-display = Отображение
section-startup = Запуск и значок состояния
section-privacy = История

## Errors

error-service-unavailable = USBGuard не запущен
error-permission-denied = Не разрешено
error-autostart = Не удалось изменить настройку автозапуска: { $message }
error-no-tray = На этом рабочем столе нет системного лотка, поэтому значок состояния показать не удалось. Вместо этого USB Guard открыл своё окно.
error-cannot-remove-rule = Для удаления постоянного правила требуется авторизация администратора, которая не была предоставлена. См. «Вы можете отменить постоянное решение» на странице состояния.

## About

repository = Репозиторий
support = Сообщить о проблеме
version = Версия { $version }
