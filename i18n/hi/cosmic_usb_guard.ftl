# SPDX-License-Identifier: GPL-3.0-or-later
# Hindi (हिन्दी) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB गार्ड
app-description = देखें और नियंत्रित करें कि कौन-से USB उपकरण जुड़ सकते हैं

## Generic actions

allow = अनुमति दें
block = रोकें
reject = अस्वीकार करें
revoke = वापस लें
forget = नियम भूलें
details = विवरण
dismiss = हटाएँ
allow-once = एक बार अनुमति दें
always-allow = हमेशा अनुमति दें
refresh = ताज़ा करें
copy = कॉपी करें
copy-command = कमांड कॉपी करें
open-app = USB गार्ड खोलें
quit = बंद करें

## Navigation

page-devices = उपकरण
page-thunderbolt = Thunderbolt
page-history = इतिहास
page-status = स्थिति
page-settings = सेटिंग्स

## Device list

devices-heading = जुड़े हुए उपकरण
devices-none = कोई USB उपकरण नहीं
devices-none-description = कुछ भी कनेक्ट नहीं है, या USBGuard सेवा इस सिस्टम की USB बस नहीं देख पा रही है।
devices-hidden = आपकी प्रदर्शन सेटिंग्स ने { $count } { $count ->
        [one] उपकरण
       *[other] उपकरण
    } छिपाए हैं
devices-pending = { $count } { $count ->
        [one] उपकरण निर्णय की
       *[other] उपकरण निर्णय की
    } प्रतीक्षा में हैं
devices-remembered = जुड़े हुए नहीं
devices-remembered-description = ऐसे उपकरण जिनका स्थायी नियम है पर जो लगे हुए नहीं हैं। नियम यहाँ बदलें या हटाएँ।
device-internal = आंतरिक उपकरण
device-internal-toggle = आंतरिक उपकरण
device-internal-description = यह इसी मशीन का हिस्सा है। सूची से छिपा दिया जाएगा और इसके बारे में कभी नहीं पूछा जाएगा। इससे इसे अनुमति नहीं मिलती — यदि इसे काम करना है तो अलग से अनुमति भी दें।
device-internal-no-hash = यह उपकरण कोई डिस्क्रिप्टर हैश नहीं बताता, इसलिए इसे आंतरिक के रूप में चिह्नित नहीं किया जा सकता।

## Device state

state-allowed = अनुमत
state-blocked = रोका गया
state-rejected = अस्वीकृत
state-unknown = अज्ञात
state-pending = निर्णय प्रतीक्षित
state-disconnected = जुड़ा नहीं है

## Standing policy rules

standing-allow = हमेशा अनुमत
standing-block = हमेशा रोका गया
standing-reject = हमेशा अस्वीकृत
standing-other = स्थायी नियम: { $target }
bound-port-toggle = USB पोर्ट से बँधा
bound-port-description = नियम तभी लागू होता है जब डिवाइस पोर्ट { $port } में लगा हो। कहीं और लगाने पर वह ब्लॉक हो जाता है और आपसे फिर पूछा जाता है।
bound-port-description-unbound = नियम को सिर्फ़ उसी पोर्ट पर लागू करें जिसमें डिवाइस अभी लगा है। इसकी पहचान की नकल करने वाले डिवाइस को भी उसी पोर्ट में होना पड़ेगा।
bound-port-unplugged = डिवाइस का नियम उसके पोर्ट से बाँधने के लिए उसे लगाएँ।

## Device fields

field-name = नाम
field-usb-id = USB आईडी
field-serial = क्रम संख्या
field-port = पोर्ट
field-hash = डिस्क्रिप्टर हैश
field-interfaces = इंटरफ़ेस
field-connection = कनेक्शन
field-status = स्थिति
field-none = सूचित नहीं

## Decision prompt

prompt-heading = इस उपकरण को अनुमति दें?
prompt-description = { $name } अभी जोड़ा गया है। जब तक आप निर्णय नहीं लेते, यह रुका रहेगा।
notify-new-device = नया USB उपकरण जोड़ा गया
notify-auto-blocked = USB उपकरण अस्वीकार किया गया
notify-auto-blocked-body = { $name } को एक स्थायी नियम ने अस्वीकार कर दिया और यह उपलब्ध नहीं है। इसे बदलने के लिए USB गार्ड खोलें।
notify-manage = उपकरण प्रबंधित करें
notify-thunderbolt = Thunderbolt डिवाइस प्रतीक्षा में
notify-thunderbolt-body = { $name } कनेक्ट है, लेकिन जब तक आप अनुमति नहीं देते, यह कंप्यूटर की मेमोरी तक नहीं पहुँच सकता।
remember-decision = यह निर्णय याद रखें
bind-to-port = इस USB पोर्ट से बाँधें ({ $port })
bind-to-port-description = सिर्फ़ इसी पोर्ट पर लागू होता है। किसी और पोर्ट में लगाने पर डिवाइस ब्लॉक हो जाता है और आपसे फिर पूछा जाता है।
prompt-wrong-port = इसका नियम पोर्ट { $bound } से बँधा है, लेकिन अब यह पोर्ट { $port } में लगा है।

## Warnings

warning-input-capable = यह उपकरण कीबोर्ड की तरह काम कर सकता है और आपकी ओर से टाइप कर सकता है।
warning-storage = यह उपकरण भंडारण उपलब्ध कराता है।
warning-network = यह उपकरण एक नेटवर्क अडैप्टर प्रस्तुत करता है, जो आपके ट्रैफ़िक का मार्ग बदल सकता है।
warning-standing-conflict = अगली बार जुड़ने पर एक स्थायी नियम इसे वापस “{ $target }” कर देगा।
warning-overriding-rule = नीति में पहले आने वाला एक व्यापक नियम भी इस डिवाइस से मेल खाता है और पहले जाँचा जाता है, इसलिए वह इस डिवाइस के अपने नियम की जगह फ़ैसला कर सकता है: { $rule }
warning-no-hash = USBGuard सेवा ने इस डिवाइस का डिस्क्रिप्टर हैश नहीं बताया, इसलिए कोई स्थायी नियम खास तौर पर इससे नहीं बाँधा जा सकता।

## Status and health

status-ok = यह सिस्टम सुरक्षित है
status-warning = सुरक्षा में समस्याएँ हैं
status-critical = यह सिस्टम सुरक्षित नहीं है
status-disconnected = USBGuard सेवा से कनेक्ट नहीं है
status-disconnected-description = { $reason }
status-checking = जाँच हो रही है…

check-daemon-running = USBGuard सेवा चल रही है
check-daemon-enabled = USBGuard सेवा बूट पर शुरू होती है
check-dbus-running = USBGuard का D-Bus इंटरफ़ेस चल रहा है
check-dbus-enabled = USBGuard का D-Bus इंटरफ़ेस बूट पर शुरू होता है
check-ipc-reachable = डीमन अनुरोधों का उत्तर देता है
check-ipc-permission = आप नीति संबंधी निर्णय ले सकते हैं
check-decisions-reversible = आप स्थायी निर्णय वापस ले सकते हैं
check-inserted-policy = नए उपकरण निर्णय की प्रतीक्षा करते हैं
check-policy-not-empty = उपकरण नीति कॉन्फ़िगर है
check-thunderbolt-security = Thunderbolt डिवाइसों को कनेक्ट होने से पहले अनुमति मिलनी चाहिए
check-thunderbolt-dma = मेमोरी Thunderbolt डिवाइसों से सुरक्षित है
check-thunderbolt-service = Thunderbolt सेवा (bolt) चल रही है
check-thunderbolt-permission = आप Thunderbolt डिवाइसों को अनुमति दे सकते हैं
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt और USB4

check-observed = देखा गया: { $value }
observed-unknown = अज्ञात
observed-no-bus = सिस्टम बस से कोई कनेक्शन नहीं
observed-responding = जवाब दे रहा है
observed-granted = दी गई
observed-password = व्यवस्थापक पासवर्ड चाहिए
observed-asks-password = पासवर्ड माँगता है, जो ठीक है
observed-refused = अस्वीकृत
observed-polkit-unavailable = polkit उपलब्ध नहीं
observed-rules = नीति में नियम: { $count }
observed-unreadable = पढ़ा नहीं जा सका
observed-running = चल रहा है
observed-not-installed = इंस्टॉल नहीं
observed-stopped = इंस्टॉल है पर चल नहीं रहा
observed-on = चालू
observed-off = बंद
observed-not-reported = रिपोर्ट नहीं किया गया
remedy-heading = इसे ठीक करने के लिए चलाएँ:
remedy-explanation = कमांड कॉपी करें, उसे टर्मिनल में पेस्ट करें और Enter दबाएँ। यह आपका पासवर्ड माँगेगा और सफल होने पर कुछ नहीं दिखाएगा। इसके बाद जाँचने के लिए "ताज़ा करें" दबाएँ कि यह काम कर गया।
remedy-policy-warning = पहले यह पढ़ें: यह अभी लगे हर डिवाइस को अनुमति देता है। चलाने से पहले जिसे आप नहीं पहचानते, उसे निकाल दें।
steps-heading = फ़र्मवेयर सेटअप में ठीक करें:
steps-firmware-only = यह सेटिंग कंप्यूटर के फ़र्मवेयर में रहती है, इसलिए कोई कमांड इसे नहीं बदल सकता।
steps-restart = अपना काम सहेजें और कंप्यूटर पुनः आरंभ करें।
steps-open-setup = कंप्यूटर शुरू होते समय फ़र्मवेयर सेटअप खोलने वाली कुंजी दबाएँ। यह आमतौर पर स्क्रीन पर दिखती है; आम कुंजियाँ हैं F1 या Enter (Lenovo), F2 (Dell, ASUS, Acer और ज़्यादातर अन्य), F10 (HP) और Del (डेस्कटॉप बोर्ड)।
steps-tb-find = Thunderbolt या USB4 सेटिंग ढूँढें, आमतौर पर Security, Config या Advanced में।
steps-tb-set = सुरक्षा स्तर को "User Authorization" या "Secure Connect" पर सेट करें। "No Security" और "Legacy" से बचें।
steps-dma-set = "Kernel DMA Protection" चालू करें, या वह विकल्प जिस पर यह निर्भर है: Intel पर "VT-d", AMD पर "AMD-Vi" या "IOMMU"। यह आमतौर पर Security, Virtualization या Advanced में होता है।
steps-save = बदलाव सहेजें, बाहर निकलें और कंप्यूटर को शुरू होने दें। फिर इस पेज पर "ताज़ा करें" दबाएँ।
steps-session-only = Polkit सिर्फ़ कंप्यूटर के सामने बैठे व्यक्ति को, यानी सक्रिय स्थानीय सत्र को, USBGuard इस्तेमाल करने देता है। SSH या रिमोट डेस्कटॉप वाला सत्र, और जिस सत्र से उपयोगकर्ता बदल लिया गया हो, अस्वीकार होता है।
steps-session-heading = ठीक करने के लिए:
steps-session-local = USB Guard को कंप्यूटर पर ही खुले सत्र में चलाएँ।
steps-session-switch = अगर आपने उपयोगकर्ता बदला था, तो इसी सत्र पर लौटें।
steps-session-refresh = इस पेज पर "ताज़ा करें" दबाएँ।

## History

history-heading = निर्णयों का इतिहास
history-empty = अभी तक कुछ दर्ज नहीं
history-empty-description = उपकरण की घटनाएँ और उन पर लिए गए निर्णय यहाँ दिखाई देंगे।
history-clear = इतिहास मिटाएँ
history-entries = { $count } { $count ->
        [one] प्रविष्टि
       *[other] प्रविष्टियाँ
    }
history-filter-all = सभी घटनाएँ
history-filter-decisions = केवल निर्णय

event-inserted = जुड़ा
event-removed = हटाया गया
event-updated = बदला गया
event-allowed = अनुमत
event-blocked = रोका गया
event-rejected = अस्वीकृत
event-revoked = वापस लिया गया
event-service-up = USBGuard सेवा उपलब्ध हुई
event-service-down = USBGuard सेवा अनुपलब्ध हुई
event-health-problem = कॉन्फ़िगरेशन समस्या

actor-user = आपके द्वारा
actor-policy = USBGuard सेवा के नियमों द्वारा
actor-external = इस ऐप के बाहर
actor-system = स्वचालित

## Hooks

hook-heading = जुड़ने पर एक प्रोग्राम चलाएँ
hook-description = यह तभी चलता है जब इस उपकरण को अनुमति मिल चुकी हो। रोका गया उपकरण कभी कुछ नहीं चलाता।
hook-program = प्रोग्राम
hook-program-placeholder = /home/you/bin/backup.sh
hook-arguments = आर्ग्युमेंट
hook-arguments-placeholder = प्रति पंक्ति एक
hook-label = नाम
hook-label-placeholder = बैकअप
hook-enabled = सक्षम
hook-save = क्रिया सहेजें
hook-remove = क्रिया हटाएँ
hook-none = कोई प्रोग्राम तय नहीं
hook-problem-not-set = चलाने के लिए एक प्रोग्राम चुनें।
hook-problem-not-absolute = स्लैश से शुरू होने वाला पूरा पथ दर्ज करें।
hook-problem-missing = वह फ़ाइल मौजूद नहीं है।
hook-problem-not-executable = वह फ़ाइल निष्पादन योग्य नहीं है। चलाएँ: chmod +x
hook-variables = प्रोग्राम को उपकरण का विवरण पर्यावरण चरों के रूप में मिलता है: { $names }।

## Settings

setting-prompt-on-insert = नए उपकरणों के बारे में पूछें
setting-prompt-on-insert-description = जब बिना स्थायी नियम वाला उपकरण जुड़े तो निर्णय के लिए पूछें।
setting-notify-on-insert = डेस्कटॉप सूचना भेजें
setting-notify-on-insert-description = साथ ही सूचना भी दें, ताकि पैनल छिपा होने पर कोई अनुरोध छूट न जाए।
setting-auto-open-popup = विंडो स्वतः खोलें
setting-auto-open-popup-description = जब किसी उपकरण पर निर्णय चाहिए हो तो USB गार्ड स्वयं अपनी विंडो खोले।
setting-default-permanent = निर्णय डिफ़ॉल्ट रूप से याद रखें
setting-default-permanent-description = अनुरोध में “यह निर्णय याद रखें” पहले से चुना रखें।
setting-show-hardwired = स्थायी रूप से जुड़े उपकरण दिखाएँ
setting-show-hardwired-description = वे डिवाइस शामिल करें जिन्हें USBGuard सेवा हार्डवायर्ड बताती है और जिन्हें निकाला नहीं जा सकता।
setting-show-root-hubs = रूट हब दिखाएँ
setting-show-root-hubs-description = उन होस्ट कंट्रोलर को शामिल करें जिनसे USB पोर्ट जुड़े हैं।
setting-show-internal = आंतरिक उपकरण दिखाएँ
setting-show-internal-description = उन उपकरणों को शामिल करें जिन्हें आपने इस मशीन का हिस्सा बताया है।
setting-show-disconnected = अलग किए गए उपकरण दिखाएँ
setting-show-disconnected-description = ऐसे उपकरण सूचीबद्ध करें जिनका स्थायी नियम है पर जो लगे नहीं हैं, ताकि उनके बिना भी निर्णय बदला जा सके।
setting-warn-input-capable = कीबोर्ड बन सकने वाले उपकरण उजागर करें
setting-warn-input-capable-description = उन उपकरणों को चिह्नित करें जो कीस्ट्रोक डाल सकते हैं।
setting-journal-enabled = निर्णयों का इतिहास रखें
setting-journal-enabled-description = उपकरण घटनाएँ और निर्णय { $path } में दर्ज करें।
setting-warn-on-health-problems = कॉन्फ़िगरेशन समस्याओं की चेतावनी दें
setting-warn-on-health-problems-description = जब यह सिस्टम पूरी तरह सुरक्षित न हो, तब चेतावनी दिखाएँ।
setting-notify-on-auto-block = उपकरण अस्वीकार होने पर बताएँ
setting-notify-on-auto-block-description = जब कोई स्थायी नियम बिना पूछे उपकरण अस्वीकार करे तो सूचित करें, ताकि वह चुपचाप काम करना बंद न कर दे।
setting-show-tray-icon = स्थिति चिह्न दिखाएँ
setting-show-tray-icon-description = सिस्टम ट्रे में ढाल दिखाएँ। इसे बंद करने पर भी ऐप उपकरणों पर नज़र रखता रहेगा।
setting-autostart = लॉगिन पर स्वतः शुरू करें
setting-autostart-description = USB गार्ड को अपने सत्र में जोड़ें ताकि कुछ भी जोड़ने से पहले ही वह नज़र रख रहा हो।
setting-start-minimized = विंडो खोले बिना शुरू करें
setting-start-minimized-description = लॉगिन पर केवल स्थिति चिह्न दिखाएँ।
setting-run-in-background = विंडो बंद होने पर चलता रहे
setting-run-in-background-description = विंडो बंद करने पर USB गार्ड नज़र रखता रहता है। इसे बंद कर दें तो विंडो बंद करते ही ऐप भी बंद हो जाएगा और अनुरोध भी रुक जाएँगे।

section-behaviour = व्यवहार
section-display = प्रदर्शन
section-startup = प्रारंभ और स्थिति चिह्न
section-privacy = इतिहास

## Errors

error-service-unavailable = USBGuard सेवा नहीं चल रही है
error-permission-denied = अनुमति नहीं
error-autostart = स्वतः प्रारंभ सेटिंग नहीं बदली जा सकी: { $message }
error-no-tray = इस डेस्कटॉप में सिस्टम ट्रे नहीं है, इसलिए स्थिति चिह्न नहीं दिखाया जा सका। USB गार्ड ने इसके बजाय अपनी विंडो खोल दी।
error-cannot-remove-rule = स्थायी नियम हटाने के लिए प्रशासक की अनुमति चाहिए, जो नहीं मिली। स्थिति पृष्ठ पर “आप स्थायी निर्णय वापस ले सकते हैं” देखें।
error-thunderbolt-permission = इस सत्र से आपको Thunderbolt डिवाइसों को अधिकृत करने की अनुमति नहीं है। स्थिति पेज पर "आप Thunderbolt डिवाइसों को अनुमति दे सकते हैं" के नीचे इसका कारण बताया गया है।
error-bolt-unavailable = Thunderbolt सेवा (bolt) नहीं चल रही है। स्थिति पेज बताता है कि इसे कैसे शुरू करें।
error-thunderbolt = Thunderbolt सेवा ने एक त्रुटि बताई: { $message }

## About

repository = रिपॉज़िटरी
support = समस्या की सूचना दें
version = संस्करण { $version }

## Thunderbolt page

tb-heading = Thunderbolt और USB4
tb-security = सुरक्षा स्तर
tb-level-none = कोई नहीं: हर डिवाइस बिना पूछे कनेक्ट हो जाता है
tb-level-user = उपयोगकर्ता प्राधिकरण: हर नए डिवाइस को अनुमति चाहिए
tb-level-secure = सुरक्षित कनेक्शन: हर नए डिवाइस को अनुमति चाहिए, और याद रखे गए डिवाइस एक कुंजी से अपनी पहचान साबित करते हैं
tb-level-dponly = सिर्फ़ DisplayPort: किसी डिवाइस को PCIe पहुँच नहीं मिलती
tb-level-usbonly = सिर्फ़ USB: किसी डिवाइस को PCIe पहुँच नहीं मिलती
tb-level-nopcie = PCIe बंद: किसी डिवाइस को PCIe पहुँच नहीं मिलती
tb-dma = DMA सुरक्षा
tb-dma-on = चालू: डिवाइस वह मेमोरी नहीं पढ़ सकते जो उन्हें नहीं दी गई
tb-dma-off = बंद: अनुमति पाया डिवाइस कोई भी मेमोरी पढ़ सकता है
tb-auto-off = bolt में स्वचालित प्राधिकरण बंद है, इसलिए याद रखे गए डिवाइस भी हर बार अनुमति की प्रतीक्षा करते हैं।
tb-never-asks = इस सुरक्षा स्तर पर अनुमति देने को कुछ नहीं है: डिवाइसों के बारे में कभी नहीं पूछा जाता।
tb-has-problems = Thunderbolt सेटअप में एक समस्या है।
tb-open-status = ठीक करने का तरीका दिखाएँ
tb-prompt-heading = इस Thunderbolt डिवाइस को अनुमति दें?
tb-prompt-description = { $name } कनेक्ट है। जब तक आप अनुमति नहीं देते, यह न कंप्यूटर की मेमोरी तक पहुँच सकता है न PCIe बस तक।
tb-devices = डिवाइस
tb-no-devices = कोई Thunderbolt या USB4 डिवाइस कनेक्ट नहीं किया गया है।
tb-remembered = याद रखा गया
tb-not-remembered = याद नहीं रखा गया
tb-status-waiting = अनुमति की प्रतीक्षा में
tb-status-authorized = अनुमति प्राप्त
tb-status-error = प्राधिकरण विफल
tb-status-working = कनेक्ट हो रहा है…
tb-forget = भूल जाएँ
tb-forget-note = कनेक्ट किया गया Thunderbolt डिवाइस निकाले जाने तक अपनी पहुँच बनाए रखता है; इसे पहले वापस लेने का कोई तरीका नहीं है। किसी डिवाइस को भूलना अगली बार कनेक्ट होने पर लागू होता है।
