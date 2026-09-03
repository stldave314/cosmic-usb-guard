# SPDX-License-Identifier: GPL-3.0-or-later
# Turkish (Türkçe) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Hangi USB aygıtlarının bağlanabileceğini inceleyin ve denetleyin

## Generic actions

allow = İzin ver
block = Engelle
reject = Reddet
revoke = Geri al
forget = Kuralı unut
details = Ayrıntılar
dismiss = Yoksay
refresh = Yenile
copy = Kopyala
open-app = USB Guard'ı aç
quit = Çık

## Navigation

page-devices = Aygıtlar
page-history = Geçmiş
page-status = Durum
page-settings = Ayarlar

## Device list

devices-heading = Bağlı aygıtlar
devices-none = USB aygıtı yok
devices-none-description = Hiçbir şey bağlı değil ya da USBGuard bu sistemin USB veri yolunu göremiyor.
devices-hidden = Görünüm ayarlarınız { $count } aygıtı gizliyor
devices-pending = { $count } aygıt karar bekliyor
devices-remembered = Bağlı değil
devices-remembered-description = Kalıcı kuralı olan ancak takılı olmayan aygıtlar. Kuralı buradan değiştirebilir veya kaldırabilirsiniz.
device-internal = Dahili aygıt
device-internal-toggle = Dahili aygıt
device-internal-description = Bu makinenin bir parçası. Listeden gizlenir ve bir daha sorulmaz. Bu ona izin vermez — çalışması gerekiyorsa ayrıca izin verin.
device-internal-no-hash = Bu aygıt bir tanımlayıcı özeti bildirmiyor, bu yüzden dahili olarak işaretlenemez.

## Device state

state-allowed = İzin verildi
state-blocked = Engellendi
state-rejected = Reddedildi
state-unknown = Bilinmiyor
state-pending = Karar bekliyor
state-disconnected = Bağlı değil

## Standing policy rules

standing-allow = Her zaman izin verilir
standing-block = Her zaman engellenir
standing-reject = Her zaman reddedilir
standing-other = Kalıcı kural: { $target }

## Device fields

field-name = Ad
field-usb-id = USB kimliği
field-serial = Seri numarası
field-port = Bağlantı noktası
field-hash = Tanımlayıcı özeti
field-interfaces = Arabirimler
field-connection = Bağlantı
field-status = Durum
field-none = Bildirilmedi

## Decision prompt

prompt-heading = Bu aygıta izin verilsin mi?
prompt-description = { $name } az önce bağlandı. Siz karar verene kadar engelli kalır.
notify-new-device = Yeni USB aygıtı bağlandı
notify-auto-blocked = USB aygıtı reddedildi
notify-auto-blocked-body = { $name } kalıcı bir kural tarafından reddedildi ve kullanılamıyor. Bunu değiştirmek için USB Guard'ı açın.
notify-manage = Aygıtı yönet
remember-decision = Bu kararı hatırla

## Warnings

warning-input-capable = Bu aygıt klavye gibi davranıp sizin adınıza yazabilir.
warning-storage = Bu aygıt depolama sunuyor.
warning-network = Bu aygıt bir ağ bağdaştırıcısı sunuyor; trafiğinizi başka yöne çevirebilir.
warning-standing-conflict = Kalıcı bir kural, bir sonraki bağlanışta bunu “{ $target }” durumuna geri döndürecek.
warning-no-hash = USBGuard bu aygıt için bir tanımlayıcı özeti bildirmedi, bu yüzden kalıcı bir kural yalnızca bu aygıta sabitlenemez.

## Status and health

status-ok = USBGuard bu sistemi koruyor
status-warning = USBGuard sorunlarla çalışıyor
status-critical = Bu sistem korunmuyor
status-disconnected = USBGuard'a bağlı değil
status-disconnected-description = { $reason }
status-checking = Denetleniyor…

check-daemon-running = USBGuard hizmeti çalışıyor
check-daemon-enabled = USBGuard hizmeti açılışta başlıyor
check-dbus-running = USBGuard D-Bus arabirimi çalışıyor
check-dbus-enabled = USBGuard D-Bus arabirimi açılışta başlıyor
check-ipc-reachable = Hizmet isteklere yanıt veriyor
check-ipc-permission = İlke kararları verebilirsiniz
check-decisions-reversible = Kalıcı bir kararı geri alabilirsiniz
check-inserted-policy = Yeni aygıtlar karar bekliyor
check-policy-not-empty = Bir aygıt ilkesi yapılandırılmış

check-observed = Gözlemlenen: { $value }
remedy-heading = Şunu çalıştırarak düzeltin:

## History

history-heading = Karar geçmişi
history-empty = Henüz kayıt yok
history-empty-description = Aygıt olayları ve onlar hakkında verilen kararlar burada görünecek.
history-clear = Geçmişi temizle
history-entries = { $count } kayıt
history-filter-all = Tüm olaylar
history-filter-decisions = Yalnızca kararlar

event-inserted = Bağlandı
event-removed = Çıkarıldı
event-updated = Değişti
event-allowed = İzin verildi
event-blocked = Engellendi
event-rejected = Reddedildi
event-revoked = Geri alındı
event-service-up = USBGuard kullanılabilir hâle geldi
event-service-down = USBGuard kullanılamaz hâle geldi
event-health-problem = Yapılandırma sorunu

actor-user = sizin tarafınızdan
actor-policy = USBGuard ilkesiyle
actor-external = bu uygulamanın dışında
actor-system = otomatik

## Hooks

hook-heading = Bağlandığında bir program çalıştır
hook-description = Yalnızca bu aygıta izin verildikten sonra çalışır. Engellenen bir aygıt asla bir şey çalıştırmaz.
hook-program = Program
hook-program-placeholder = /home/siz/bin/yedek.sh
hook-arguments = Bağımsız değişkenler
hook-arguments-placeholder = Her satıra bir tane
hook-label = Ad
hook-label-placeholder = Yedekleme
hook-enabled = Etkin
hook-save = Eylemi kaydet
hook-remove = Eylemi kaldır
hook-none = Program ayarlanmadı
hook-problem-not-set = Çalıştırılacak bir program seçin.
hook-problem-not-absolute = Eğik çizgiyle başlayan tam yolu girin.
hook-problem-missing = Bu dosya yok.
hook-problem-not-executable = Bu dosya çalıştırılabilir değil. Şunu çalıştırın: chmod +x
hook-variables = Program, aygıt bilgilerini ortam değişkenleri olarak alır: { $names }.

## Settings

setting-prompt-on-insert = Yeni aygıtları sor
setting-prompt-on-insert-description = Kalıcı kuralı olmayan bir aygıt bağlandığında karar istemi göster.
setting-notify-on-insert = Masaüstü bildirimi gönder
setting-notify-on-insert-description = Panel gizliyken bir istem kaçmasın diye ayrıca bildirim gönder.
setting-auto-open-popup = Pencereyi kendiliğinden aç
setting-auto-open-popup-description = Bir aygıt karar gerektirdiğinde USB Guard kendi penceresini açsın.
setting-default-permanent = Kararları varsayılan olarak hatırla
setting-default-permanent-description = İstemde “Bu kararı hatırla” seçeneğini önceden işaretle.
setting-show-hardwired = Lehimli aygıtları göster
setting-show-hardwired-description = USBGuard'ın sabit bağlantılı olarak bildirdiği, çıkarılamayan aygıtları da içer.
setting-show-root-hubs = Kök hub'ları göster
setting-show-root-hubs-description = USB bağlantı noktalarının bağlı olduğu ana makine denetleyicilerini de içer.
setting-show-internal = Dahili aygıtları göster
setting-show-internal-description = Bu makinenin parçası olarak işaretlediğiniz aygıtları da içer.
setting-show-disconnected = Bağlı olmayan aygıtları göster
setting-show-disconnected-description = Kalıcı kuralı olan ama takılı olmayan aygıtları listele; böylece bir karar aygıt olmadan da değiştirilebilir.
setting-warn-input-capable = Klavye olabilecek aygıtları vurgula
setting-warn-input-capable-description = Tuş vuruşu enjekte edebilecek aygıtları belirt.
setting-journal-enabled = Karar geçmişi tut
setting-journal-enabled-description = Aygıt olaylarını ve kararları { $path } dosyasına kaydet.
setting-warn-on-health-problems = Yapılandırma sorunlarını bildir
setting-warn-on-health-problems-description = USBGuard bu sistemi koruyacak şekilde kurulmadığında bir uyarı göster.
setting-notify-on-auto-block = Bir aygıt reddedildiğinde bana söyle
setting-notify-on-auto-block-description = Kalıcı bir kural sormadan bir aygıtı reddettiğinde bildir; böylece sessizce çalışmamış olmaz.
setting-show-tray-icon = Durum simgesini göster
setting-show-tray-icon-description = Kalkanı sistem tepsisinde göster. Bunu kapatmak uygulamanın aygıtları izlemesini durdurmaz.
setting-autostart = Oturum açıldığında kendiliğinden başlat
setting-autostart-description = USB Guard'ı oturumunuza ekleyin; siz bir şey takmadan önce izliyor olsun.
setting-start-minimized = Pencereyi açmadan başlat
setting-start-minimized-description = Oturum açıldığında yalnızca durum simgesini göster.
setting-run-in-background = Pencere kapatıldığında çalışmayı sürdür
setting-run-in-background-description = Pencereyi kapatmak USB Guard'ı izlemede bırakır. Bunu kapatırsanız pencereyi kapatmak uygulamadan çıkar ve istemler de durur.

section-behaviour = Davranış
section-display = Görünüm
section-startup = Başlangıç ve durum simgesi
section-privacy = Geçmiş

## Errors

error-service-unavailable = USBGuard çalışmıyor
error-permission-denied = İzin verilmedi
error-autostart = Otomatik başlatma ayarı değiştirilemedi: { $message }
error-no-tray = Bu masaüstünde sistem tepsisi yok, bu yüzden durum simgesi gösterilemedi. USB Guard bunun yerine penceresini açtı.
error-cannot-remove-rule = Kalıcı bir kuralı kaldırmak yönetici yetkisi gerektirir ve bu yetki verilmedi. Durum sayfasındaki “Kalıcı bir kararı geri alabilirsiniz” maddesine bakın.

## About

repository = Depo
support = Sorun bildir
version = Sürüm { $version }
