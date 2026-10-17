# SPDX-License-Identifier: GPL-3.0-or-later
# German (Deutsch) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Prüfen und steuern, welche USB-Geräte sich verbinden dürfen

## Generic actions

allow = Zulassen
block = Blockieren
reject = Abweisen
revoke = Widerrufen
forget = Regel vergessen
details = Details
dismiss = Verwerfen
allow-once = Einmal erlauben
always-allow = Immer erlauben
refresh = Aktualisieren
copy = Kopieren
copy-command = Befehl kopieren
open-app = USB Guard öffnen
quit = Beenden

## Navigation

page-devices = Geräte
page-thunderbolt = Thunderbolt
page-history = Verlauf
page-status = Status
page-settings = Einstellungen

## Device list

devices-heading = Verbundene Geräte
devices-none = Keine USB-Geräte
devices-none-description = Nichts ist angeschlossen, oder der USBGuard-Dienst kann den USB-Bus dieses Systems nicht sehen.
devices-hidden = { $count } { $count ->
        [one] Gerät
       *[other] Geräte
    } durch Ihre Anzeigeeinstellungen ausgeblendet
devices-pending = { $count } { $count ->
        [one] Gerät wartet
       *[other] Geräte warten
    } auf eine Entscheidung
devices-remembered = Nicht verbunden
devices-remembered-description = Geräte mit einer dauerhaften Regel, die nicht angeschlossen sind. Ändern oder entfernen Sie die Regel hier.
device-internal = Internes Gerät
device-internal-toggle = Internes Gerät
device-internal-description = Teil dieses Rechners. Wird aus der Liste ausgeblendet und nie nachgefragt. Das erlaubt es nicht — lassen Sie es zusätzlich zu, wenn es funktionieren soll.
device-internal-no-hash = Dieses Gerät meldet keinen Deskriptor-Hash und kann daher nicht als intern markiert werden.

## Device state

state-allowed = Zugelassen
state-blocked = Blockiert
state-rejected = Abgewiesen
state-unknown = Unbekannt
state-pending = Entscheidung ausstehend
state-disconnected = Nicht verbunden

## Standing policy rules

standing-allow = Immer zugelassen
standing-block = Immer blockiert
standing-reject = Immer abgewiesen
standing-other = Dauerhafte Regel: { $target }
bound-port-toggle = An USB-Anschluss gebunden
bound-port-description = Die Regel gilt nur, solange das Gerät an Anschluss { $port } steckt. Überall sonst wird es blockiert und Sie werden erneut gefragt.
bound-port-description-unbound = Die Regel nur an dem Anschluss gelten lassen, an dem das Gerät jetzt steckt. Ein Gerät, das seine Identität nachahmt, müsste dann ebenfalls an diesem Anschluss stecken.
bound-port-unplugged = Schließen Sie das Gerät an, um seine Regel an den aktuellen Anschluss zu binden.

## Device fields

field-name = Name
field-usb-id = USB-ID
field-serial = Seriennummer
field-port = Anschluss
field-hash = Deskriptor-Hash
field-interfaces = Schnittstellen
field-connection = Verbindung
field-status = Status
field-none = Nicht gemeldet

## Decision prompt

prompt-heading = Dieses Gerät zulassen?
prompt-description = { $name } wurde gerade angeschlossen. Es bleibt blockiert, bis Sie entscheiden.
notify-new-device = Neues USB-Gerät angeschlossen
notify-auto-blocked = USB-Gerät abgewiesen
notify-auto-blocked-body = { $name } wurde durch eine dauerhafte Regel abgewiesen und ist nicht verfügbar. Öffnen Sie USB Guard, um das zu ändern.
notify-manage = Gerät verwalten
notify-thunderbolt = Thunderbolt-Gerät wartet
notify-thunderbolt-body = { $name } ist verbunden, kann aber erst auf den Speicher des Computers zugreifen, wenn Sie es erlauben.
remember-decision = Diese Entscheidung merken
bind-to-port = An diesen USB-Anschluss binden ({ $port })
bind-to-port-description = Gilt nur an diesem Anschluss. An jedem anderen Anschluss wird das Gerät blockiert und Sie werden erneut gefragt.
prompt-wrong-port = Seine Regel ist an Anschluss { $bound } gebunden, es steckt aber jetzt an Anschluss { $port }.

## Warnings

warning-input-capable = Dieses Gerät kann sich als Tastatur ausgeben und in Ihrem Namen tippen.
warning-storage = Dieses Gerät stellt Speicher bereit.
warning-network = Dieses Gerät stellt einen Netzwerkadapter bereit, der Ihren Datenverkehr umleiten kann.
warning-standing-conflict = Eine dauerhafte Regel setzt dies beim nächsten Anschließen wieder auf „{ $target }“.
warning-overriding-rule = Eine allgemeinere Regel weiter oben in der Richtlinie passt ebenfalls zu diesem Gerät und wird zuerst geprüft, sodass sie statt der eigenen Regel des Geräts entscheiden kann: { $rule }
warning-no-hash = Der USBGuard-Dienst hat für dieses Gerät keinen Deskriptor-Hash gemeldet, daher kann keine dauerhafte Regel gezielt an dieses Gerät gebunden werden.

## Status and health

status-ok = Dieses System ist geschützt
status-warning = Beim Schutz gibt es Probleme
status-critical = Dieses System ist nicht geschützt
status-disconnected = Nicht mit dem USBGuard-Dienst verbunden
status-disconnected-description = { $reason }
status-checking = Wird geprüft …

check-daemon-running = Der USBGuard-Dienst läuft
check-daemon-enabled = Der USBGuard-Dienst startet beim Hochfahren
check-dbus-running = Die D-Bus-Schnittstelle von USBGuard läuft
check-dbus-enabled = Die D-Bus-Schnittstelle von USBGuard startet beim Hochfahren
check-ipc-reachable = Der Dienst beantwortet Anfragen
check-ipc-permission = Sie dürfen Richtlinienentscheidungen treffen
check-decisions-reversible = Sie dürfen eine dauerhafte Entscheidung rückgängig machen
check-inserted-policy = Neue Geräte warten auf eine Entscheidung
check-policy-not-empty = Eine Geräterichtlinie ist eingerichtet
check-thunderbolt-security = Thunderbolt-Geräte müssen vor dem Verbinden erlaubt werden
check-thunderbolt-dma = Der Speicher ist vor Thunderbolt-Geräten geschützt
check-thunderbolt-service = Der Thunderbolt-Dienst (bolt) läuft
check-thunderbolt-permission = Sie dürfen Thunderbolt-Geräte erlauben
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt und USB4

check-observed = Beobachtet: { $value }
observed-unknown = unbekannt
observed-no-bus = keine Verbindung zum Systembus
observed-responding = antwortet
observed-granted = erteilt
observed-password = erfordert ein Administratorpasswort
observed-refused = abgelehnt
observed-polkit-unavailable = polkit nicht verfügbar
observed-rules = Regeln in der Richtlinie: { $count }
observed-unreadable = konnte nicht gelesen werden
observed-running = läuft
observed-not-installed = nicht installiert
observed-stopped = installiert, aber nicht gestartet
observed-on = an
observed-off = aus
observed-not-reported = nicht gemeldet
remedy-heading = Beheben Sie es mit:
remedy-explanation = Kopieren Sie den Befehl, fügen Sie ihn in ein Terminal ein und drücken Sie die Eingabetaste. Er fragt nach Ihrem Passwort und gibt bei Erfolg nichts aus. Drücken Sie danach auf Aktualisieren, um zu prüfen, ob es funktioniert hat.
remedy-policy-warning = Zuerst lesen: Damit wird jedes Gerät erlaubt, das gerade angeschlossen ist. Ziehen Sie alles ab, was Sie nicht kennen, bevor Sie es ausführen.
steps-heading = So beheben Sie es im Firmware-Setup:
steps-firmware-only = Diese Einstellung ist in der Firmware des Computers gespeichert, daher kann kein Befehl sie ändern.
steps-restart = Speichern Sie Ihre Arbeit und starten Sie den Computer neu.
steps-open-setup = Drücken Sie beim Start die Taste, die das Firmware-Setup öffnet. Sie wird meist auf dem Bildschirm angezeigt; üblich sind F1 oder Enter (Lenovo), F2 (Dell, ASUS, Acer und die meisten anderen), F10 (HP) und Entf (Desktop-Mainboards).
steps-tb-find = Suchen Sie die Thunderbolt- oder USB4-Einstellungen, meist unter Security, Config oder Advanced.
steps-tb-set = Stellen Sie die Sicherheitsstufe auf „User Authorization“ oder „Secure Connect“. Vermeiden Sie „No Security“ und „Legacy“.
steps-dma-set = Schalten Sie „Kernel DMA Protection“ ein oder die Option, von der sie abhängt: „VT-d“ bei Intel, „AMD-Vi“ oder „IOMMU“ bei AMD. Meist unter Security, Virtualization oder Advanced.
steps-save = Speichern Sie die Änderungen, beenden Sie das Setup und lassen Sie den Computer starten. Drücken Sie dann auf dieser Seite auf Aktualisieren.
steps-session-only = Polkit erlaubt USBGuard nur der Person am Computer: der aktiven, lokalen Sitzung. Eine Sitzung über SSH oder Remotedesktop sowie eine, von der weggewechselt wurde, wird abgelehnt.
steps-session-heading = So beheben Sie es:
steps-session-local = Führen Sie USB Guard in der Sitzung direkt am Computer aus.
steps-session-switch = Wenn Sie den Benutzer gewechselt haben, wechseln Sie zu dieser Sitzung zurück.
steps-session-refresh = Drücken Sie auf dieser Seite auf Aktualisieren.

## History

history-heading = Entscheidungsverlauf
history-empty = Noch nichts aufgezeichnet
history-empty-description = Geräteereignisse und die dazu getroffenen Entscheidungen erscheinen hier.
history-clear = Verlauf löschen
history-entries = { $count } { $count ->
        [one] Eintrag
       *[other] Einträge
    }
history-filter-all = Alle Ereignisse
history-filter-decisions = Nur Entscheidungen

event-inserted = Angeschlossen
event-removed = Getrennt
event-updated = Geändert
event-allowed = Zugelassen
event-blocked = Blockiert
event-rejected = Abgewiesen
event-revoked = Widerrufen
event-service-up = Der USBGuard-Dienst wurde verfügbar
event-service-down = Der USBGuard-Dienst wurde unverfügbar
event-health-problem = Konfigurationsproblem

actor-user = von Ihnen
actor-policy = durch die Regeln des USBGuard-Dienstes
actor-external = außerhalb dieser Anwendung
actor-system = automatisch

## Hooks

hook-heading = Programm beim Verbinden ausführen
hook-description = Läuft erst, nachdem dieses Gerät zugelassen wurde. Ein blockiertes Gerät führt niemals etwas aus.
hook-program = Programm
hook-program-placeholder = /home/sie/bin/backup.sh
hook-arguments = Argumente
hook-arguments-placeholder = Eines pro Zeile
hook-label = Name
hook-label-placeholder = Sicherung
hook-enabled = Aktiviert
hook-save = Aktion speichern
hook-remove = Aktion entfernen
hook-none = Kein Programm festgelegt
hook-problem-not-set = Wählen Sie ein Programm zum Ausführen.
hook-problem-not-absolute = Geben Sie den vollständigen Pfad an, beginnend mit einem Schrägstrich.
hook-problem-missing = Diese Datei existiert nicht.
hook-problem-not-executable = Diese Datei ist nicht ausführbar. Führen Sie aus: chmod +x
hook-variables = Das Programm erhält die Gerätedaten als Umgebungsvariablen: { $names }.

## Settings

setting-prompt-on-insert = Bei neuen Geräten nachfragen
setting-prompt-on-insert-description = Eine Abfrage anzeigen, wenn ein Gerät ohne dauerhafte Regel angeschlossen wird.
setting-notify-on-insert = Desktop-Benachrichtigung senden
setting-notify-on-insert-description = Zusätzlich benachrichtigen, damit keine Abfrage übersehen wird, wenn das Panel verborgen ist.
setting-auto-open-popup = Fenster automatisch öffnen
setting-auto-open-popup-description = USB Guard sein Fenster selbst öffnen lassen, wenn ein Gerät eine Entscheidung braucht.
setting-default-permanent = Entscheidungen standardmäßig merken
setting-default-permanent-description = „Diese Entscheidung merken“ in der Abfrage vorab ankreuzen.
setting-show-hardwired = Fest verlötete Geräte anzeigen
setting-show-hardwired-description = Geräte einbeziehen, die der USBGuard-Dienst als fest verbaut meldet und die sich nicht abziehen lassen.
setting-show-root-hubs = Root-Hubs anzeigen
setting-show-root-hubs-description = Die Host-Controller einbeziehen, an denen die USB-Anschlüsse hängen.
setting-show-internal = Interne Geräte anzeigen
setting-show-internal-description = Geräte einbeziehen, die Sie als Teil dieses Rechners markiert haben.
setting-show-disconnected = Getrennte Geräte anzeigen
setting-show-disconnected-description = Geräte auflisten, die eine dauerhafte Regel haben, aber nicht angeschlossen sind, damit eine Entscheidung auch ohne sie geändert werden kann.
setting-warn-input-capable = Tastaturfähige Geräte hervorheben
setting-warn-input-capable-description = Geräte kennzeichnen, die Tastenanschläge einschleusen könnten.
setting-journal-enabled = Entscheidungsverlauf führen
setting-journal-enabled-description = Geräteereignisse und Entscheidungen nach { $path } schreiben.
setting-warn-on-health-problems = Vor Konfigurationsproblemen warnen
setting-warn-on-health-problems-description = Einen Hinweis anzeigen, wenn dieses System nicht vollständig geschützt ist.
setting-notify-on-auto-block = Melden, wenn ein Gerät abgewiesen wird
setting-notify-on-auto-block-description = Benachrichtigen, wenn eine dauerhafte Regel ein Gerät ohne Nachfrage abweist, damit es nicht einfach stillschweigend nicht funktioniert.
setting-show-tray-icon = Statussymbol anzeigen
setting-show-tray-icon-description = Das Schild im Systemabschnitt der Kontrollleiste anzeigen. Das Ausschalten hält die Anwendung nicht davon ab, Geräte zu überwachen.
setting-autostart = Automatisch bei der Anmeldung starten
setting-autostart-description = USB Guard zur Sitzung hinzufügen, damit es schon überwacht, bevor Sie etwas anschließen.
setting-start-minimized = Ohne Fenster starten
setting-start-minimized-description = Bei der Anmeldung nur das Statussymbol anzeigen.
setting-run-in-background = Beim Schließen des Fensters weiterlaufen
setting-run-in-background-description = Das Schließen des Fensters lässt USB Guard weiter überwachen. Schalten Sie dies aus, beendet das Schließen die Anwendung — und damit auch die Abfragen.

section-behaviour = Verhalten
section-display = Anzeige
section-startup = Start und Statussymbol
section-privacy = Verlauf

## Errors

error-service-unavailable = Der USBGuard-Dienst läuft nicht
error-permission-denied = Nicht erlaubt
error-autostart = Die Autostart-Einstellung konnte nicht geändert werden: { $message }
error-no-tray = Dieser Desktop hat keinen Systemabschnitt, daher konnte das Statussymbol nicht angezeigt werden. USB Guard hat stattdessen sein Fenster geöffnet.
error-cannot-remove-rule = Das Entfernen einer dauerhaften Regel erfordert eine Administrator-Autorisierung, die nicht erteilt wurde. Siehe „Sie dürfen eine dauerhafte Entscheidung rückgängig machen“ auf der Statusseite.
error-thunderbolt-permission = Sie dürfen keine Thunderbolt-Geräte autorisieren. Die Statusseite bietet unter „Sie dürfen Thunderbolt-Geräte erlauben“ eine Lösung.
error-bolt-unavailable = Der Thunderbolt-Dienst (bolt) läuft nicht. Die Statusseite zeigt, wie Sie ihn starten.
error-thunderbolt = Der Thunderbolt-Dienst meldete einen Fehler: { $message }

## About

repository = Repository
support = Ein Problem melden
version = Version { $version }

## Thunderbolt page

tb-heading = Thunderbolt und USB4
tb-security = Sicherheitsstufe
tb-level-none = Keine: Jedes Gerät verbindet sich, ohne dass gefragt wird
tb-level-user = Benutzerautorisierung: Jedes neue Gerät muss erlaubt werden
tb-level-secure = Sichere Verbindung: Jedes neue Gerät muss erlaubt werden, und gespeicherte Geräte weisen sich mit einem Schlüssel aus
tb-level-dponly = Nur DisplayPort: Kein Gerät erhält PCIe-Zugriff
tb-level-usbonly = Nur USB: Kein Gerät erhält PCIe-Zugriff
tb-level-nopcie = PCIe deaktiviert: Kein Gerät erhält PCIe-Zugriff
tb-dma = DMA-Schutz
tb-dma-on = Ein: Geräte können keinen Speicher lesen, der ihnen nicht zugewiesen wurde
tb-dma-off = Aus: Ein erlaubtes Gerät kann beliebigen Speicher lesen
tb-auto-off = Die automatische Autorisierung ist in bolt ausgeschaltet, daher warten auch gespeicherte Geräte jedes Mal darauf, erlaubt zu werden.
tb-never-asks = Auf dieser Sicherheitsstufe gibt es nichts zu erlauben: Nach Geräten wird nie gefragt.
tb-has-problems = Es gibt ein Problem mit der Thunderbolt-Einrichtung.
tb-open-status = Lösung anzeigen
tb-prompt-heading = Dieses Thunderbolt-Gerät erlauben?
tb-prompt-description = { $name } ist verbunden. Bis Sie es erlauben, kann es weder auf den Speicher des Computers noch auf den PCIe-Bus zugreifen.
tb-devices = Geräte
tb-no-devices = Es wurden noch keine Thunderbolt- oder USB4-Geräte verbunden.
tb-remembered = Gespeichert
tb-not-remembered = Nicht gespeichert
tb-status-waiting = Wartet auf Erlaubnis
tb-status-authorized = Erlaubt
tb-status-error = Autorisierung fehlgeschlagen
tb-status-working = Verbindet…
tb-forget = Vergessen
tb-forget-note = Ein verbundenes Thunderbolt-Gerät behält seinen Zugriff, bis es abgezogen wird; er lässt sich nicht früher entziehen. Das Vergessen eines Geräts wirkt bei der nächsten Verbindung.
