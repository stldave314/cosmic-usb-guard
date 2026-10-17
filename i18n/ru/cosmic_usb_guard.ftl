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
allow-once = Разрешить один раз
always-allow = Всегда разрешать
refresh = Обновить
copy = Копировать
copy-command = Копировать команду
open-app = Открыть USB Guard
quit = Выйти

## Navigation

page-devices = Устройства
page-thunderbolt = Thunderbolt
page-history = История
page-status = Состояние
page-settings = Настройки

## Device list

devices-heading = Подключённые устройства
devices-none = Нет USB-устройств
devices-none-description = Ничего не подключено, или служба USBGuard не видит шину USB этой системы.
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
bound-port-toggle = Привязано к USB-порту
bound-port-description = Правило действует, только пока устройство подключено к порту { $port }. В любом другом месте оно блокируется, и вас спросят снова.
bound-port-description-unbound = Сделать так, чтобы правило действовало только на порту, к которому устройство подключено сейчас. Устройству, копирующему его идентификацию, тоже пришлось бы быть в этом порту.
bound-port-unplugged = Подключите устройство, чтобы привязать его правило к текущему порту.

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
notify-thunderbolt = Устройство Thunderbolt ожидает
notify-thunderbolt-body = { $name } подключено, но не может обращаться к памяти компьютера, пока вы его не разрешите.
remember-decision = Запомнить это решение
bind-to-port = Привязать к этому USB-порту ({ $port })
bind-to-port-description = Действует только на этом порту. При подключении к любому другому устройство будет заблокировано, и вас спросят снова.
prompt-wrong-port = Его правило привязано к порту { $bound }, но сейчас оно подключено к порту { $port }.

## Warnings

warning-input-capable = Это устройство может выдавать себя за клавиатуру и вводить текст от вашего имени.
warning-storage = Это устройство предоставляет накопитель.
warning-network = Это устройство предоставляет сетевой адаптер, способный перенаправить ваш трафик.
warning-standing-conflict = Постоянное правило вернёт значение «{ $target }» при следующем подключении.
warning-overriding-rule = Более общее правило, стоящее в политике раньше, тоже подходит к этому устройству и проверяется первым, поэтому оно может решить вместо собственного правила устройства: { $rule }
warning-no-hash = Служба USBGuard не сообщила хэш дескриптора этого устройства, поэтому постоянное правило нельзя привязать именно к нему.

## Status and health

status-ok = Эта система защищена
status-warning = В защите есть проблемы
status-critical = Эта система не защищена
status-disconnected = Нет подключения к службе USBGuard
status-disconnected-description = { $reason }
status-checking = Проверка…

check-daemon-running = Служба USBGuard работает
check-daemon-enabled = Служба USBGuard запускается при загрузке
check-dbus-running = Интерфейс D-Bus службы USBGuard работает
check-dbus-enabled = Интерфейс D-Bus службы USBGuard запускается при загрузке
check-ipc-reachable = Служба отвечает на запросы
check-ipc-permission = Вы можете принимать решения по политике
check-decisions-reversible = Вы можете отменить постоянное решение
check-inserted-policy = Новые устройства ожидают решения
check-policy-not-empty = Политика устройств настроена
check-thunderbolt-security = Устройства Thunderbolt нужно разрешать до подключения
check-thunderbolt-dma = Память защищена от устройств Thunderbolt
check-thunderbolt-service = Служба Thunderbolt (bolt) работает
check-thunderbolt-permission = Вы можете разрешать устройства Thunderbolt
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt и USB4

check-observed = Наблюдается: { $value }
observed-unknown = неизвестно
observed-no-bus = нет подключения к системной шине
observed-responding = отвечает
observed-granted = разрешено
observed-password = требуется пароль администратора
observed-refused = отказано
observed-polkit-unavailable = polkit недоступен
observed-rules = правил в политике: { $count }
observed-unreadable = не удалось прочитать
observed-running = работает
observed-not-installed = не установлено
observed-stopped = установлено, но не запущено
observed-on = включено
observed-off = выключено
observed-not-reported = не сообщается
remedy-heading = Исправьте, выполнив:
remedy-explanation = Скопируйте команду, вставьте её в терминал и нажмите Enter. Она запросит ваш пароль и ничего не выведет при успехе. Затем нажмите «Обновить», чтобы проверить результат.
remedy-policy-warning = Сначала прочтите: это разрешит все устройства, подключённые прямо сейчас. Перед запуском отключите всё, что вы не узнаёте.
steps-heading = Исправьте это в настройках прошивки:
steps-firmware-only = Этот параметр хранится в прошивке компьютера, поэтому никакая команда не может его изменить.
steps-restart = Сохраните работу и перезагрузите компьютер.
steps-open-setup = Во время запуска нажмите клавишу входа в настройки прошивки. Обычно она показана на экране; чаще всего это F1 или Enter (Lenovo), F2 (Dell, ASUS, Acer и большинство других), F10 (HP) и Del (настольные платы).
steps-tb-find = Найдите параметры Thunderbolt или USB4, обычно в разделе Security, Config или Advanced.
steps-tb-set = Установите уровень безопасности «User Authorization» или «Secure Connect». Избегайте «No Security» и «Legacy».
steps-dma-set = Включите «Kernel DMA Protection» или параметр, от которого она зависит: «VT-d» у Intel, «AMD-Vi» или «IOMMU» у AMD. Обычно он находится в разделе Security, Virtualization или Advanced.
steps-save = Сохраните изменения, выйдите и дайте компьютеру загрузиться. Затем нажмите «Обновить» на этой странице.
steps-session-only = Polkit разрешает пользоваться USBGuard только тому, кто сидит за компьютером: активному локальному сеансу. Сеанс через SSH или удалённый рабочий стол, а также сеанс, из которого переключились на другого пользователя, получают отказ.
steps-session-heading = Как исправить:
steps-session-local = Запустите USB Guard в сеансе на самом компьютере.
steps-session-switch = Если вы переключали пользователя, вернитесь в этот сеанс.
steps-session-refresh = Нажмите «Обновить» на этой странице.

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
event-service-up = Служба USBGuard стала доступна
event-service-down = Служба USBGuard стала недоступна
event-health-problem = Проблема конфигурации

actor-user = вами
actor-policy = по правилам службы USBGuard
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
setting-show-hardwired-description = Показывать устройства, которые служба USBGuard считает встроенными; их нельзя отключить.
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
setting-warn-on-health-problems-description = Показывать предупреждение, когда эта система защищена не полностью.
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

error-service-unavailable = Служба USBGuard не запущена
error-permission-denied = Не разрешено
error-autostart = Не удалось изменить настройку автозапуска: { $message }
error-no-tray = На этом рабочем столе нет системного лотка, поэтому значок состояния показать не удалось. Вместо этого USB Guard открыл своё окно.
error-cannot-remove-rule = Для удаления постоянного правила требуется авторизация администратора, которая не была предоставлена. См. «Вы можете отменить постоянное решение» на странице состояния.
error-thunderbolt-permission = Вам не разрешено авторизовать устройства Thunderbolt. На странице «Состояние» есть решение в пункте «Вы можете разрешать устройства Thunderbolt».
error-bolt-unavailable = Служба Thunderbolt (bolt) не запущена. На странице «Состояние» показано, как её запустить.
error-thunderbolt = Служба Thunderbolt сообщила об ошибке: { $message }

## About

repository = Репозиторий
support = Сообщить о проблеме
version = Версия { $version }

## Thunderbolt page

tb-heading = Thunderbolt и USB4
tb-security = Уровень безопасности
tb-level-none = Нет: каждое устройство подключается без вопросов
tb-level-user = Авторизация пользователем: каждое новое устройство нужно разрешить
tb-level-secure = Безопасное подключение: каждое новое устройство нужно разрешить, а запомненные подтверждают подлинность ключом
tb-level-dponly = Только DisplayPort: ни одно устройство не получает доступ к PCIe
tb-level-usbonly = Только USB: ни одно устройство не получает доступ к PCIe
tb-level-nopcie = PCIe отключён: ни одно устройство не получает доступ к PCIe
tb-dma = Защита от DMA
tb-dma-on = Включена: устройства не могут читать память, которая им не выделена
tb-dma-off = Выключена: разрешённое устройство может читать любую память
tb-auto-off = Автоматическая авторизация в bolt отключена, поэтому запомненные устройства тоже каждый раз ждут разрешения.
tb-never-asks = На этом уровне безопасности разрешать нечего: об устройствах никогда не спрашивают.
tb-has-problems = В настройке Thunderbolt есть проблема.
tb-open-status = Показать, как исправить
tb-prompt-heading = Разрешить это устройство Thunderbolt?
tb-prompt-description = { $name } подключено. Пока вы его не разрешите, оно не может обращаться ни к памяти компьютера, ни к шине PCIe.
tb-devices = Устройства
tb-no-devices = Устройства Thunderbolt или USB4 ещё не подключались.
tb-remembered = Запомнено
tb-not-remembered = Не запомнено
tb-status-waiting = Ожидает разрешения
tb-status-authorized = Разрешено
tb-status-error = Ошибка авторизации
tb-status-working = Подключение…
tb-forget = Забыть
tb-forget-note = Подключённое устройство Thunderbolt сохраняет доступ, пока его не отключат; отозвать его раньше нельзя. Забывание устройства вступает в силу при следующем подключении.
