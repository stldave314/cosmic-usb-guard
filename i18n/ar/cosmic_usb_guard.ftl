# SPDX-License-Identifier: GPL-3.0-or-later
# Arabic (العربية) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = حارس USB
app-description = راجِع وتحكَّم في أجهزة USB المسموح لها بالاتصال

## Generic actions

allow = السماح
block = الحجب
reject = الرفض
revoke = إبطال
forget = نسيان القاعدة
details = التفاصيل
dismiss = تجاهل
refresh = تحديث
copy = نسخ
open-app = فتح حارس USB
quit = إنهاء

## Navigation

page-devices = الأجهزة
page-history = السجل
page-status = الحالة
page-settings = الإعدادات

## Device list

devices-heading = الأجهزة المتصلة
devices-none = لا توجد أجهزة USB
devices-none-description = لا شيء متصل، أو أن USBGuard لا يرى ناقل USB في هذا النظام.
devices-hidden = { $count ->
        [zero] لا أجهزة مخفية
        [one] جهاز واحد مخفي
        [two] جهازان مخفيان
        [few] { $count } أجهزة مخفية
        [many] { $count } جهازًا مخفيًا
       *[other] { $count } جهاز مخفي
    } بسبب إعدادات العرض
devices-pending = { $count ->
        [zero] لا أجهزة تنتظر
        [one] جهاز واحد ينتظر
        [two] جهازان ينتظران
        [few] { $count } أجهزة تنتظر
        [many] { $count } جهازًا ينتظر
       *[other] { $count } جهاز ينتظر
    } قرارًا
devices-remembered = غير متصلة
devices-remembered-description = أجهزة لها قاعدة دائمة لكنها غير موصولة. يمكنك تغيير القاعدة أو إزالتها من هنا.
device-internal = جهاز داخلي
device-internal-toggle = جهاز داخلي
device-internal-description = جزء من هذا الحاسوب. يُخفى من القائمة ولا يُسأل عنه أبدًا. هذا لا يمنحه إذنًا — اسمح له أيضًا إذا كان يجب أن يعمل.
device-internal-no-hash = لا يُبلِّغ هذا الجهاز عن بصمة واصف، لذا لا يمكن تعليمه كجهاز داخلي.

## Device state

state-allowed = مسموح
state-blocked = محجوب
state-rejected = مرفوض
state-unknown = غير معروف
state-pending = بانتظار قرار
state-disconnected = غير متصل

## Standing policy rules

standing-allow = مسموح دائمًا
standing-block = محجوب دائمًا
standing-reject = مرفوض دائمًا
standing-other = قاعدة دائمة: { $target }

## Device fields

field-name = الاسم
field-usb-id = معرّف USB
field-serial = الرقم التسلسلي
field-port = المنفذ
field-hash = بصمة الواصف
field-interfaces = الواجهات
field-connection = الاتصال
field-status = الحالة
field-none = غير مُبلَّغ عنه

## Decision prompt

prompt-heading = هل تسمح لهذا الجهاز؟
prompt-description = تم توصيل { $name } للتو. سيبقى محجوبًا حتى تقرر.
notify-new-device = تم توصيل جهاز USB جديد
notify-auto-blocked = تم رفض جهاز USB
notify-auto-blocked-body = رُفض { $name } بسبب قاعدة دائمة وهو غير متاح. افتح حارس USB لتغيير ذلك.
notify-manage = إدارة الجهاز
remember-decision = تذكَّر هذا القرار

## Warnings

warning-input-capable = يمكن لهذا الجهاز أن يتصرف كلوحة مفاتيح ويكتب نيابةً عنك.
warning-storage = يوفّر هذا الجهاز وحدة تخزين.
warning-network = يقدّم هذا الجهاز مهايئ شبكة، ويمكنه تحويل مسار حركة بياناتك.
warning-standing-conflict = ستعيد قاعدة دائمة هذا إلى «{ $target }» في المرة القادمة التي يُوصل فيها.
warning-no-hash = لم يُبلّغ USBGuard عن بصمة واصف لهذا الجهاز، لذا لا يمكن ربط قاعدة دائمة به تحديدًا.

## Status and health

status-ok = USBGuard يحمي هذا النظام
status-warning = USBGuard يعمل مع وجود مشاكل
status-critical = هذا النظام غير محمي
status-disconnected = غير متصل بـ USBGuard
status-disconnected-description = { $reason }
status-checking = جارٍ الفحص…

check-daemon-running = خدمة USBGuard تعمل
check-daemon-enabled = خدمة USBGuard تبدأ عند الإقلاع
check-dbus-running = واجهة D-Bus الخاصة بـ USBGuard تعمل
check-dbus-enabled = واجهة D-Bus الخاصة بـ USBGuard تبدأ عند الإقلاع
check-ipc-reachable = الخدمة تستجيب للطلبات
check-ipc-permission = يمكنك اتخاذ قرارات السياسة
check-decisions-reversible = يمكنك التراجع عن قرار دائم
check-inserted-policy = الأجهزة الجديدة تنتظر قرارًا
check-policy-not-empty = توجد سياسة أجهزة مُهيّأة

check-observed = المُلاحَظ: { $value }
remedy-heading = أصلِح ذلك بتشغيل:

## History

history-heading = سجل القرارات
history-empty = لم يُسجَّل شيء بعد
history-empty-description = ستظهر هنا أحداث الأجهزة والقرارات المتخذة بشأنها.
history-clear = مسح السجل
history-entries = { $count ->
        [zero] لا مُدخلات
        [one] مُدخل واحد
        [two] مُدخلان
        [few] { $count } مُدخلات
        [many] { $count } مُدخلًا
       *[other] { $count } مُدخل
    }
history-filter-all = كل الأحداث
history-filter-decisions = القرارات فقط

event-inserted = متصل
event-removed = مفصول
event-updated = تغيّر
event-allowed = سُمح به
event-blocked = حُجب
event-rejected = رُفض
event-revoked = أُبطل
event-service-up = أصبح USBGuard متاحًا
event-service-down = أصبح USBGuard غير متاح
event-health-problem = مشكلة في الإعداد

actor-user = بواسطتك
actor-policy = بواسطة سياسة USBGuard
actor-external = من خارج هذا التطبيق
actor-system = تلقائي

## Hooks

hook-heading = تشغيل برنامج عند التوصيل
hook-description = لا يعمل إلا بعد السماح لهذا الجهاز. الجهاز المحجوب لا يشغّل أي شيء إطلاقًا.
hook-program = البرنامج
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = الوسائط
hook-arguments-placeholder = واحدة في كل سطر
hook-label = الاسم
hook-label-placeholder = نسخة احتياطية
hook-enabled = مُفعَّل
hook-save = حفظ الإجراء
hook-remove = إزالة الإجراء
hook-none = لم يُحدَّد برنامج
hook-problem-not-set = اختر برنامجًا لتشغيله.
hook-problem-not-absolute = أدخل المسار الكامل، بادئًا بشرطة مائلة.
hook-problem-missing = هذا الملف غير موجود.
hook-problem-not-executable = هذا الملف غير قابل للتنفيذ. شغّل: chmod +x
hook-variables = يتلقى البرنامج تفاصيل الجهاز عبر متغيرات البيئة: { $names }.

## Settings

setting-prompt-on-insert = اسأل عن الأجهزة الجديدة
setting-prompt-on-insert-description = أظهِر طلب قرار عند توصيل جهاز ليس له قاعدة دائمة.
setting-notify-on-insert = إرسال إشعار سطح المكتب
setting-notify-on-insert-description = أرسِل إشعارًا أيضًا حتى لا يفوتك الطلب عندما تكون اللوحة مخفية.
setting-auto-open-popup = فتح النافذة تلقائيًا
setting-auto-open-popup-description = دع حارس USB يفتح نافذته بنفسه عندما يحتاج جهاز إلى قرار.
setting-default-permanent = تذكَّر القرارات افتراضيًا
setting-default-permanent-description = علِّم «تذكَّر هذا القرار» مسبقًا في الطلب.
setting-show-hardwired = إظهار الأجهزة المُلحَمة
setting-show-hardwired-description = تضمين الأجهزة التي يعتبرها USBGuard ثابتة ولا يمكن فصلها.
setting-show-root-hubs = إظهار المُوزِّعات الجذرية
setting-show-root-hubs-description = تضمين متحكّمات المضيف التي تتفرع منها منافذ USB.
setting-show-internal = إظهار الأجهزة الداخلية
setting-show-internal-description = تضمين الأجهزة التي علّمتها كجزء من هذا الحاسوب.
setting-show-disconnected = إظهار الأجهزة غير المتصلة
setting-show-disconnected-description = أدرِج الأجهزة التي لها قاعدة دائمة لكنها غير موصولة، حتى يمكن تغيير القرار من دونها.
setting-warn-input-capable = إبراز الأجهزة القادرة على العمل كلوحة مفاتيح
setting-warn-input-capable-description = نبّه إلى الأجهزة التي يمكنها حقن ضغطات المفاتيح.
setting-journal-enabled = الاحتفاظ بسجل القرارات
setting-journal-enabled-description = سجِّل أحداث الأجهزة والقرارات في { $path }.
setting-warn-on-health-problems = التحذير من مشاكل الإعداد
setting-warn-on-health-problems-description = أظهِر تنبيهًا عندما لا يكون USBGuard مُهيّأً لحماية هذا النظام.
setting-notify-on-auto-block = أخبِرني عند رفض جهاز
setting-notify-on-auto-block-description = نبِّه عندما ترفض قاعدة دائمة جهازًا دون سؤال، حتى لا يتوقف عن العمل بصمت.
setting-show-tray-icon = إظهار أيقونة الحالة
setting-show-tray-icon-description = أظهِر الدرع في صينية النظام. إيقاف ذلك لا يمنع التطبيق من مراقبة الأجهزة.
setting-autostart = البدء تلقائيًا عند تسجيل الدخول
setting-autostart-description = أضِف حارس USB إلى جلستك ليكون مراقبًا قبل أن توصّل أي شيء.
setting-start-minimized = البدء دون فتح النافذة
setting-start-minimized-description = عند تسجيل الدخول، أظهِر أيقونة الحالة فقط.
setting-run-in-background = الاستمرار في العمل عند إغلاق النافذة
setting-run-in-background-description = إغلاق النافذة يترك حارس USB يراقب. أوقِف هذا وسيؤدي الإغلاق إلى إنهاء التطبيق، مما يوقف الطلبات أيضًا.

section-behaviour = السلوك
section-display = العرض
section-startup = بدء التشغيل وأيقونة الحالة
section-privacy = السجل

## Errors

error-service-unavailable = USBGuard لا يعمل
error-permission-denied = غير مسموح
error-autostart = تعذّر تغيير إعداد البدء التلقائي: { $message }
error-no-tray = لا تحتوي بيئة سطح المكتب هذه على صينية نظام، لذا تعذّر إظهار أيقونة الحالة. فتح حارس USB نافذته بدلًا من ذلك.
error-cannot-remove-rule = تتطلب إزالة قاعدة دائمة تصريحًا من المسؤول، ولم يُمنح. راجِع «يمكنك التراجع عن قرار دائم» في صفحة الحالة.

## About

repository = المستودع
support = الإبلاغ عن مشكلة
version = الإصدار { $version }
