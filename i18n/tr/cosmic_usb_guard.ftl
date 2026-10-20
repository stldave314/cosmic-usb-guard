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
allow-once = Bir kez izin ver
always-allow = Her zaman izin ver
refresh = Yenile
copy = Kopyala
copy-command = Komutu kopyala
open-app = USB Guard'ı aç
quit = Çık

## Navigation

page-devices = Aygıtlar
page-thunderbolt = Thunderbolt
page-history = Geçmiş
page-status = Durum
page-settings = Ayarlar

## Device list

devices-heading = Bağlı aygıtlar
devices-none = USB aygıtı yok
devices-none-description = Hiçbir şey bağlı değil ya da USBGuard hizmeti bu sistemin USB veri yolunu göremiyor.
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
bound-port-toggle = USB bağlantı noktasına bağlı
bound-port-description = Kural yalnızca aygıt { $port } bağlantı noktasına takılıyken geçerlidir. Başka bir yerde engellenir ve size yeniden sorulur.
bound-port-description-unbound = Kuralın yalnızca aygıtın şu an takılı olduğu bağlantı noktasında geçerli olmasını sağlayın. Kimliğini taklit eden bir aygıtın da o bağlantı noktasında olması gerekir.
bound-port-unplugged = Kuralını takılı olduğu bağlantı noktasına bağlamak için aygıtı takın.

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
notify-thunderbolt = Thunderbolt aygıtı bekliyor
notify-thunderbolt-body = { $name } bağlı, ancak siz izin verene kadar bilgisayarın belleğine erişemez.
remember-decision = Bu kararı hatırla
bind-to-port = Bu USB bağlantı noktasına bağla ({ $port })
bind-to-port-description = Yalnızca bu bağlantı noktasında geçerlidir. Başka bir yere takılırsa aygıt engellenir ve size yeniden sorulur.
prompt-wrong-port = Kuralı { $bound } bağlantı noktasına bağlı, ancak şu an { $port } bağlantı noktasına takılı.

## Warnings

warning-input-capable = Bu aygıt klavye gibi davranıp sizin adınıza yazabilir.
warning-storage = Bu aygıt depolama sunuyor.
warning-network = Bu aygıt bir ağ bağdaştırıcısı sunuyor; trafiğinizi başka yöne çevirebilir.
warning-standing-conflict = Kalıcı bir kural, bir sonraki bağlanışta bunu “{ $target }” durumuna geri döndürecek.
warning-overriding-rule = İlkede daha önce gelen daha geniş bir kural da bu aygıtla eşleşiyor ve önce denetleniyor; bu yüzden aygıtın kendi kuralı yerine o karar verebilir: { $rule }
warning-no-hash = USBGuard hizmeti bu aygıt için bir tanımlayıcı karması bildirmedi, bu yüzden kalıcı bir kural özellikle bu aygıta bağlanamaz.

## Status and health

status-ok = Bu sistem korunuyor
status-warning = Korumada sorunlar var
status-critical = Bu sistem korunmuyor
status-disconnected = USBGuard hizmetine bağlı değil
status-disconnected-description = { $reason }
status-checking = Denetleniyor…

check-daemon-running = USBGuard hizmeti çalışıyor
check-daemon-enabled = USBGuard hizmeti açılışta başlıyor
check-dbus-running = USBGuard D-Bus arayüzü çalışıyor
check-dbus-enabled = USBGuard D-Bus arayüzü açılışta başlıyor
check-ipc-reachable = Hizmet isteklere yanıt veriyor
check-ipc-permission = İlke kararları verebilirsiniz
check-decisions-reversible = Kalıcı bir kararı geri alabilirsiniz
check-inserted-policy = Yeni aygıtlar karar bekliyor
check-policy-not-empty = Bir aygıt ilkesi yapılandırılmış
check-thunderbolt-security = Thunderbolt aygıtlarına bağlanmadan önce izin verilmelidir
check-thunderbolt-dma = Bellek Thunderbolt aygıtlarına karşı korunuyor
check-thunderbolt-service = Thunderbolt hizmeti (bolt) çalışıyor
check-thunderbolt-permission = Thunderbolt aygıtlarına izin verebilirsiniz
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt ve USB4

check-observed = Gözlemlenen: { $value }
observed-unknown = bilinmiyor
observed-no-bus = sistem veri yoluna bağlantı yok
observed-responding = yanıt veriyor
observed-granted = verildi
observed-password = yönetici parolası gerektiriyor
observed-asks-password = parola soruyor, bu sorun değil
observed-refused = reddedildi
observed-polkit-unavailable = polkit kullanılamıyor
observed-rules = ilkedeki kural sayısı: { $count }
observed-unreadable = okunamadı
observed-running = çalışıyor
observed-not-installed = kurulu değil
observed-stopped = kurulu ama çalışmıyor
observed-on = açık
observed-off = kapalı
observed-not-reported = bildirilmiyor
remedy-heading = Şunu çalıştırarak düzeltin:
remedy-explanation = Komutu kopyalayın, bir terminale yapıştırın ve Enter tuşuna basın. Parolanızı sorar ve başarılı olduğunda hiçbir şey yazdırmaz. Ardından işe yarayıp yaramadığını görmek için Yenile'ye basın.
remedy-policy-warning = Önce okuyun: bu, şu anda takılı olan her aygıta izin verir. Çalıştırmadan önce tanımadığınız her şeyi çıkarın.
steps-heading = Donanım yazılımı kurulumunda düzeltin:
steps-firmware-only = Bu ayar bilgisayarın donanım yazılımında saklanır, bu yüzden hiçbir komut onu değiştiremez.
steps-restart = Çalışmanızı kaydedin ve bilgisayarı yeniden başlatın.
steps-open-setup = Bilgisayar açılırken donanım yazılımı kurulumunu açan tuşa basın. Tuş genellikle ekranda gösterilir; yaygın olanlar F1 veya Enter (Lenovo), F2 (Dell, ASUS, Acer ve çoğu diğerleri), F10 (HP) ve Del'dir (masaüstü anakartlar).
steps-tb-find = Thunderbolt veya USB4 ayarlarını bulun; genellikle Security, Config ya da Advanced altındadır.
steps-tb-set = Güvenlik düzeyini "User Authorization" veya "Secure Connect" olarak ayarlayın. "No Security" ve "Legacy" seçeneklerinden kaçının.
steps-dma-set = "Kernel DMA Protection" seçeneğini ya da bağlı olduğu seçeneği açın: Intel'de "VT-d", AMD'de "AMD-Vi" veya "IOMMU". Genellikle Security, Virtualization ya da Advanced altındadır.
steps-save = Değişiklikleri kaydedin, çıkın ve bilgisayarın açılmasını bekleyin. Ardından bu sayfada Yenile'ye basın.
steps-session-only = Polkit, USBGuard'ı yalnızca bilgisayarın başındaki kişinin, yani etkin yerel oturumun kullanmasına izin verir. SSH veya uzak masaüstü ile açılan oturumu ve başka kullanıcıya geçilmiş oturumu reddeder.
steps-session-heading = Düzeltmek için:
steps-session-local = USB Guard'ı bilgisayarın başındaki oturumda çalıştırın.
steps-session-switch = Kullanıcı değiştirdiyseniz bu oturuma geri dönün.
steps-session-refresh = Bu sayfada Yenile'ye basın.

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
event-service-up = USBGuard hizmeti kullanılabilir oldu
event-service-down = USBGuard hizmeti kullanılamaz oldu
event-health-problem = Yapılandırma sorunu

actor-user = sizin tarafınızdan
actor-policy = USBGuard hizmetinin kurallarıyla
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
setting-show-hardwired-description = USBGuard hizmetinin sabit bağlı olarak bildirdiği, çıkarılamayan aygıtları dahil et.
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
setting-warn-on-health-problems-description = Bu sistem tam olarak korunmadığında bir uyarı göster.
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

error-service-unavailable = USBGuard hizmeti çalışmıyor
error-permission-denied = İzin verilmedi
error-autostart = Otomatik başlatma ayarı değiştirilemedi: { $message }
error-no-tray = Bu masaüstünde sistem tepsisi yok, bu yüzden durum simgesi gösterilemedi. USB Guard bunun yerine penceresini açtı.
error-cannot-remove-rule = Kalıcı bir kuralı kaldırmak yönetici yetkisi gerektirir ve bu yetki verilmedi. Durum sayfasındaki “Kalıcı bir kararı geri alabilirsiniz” maddesine bakın.
error-thunderbolt-permission = Bu oturumdan Thunderbolt aygıtlarını yetkilendirme izniniz yok. Nedeni Durum sayfasında "Thunderbolt aygıtlarına izin verebilirsiniz" altında açıklanıyor.
error-bolt-unavailable = Thunderbolt hizmeti (bolt) çalışmıyor. Durum sayfası nasıl başlatılacağını gösterir.
error-thunderbolt = Thunderbolt hizmeti bir hata bildirdi: { $message }

## About

repository = Depo
support = Sorun bildir
version = Sürüm { $version }

## Thunderbolt page

tb-heading = Thunderbolt ve USB4
tb-security = Güvenlik düzeyi
tb-level-none = Yok: her aygıt sorulmadan bağlanır
tb-level-user = Kullanıcı yetkilendirmesi: her yeni aygıta izin verilmelidir
tb-level-secure = Güvenli bağlantı: her yeni aygıta izin verilmelidir ve hatırlanan aygıtlar kimliklerini bir anahtarla kanıtlar
tb-level-dponly = Yalnızca DisplayPort: hiçbir aygıt PCIe erişimi almaz
tb-level-usbonly = Yalnızca USB: hiçbir aygıt PCIe erişimi almaz
tb-level-nopcie = PCIe devre dışı: hiçbir aygıt PCIe erişimi almaz
tb-dma = DMA koruması
tb-dma-on = Açık: aygıtlar kendilerine verilmeyen belleği okuyamaz
tb-dma-off = Kapalı: izin verilen bir aygıt herhangi bir belleği okuyabilir
tb-auto-off = bolt'ta otomatik yetkilendirme kapalı, bu yüzden hatırlanan aygıtlar da her seferinde izin bekler.
tb-never-asks = Bu güvenlik düzeyinde izin verilecek bir şey yok: aygıtlar hakkında hiç sorulmaz.
tb-has-problems = Thunderbolt kurulumunda bir sorun var.
tb-open-status = Nasıl düzeltileceğini göster
tb-prompt-heading = Bu Thunderbolt aygıtına izin verilsin mi?
tb-prompt-description = { $name } bağlı. Siz izin verene kadar ne bilgisayarın belleğine ne de PCIe veri yoluna erişebilir.
tb-devices = Aygıtlar
tb-no-devices = Hiç Thunderbolt veya USB4 aygıtı bağlanmadı.
tb-remembered = Hatırlanıyor
tb-not-remembered = Hatırlanmıyor
tb-status-waiting = İzin bekliyor
tb-status-authorized = İzin verildi
tb-status-error = Yetkilendirme başarısız
tb-status-working = Bağlanıyor…
tb-forget = Unut
tb-forget-note = Bağlı bir Thunderbolt aygıtı çıkarılana kadar erişimini korur; bunu daha önce geri almanın bir yolu yoktur. Bir aygıtı unutmak bir sonraki bağlanışta geçerli olur.
