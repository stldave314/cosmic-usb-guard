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
refresh = Aktualisieren
copy = Kopieren
open-app = USB Guard öffnen
quit = Beenden

## Navigation

page-devices = Geräte
page-history = Verlauf
page-status = Status
page-settings = Einstellungen

## Device list

devices-heading = Verbundene Geräte
devices-none = Keine USB-Geräte
devices-none-description = Es ist nichts angeschlossen, oder USBGuard sieht den USB-Bus dieses Systems nicht.
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
remember-decision = Diese Entscheidung merken

## Warnings

warning-input-capable = Dieses Gerät kann sich als Tastatur ausgeben und in Ihrem Namen tippen.
warning-storage = Dieses Gerät stellt Speicher bereit.
warning-network = Dieses Gerät stellt einen Netzwerkadapter bereit, der Ihren Datenverkehr umleiten kann.
warning-standing-conflict = Eine dauerhafte Regel setzt dies beim nächsten Anschließen wieder auf „{ $target }“.
warning-no-hash = USBGuard hat für dieses Gerät keinen Deskriptor-Hash gemeldet, daher kann keine dauerhafte Regel gezielt daran gebunden werden.

## Status and health

status-ok = USBGuard schützt dieses System
status-warning = USBGuard läuft mit Problemen
status-critical = Dieses System ist nicht geschützt
status-disconnected = Nicht mit USBGuard verbunden
status-disconnected-description = { $reason }
status-checking = Wird geprüft …

check-daemon-running = USBGuard-Dienst läuft
check-daemon-enabled = USBGuard-Dienst startet beim Systemstart
check-dbus-running = USBGuard-D-Bus-Schnittstelle läuft
check-dbus-enabled = USBGuard-D-Bus-Schnittstelle startet beim Systemstart
check-ipc-reachable = Der Dienst beantwortet Anfragen
check-ipc-permission = Sie dürfen Richtlinienentscheidungen treffen
check-decisions-reversible = Sie dürfen eine dauerhafte Entscheidung rückgängig machen
check-inserted-policy = Neue Geräte warten auf eine Entscheidung
check-policy-not-empty = Eine Geräterichtlinie ist eingerichtet

check-observed = Beobachtet: { $value }
remedy-heading = Beheben Sie es mit:

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
event-service-up = USBGuard wurde verfügbar
event-service-down = USBGuard wurde nicht mehr verfügbar
event-health-problem = Konfigurationsproblem

actor-user = von Ihnen
actor-policy = durch USBGuard-Richtlinie
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
setting-show-hardwired-description = Geräte einbeziehen, die USBGuard als fest verdrahtet meldet und die nicht abgezogen werden können.
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
setting-warn-on-health-problems-description = Einen Hinweis anzeigen, wenn USBGuard nicht zum Schutz dieses Systems eingerichtet ist.
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

error-service-unavailable = USBGuard läuft nicht
error-permission-denied = Nicht erlaubt
error-autostart = Die Autostart-Einstellung konnte nicht geändert werden: { $message }
error-no-tray = Dieser Desktop hat keinen Systemabschnitt, daher konnte das Statussymbol nicht angezeigt werden. USB Guard hat stattdessen sein Fenster geöffnet.
error-cannot-remove-rule = Das Entfernen einer dauerhaften Regel erfordert eine Administrator-Autorisierung, die nicht erteilt wurde. Siehe „Sie dürfen eine dauerhafte Entscheidung rückgängig machen“ auf der Statusseite.

## About

repository = Repository
support = Ein Problem melden
version = Version { $version }
