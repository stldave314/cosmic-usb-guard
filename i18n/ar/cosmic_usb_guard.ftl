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
allow-once = السماح مرة واحدة
always-allow = السماح دائمًا
refresh = تحديث
copy = نسخ
copy-command = نسخ الأمر
open-app = فتح حارس USB
quit = إنهاء

## Navigation

page-devices = الأجهزة
page-thunderbolt = Thunderbolt
page-history = السجل
page-status = الحالة
page-settings = الإعدادات

## Device list

devices-heading = الأجهزة المتصلة
devices-none = لا توجد أجهزة USB
devices-none-description = لا يوجد شيء موصول، أو أن خدمة USBGuard لا ترى ناقل USB في هذا النظام.
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
bound-port-toggle = مربوط بمنفذ USB
bound-port-description = تسري القاعدة فقط ما دام الجهاز موصولًا بالمنفذ { $port }. في أي مكان آخر يُحظر ويُطلب منك القرار مجددًا.
bound-port-description-unbound = اجعل القاعدة تسري فقط على المنفذ الموصول به الجهاز الآن. فأي جهاز ينتحل هويته سيحتاج أيضًا إلى أن يكون في ذلك المنفذ.
bound-port-unplugged = صِل الجهاز لربط قاعدته بالمنفذ الموصول به.

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
notify-thunderbolt = جهاز Thunderbolt في الانتظار
notify-thunderbolt-body = { $name } متصل، لكنه لا يستطيع الوصول إلى ذاكرة الحاسوب حتى تسمح له.
remember-decision = تذكَّر هذا القرار
bind-to-port = الربط بمنفذ USB هذا ({ $port })
bind-to-port-description = يسري على هذا المنفذ فقط. إذا وُصل الجهاز بأي منفذ آخر فسيُحظر وسيُطلب منك القرار مجددًا.
prompt-wrong-port = قاعدته مربوطة بالمنفذ { $bound }، لكنه الآن موصول بالمنفذ { $port }.

## Warnings

warning-input-capable = يمكن لهذا الجهاز أن يتصرف كلوحة مفاتيح ويكتب نيابةً عنك.
warning-storage = يوفّر هذا الجهاز وحدة تخزين.
warning-network = يقدّم هذا الجهاز مهايئ شبكة، ويمكنه تحويل مسار حركة بياناتك.
warning-standing-conflict = ستعيد قاعدة دائمة هذا إلى «{ $target }» في المرة القادمة التي يُوصل فيها.
warning-overriding-rule = قاعدة أعم تسبقها في السياسة تطابق هذا الجهاز أيضًا وتُفحص أولًا، لذا قد تقرر بدلًا من قاعدة الجهاز نفسه: { $rule }
warning-no-hash = لم تُبلغ خدمة USBGuard عن تجزئة واصف لهذا الجهاز، لذا لا يمكن ربط قاعدة دائمة به تحديدًا.

## Status and health

status-ok = هذا النظام محمي
status-warning = توجد مشاكل في الحماية
status-critical = هذا النظام غير محمي
status-disconnected = غير متصل بخدمة USBGuard
status-disconnected-description = { $reason }
status-checking = جارٍ الفحص…

check-daemon-running = خدمة USBGuard تعمل
check-daemon-enabled = خدمة USBGuard تبدأ عند الإقلاع
check-dbus-running = واجهة D-Bus لخدمة USBGuard تعمل
check-dbus-enabled = واجهة D-Bus لخدمة USBGuard تبدأ عند الإقلاع
check-ipc-reachable = الخدمة تستجيب للطلبات
check-ipc-permission = يمكنك اتخاذ قرارات السياسة
check-decisions-reversible = يمكنك التراجع عن قرار دائم
check-inserted-policy = الأجهزة الجديدة تنتظر قرارًا
check-policy-not-empty = توجد سياسة أجهزة مُهيّأة
check-thunderbolt-security = يجب السماح بأجهزة Thunderbolt قبل أن تتصل
check-thunderbolt-dma = الذاكرة محمية من أجهزة Thunderbolt
check-thunderbolt-service = خدمة Thunderbolt ‏(bolt) تعمل
check-thunderbolt-permission = يمكنك السماح بأجهزة Thunderbolt
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt وUSB4

check-observed = المُلاحَظ: { $value }
observed-unknown = غير معروف
observed-no-bus = لا يوجد اتصال بناقل النظام
observed-responding = يستجيب
observed-granted = ممنوح
observed-password = يتطلب كلمة مرور المسؤول
observed-asks-password = يطلب كلمة مرور، وهذا أمر سليم
observed-refused = مرفوض
observed-polkit-unavailable = polkit غير متاح
observed-rules = عدد القواعد في السياسة: { $count }
observed-unreadable = تعذّرت القراءة
observed-running = يعمل
observed-not-installed = غير مثبّت
observed-stopped = مثبّت لكنه لا يعمل
observed-on = مفعّلة
observed-off = متوقفة
observed-not-reported = غير مُبلَّغ عنه
remedy-heading = أصلِح ذلك بتشغيل:
remedy-explanation = انسخ الأمر والصقه في الطرفية ثم اضغط Enter. سيطلب كلمة مرورك ولن يعرض شيئًا عند نجاحه. اضغط بعد ذلك على تحديث للتحقق من نجاحه.
remedy-policy-warning = اقرأ هذا أولًا: يسمح هذا بكل جهاز موصول الآن. افصل أي شيء لا تعرفه قبل تشغيله.
steps-heading = أصلحه من إعداد البرنامج الثابت:
steps-firmware-only = هذا الإعداد مخزّن في البرنامج الثابت للحاسوب، لذا لا يستطيع أي أمر تغييره.
steps-restart = احفظ عملك وأعد تشغيل الحاسوب.
steps-open-setup = أثناء بدء التشغيل اضغط المفتاح الذي يفتح إعداد البرنامج الثابت. يظهر عادة على الشاشة، والشائع منها F1 أو Enter ‏(Lenovo)، وF2 ‏(Dell وASUS وAcer ومعظم الآخرين)، وF10 ‏(HP)، وDel ‏(لوحات الحواسيب المكتبية).
steps-tb-find = ابحث عن إعدادات Thunderbolt أو USB4، وتوجد عادة تحت Security أو Config أو Advanced.
steps-tb-set = اضبط مستوى الأمان على "User Authorization" أو "Secure Connect". تجنب "No Security" و"Legacy".
steps-dma-set = فعّل "Kernel DMA Protection" أو الخيار الذي تعتمد عليه: "VT-d" في Intel، أو "AMD-Vi" أو "IOMMU" في AMD. يوجد عادة تحت Security أو Virtualization أو Advanced.
steps-save = احفظ التغييرات واخرج ودع الحاسوب يبدأ التشغيل. ثم اضغط تحديث في هذه الصفحة.
steps-session-only = لا يسمح polkit باستخدام USBGuard إلا للشخص الموجود عند الحاسوب: الجلسة المحلية النشطة. ويرفض الجلسة عبر SSH أو سطح المكتب البعيد، والجلسة التي تم التبديل عنها.
steps-session-heading = لإصلاح ذلك:
steps-session-local = شغّل USB Guard في الجلسة على الحاسوب نفسه.
steps-session-switch = إذا بدّلت المستخدم، فعد إلى هذه الجلسة.
steps-session-refresh = اضغط تحديث في هذه الصفحة.

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
event-service-up = أصبحت خدمة USBGuard متاحة
event-service-down = أصبحت خدمة USBGuard غير متاحة
event-health-problem = مشكلة في الإعداد

actor-user = بواسطتك
actor-policy = بواسطة قواعد خدمة USBGuard
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
setting-show-hardwired-description = تضمين الأجهزة التي تُبلغ عنها خدمة USBGuard بأنها مدمجة ولا يمكن فصلها.
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
setting-warn-on-health-problems-description = أظهِر تنبيهًا عندما لا يكون هذا النظام محميًا بالكامل.
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

error-service-unavailable = خدمة USBGuard لا تعمل
error-permission-denied = غير مسموح
error-autostart = تعذّر تغيير إعداد البدء التلقائي: { $message }
error-no-tray = لا تحتوي بيئة سطح المكتب هذه على صينية نظام، لذا تعذّر إظهار أيقونة الحالة. فتح حارس USB نافذته بدلًا من ذلك.
error-cannot-remove-rule = تتطلب إزالة قاعدة دائمة تصريحًا من المسؤول، ولم يُمنح. راجِع «يمكنك التراجع عن قرار دائم» في صفحة الحالة.
error-thunderbolt-permission = غير مسموح لك بتخويل أجهزة Thunderbolt من هذه الجلسة. توضح صفحة الحالة السبب تحت "يمكنك السماح بأجهزة Thunderbolt".
error-bolt-unavailable = خدمة Thunderbolt ‏(bolt) لا تعمل. توضح صفحة الحالة كيفية تشغيلها.
error-thunderbolt = أبلغت خدمة Thunderbolt عن خطأ: { $message }

## About

repository = المستودع
support = الإبلاغ عن مشكلة
version = الإصدار { $version }

## Thunderbolt page

tb-heading = Thunderbolt وUSB4
tb-security = مستوى الأمان
tb-level-none = لا شيء: يتصل كل جهاز دون أن يُسأل عنه
tb-level-user = تخويل المستخدم: يجب السماح بكل جهاز جديد
tb-level-secure = اتصال آمن: يجب السماح بكل جهاز جديد، وتثبت الأجهزة المحفوظة هويتها بمفتاح
tb-level-dponly = DisplayPort فقط: لا يحصل أي جهاز على وصول PCIe
tb-level-usbonly = USB فقط: لا يحصل أي جهاز على وصول PCIe
tb-level-nopcie = PCIe معطّل: لا يحصل أي جهاز على وصول PCIe
tb-dma = الحماية من DMA
tb-dma-on = مفعّلة: لا تستطيع الأجهزة قراءة ذاكرة لم تُمنح لها
tb-dma-off = متوقفة: يستطيع الجهاز المسموح به قراءة أي ذاكرة
tb-auto-off = التخويل التلقائي متوقف في bolt، لذا تنتظر الأجهزة المحفوظة أيضًا السماح لها في كل مرة.
tb-never-asks = لا يوجد ما يُسمح به في مستوى الأمان هذا: لا يُسأل عن الأجهزة أبدًا.
tb-has-problems = توجد مشكلة في إعداد Thunderbolt.
tb-open-status = اعرض طريقة الإصلاح
tb-prompt-heading = هل تسمح بجهاز Thunderbolt هذا؟
tb-prompt-description = { $name } متصل. حتى تسمح له، لا يستطيع الوصول إلى ذاكرة الحاسوب ولا إلى ناقل PCIe.
tb-devices = الأجهزة
tb-no-devices = لم يُوصل أي جهاز Thunderbolt أو USB4.
tb-remembered = محفوظ
tb-not-remembered = غير محفوظ
tb-status-waiting = ينتظر السماح
tb-status-authorized = مسموح
tb-status-error = فشل التخويل
tb-status-working = جارٍ الاتصال…
tb-forget = نسيان
tb-forget-note = يحتفظ جهاز Thunderbolt المتصل بصلاحيته حتى يُفصل، ولا توجد طريقة لسحبها قبل ذلك. يسري نسيان الجهاز في المرة التالية التي يُوصل فيها.
