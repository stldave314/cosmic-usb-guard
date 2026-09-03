# SPDX-License-Identifier: GPL-3.0-or-later
# Italian (Italiano) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Esamina e controlla quali dispositivi USB possono connettersi

## Generic actions

allow = Consenti
block = Blocca
reject = Rifiuta
revoke = Revoca
forget = Dimentica regola
details = Dettagli
dismiss = Ignora
refresh = Aggiorna
copy = Copia
open-app = Apri USB Guard
quit = Esci

## Navigation

page-devices = Dispositivi
page-history = Cronologia
page-status = Stato
page-settings = Impostazioni

## Device list

devices-heading = Dispositivi collegati
devices-none = Nessun dispositivo USB
devices-none-description = Non è collegato nulla, oppure USBGuard non vede il bus USB di questo sistema.
devices-hidden = { $count } { $count ->
        [one] dispositivo nascosto
       *[other] dispositivi nascosti
    } dalle impostazioni di visualizzazione
devices-pending = { $count } { $count ->
        [one] dispositivo attende
       *[other] dispositivi attendono
    } una decisione
devices-remembered = Non collegati
devices-remembered-description = Dispositivi con una regola permanente che non sono collegati. Modifica o rimuovi qui la regola.
device-internal = Dispositivo interno
device-internal-toggle = Dispositivo interno
device-internal-description = Fa parte di questa macchina. Viene nascosto dall'elenco e non viene mai chiesto. Questo non lo autorizza: consentilo anche, se deve funzionare.
device-internal-no-hash = Questo dispositivo non riporta un hash del descrittore, quindi non può essere contrassegnato come interno.

## Device state

state-allowed = Consentito
state-blocked = Bloccato
state-rejected = Rifiutato
state-unknown = Sconosciuto
state-pending = In attesa di decisione
state-disconnected = Non collegato

## Standing policy rules

standing-allow = Sempre consentito
standing-block = Sempre bloccato
standing-reject = Sempre rifiutato
standing-other = Regola permanente: { $target }

## Device fields

field-name = Nome
field-usb-id = ID USB
field-serial = Numero di serie
field-port = Porta
field-hash = Hash del descrittore
field-interfaces = Interfacce
field-connection = Connessione
field-status = Stato
field-none = Non riportato

## Decision prompt

prompt-heading = Consentire questo dispositivo?
prompt-description = { $name } è stato appena collegato. Resta bloccato finché non decidi.
notify-new-device = Nuovo dispositivo USB collegato
notify-auto-blocked = Dispositivo USB rifiutato
notify-auto-blocked-body = { $name } è stato rifiutato da una regola permanente e non è disponibile. Apri USB Guard per cambiarlo.
notify-manage = Gestisci dispositivo
remember-decision = Ricorda questa decisione

## Warnings

warning-input-capable = Questo dispositivo può comportarsi come una tastiera e digitare per tuo conto.
warning-storage = Questo dispositivo espone spazio di archiviazione.
warning-network = Questo dispositivo presenta una scheda di rete, che può dirottare il tuo traffico.
warning-standing-conflict = Una regola permanente lo riporterà a «{ $target }» al prossimo collegamento.
warning-no-hash = USBGuard non ha riportato un hash del descrittore per questo dispositivo, quindi una regola permanente non può essere ancorata specificamente a esso.

## Status and health

status-ok = USBGuard sta proteggendo questo sistema
status-warning = USBGuard è in esecuzione con problemi
status-critical = Questo sistema non è protetto
status-disconnected = Non connesso a USBGuard
status-disconnected-description = { $reason }
status-checking = Verifica in corso…

check-daemon-running = Il servizio USBGuard è in esecuzione
check-daemon-enabled = Il servizio USBGuard si avvia all'accensione
check-dbus-running = L'interfaccia D-Bus di USBGuard è in esecuzione
check-dbus-enabled = L'interfaccia D-Bus di USBGuard si avvia all'accensione
check-ipc-reachable = Il servizio risponde alle richieste
check-ipc-permission = Puoi prendere decisioni sui criteri
check-decisions-reversible = Puoi annullare una decisione permanente
check-inserted-policy = I nuovi dispositivi attendono una decisione
check-policy-not-empty = È configurato un criterio per i dispositivi

check-observed = Osservato: { $value }
remedy-heading = Correggi eseguendo:

## History

history-heading = Cronologia delle decisioni
history-empty = Nulla di registrato finora
history-empty-description = Gli eventi dei dispositivi e le decisioni prese appariranno qui.
history-clear = Cancella cronologia
history-entries = { $count } { $count ->
        [one] voce
       *[other] voci
    }
history-filter-all = Tutti gli eventi
history-filter-decisions = Solo decisioni

event-inserted = Collegato
event-removed = Scollegato
event-updated = Modificato
event-allowed = Consentito
event-blocked = Bloccato
event-rejected = Rifiutato
event-revoked = Revocato
event-service-up = USBGuard è diventato disponibile
event-service-down = USBGuard non è più disponibile
event-health-problem = Problema di configurazione

actor-user = da te
actor-policy = dal criterio di USBGuard
actor-external = fuori da questa applicazione
actor-system = automatico

## Hooks

hook-heading = Esegui un programma al collegamento
hook-description = Viene eseguito solo dopo che questo dispositivo è stato consentito. Un dispositivo bloccato non esegue mai nulla.
hook-program = Programma
hook-program-placeholder = /home/tu/bin/backup.sh
hook-arguments = Argomenti
hook-arguments-placeholder = Uno per riga
hook-label = Nome
hook-label-placeholder = Backup
hook-enabled = Attivo
hook-save = Salva azione
hook-remove = Rimuovi azione
hook-none = Nessun programma impostato
hook-problem-not-set = Scegli un programma da eseguire.
hook-problem-not-absolute = Inserisci il percorso completo, che inizi con una barra.
hook-problem-missing = Quel file non esiste.
hook-problem-not-executable = Quel file non è eseguibile. Esegui: chmod +x
hook-variables = Il programma riceve i dettagli del dispositivo come variabili d'ambiente: { $names }.

## Settings

setting-prompt-on-insert = Chiedi per i nuovi dispositivi
setting-prompt-on-insert-description = Mostra una richiesta di decisione quando viene collegato un dispositivo senza regola permanente.
setting-notify-on-insert = Invia una notifica di sistema
setting-notify-on-insert-description = Notifica anche, così una richiesta non passa inosservata quando il pannello è nascosto.
setting-auto-open-popup = Apri la finestra automaticamente
setting-auto-open-popup-description = Lascia che USB Guard apra la propria finestra quando un dispositivo richiede una decisione.
setting-default-permanent = Ricorda le decisioni per impostazione predefinita
setting-default-permanent-description = Spunta in anticipo «Ricorda questa decisione» nella richiesta.
setting-show-hardwired = Mostra i dispositivi saldati
setting-show-hardwired-description = Includi i dispositivi che USBGuard riporta come cablati e che non possono essere scollegati.
setting-show-root-hubs = Mostra gli hub radice
setting-show-root-hubs-description = Includi i controller host a cui sono collegate le porte USB.
setting-show-internal = Mostra i dispositivi interni
setting-show-internal-description = Includi i dispositivi che hai contrassegnato come parte di questa macchina.
setting-show-disconnected = Mostra i dispositivi scollegati
setting-show-disconnected-description = Elenca i dispositivi che hanno una regola permanente ma non sono collegati, così una decisione può essere modificata anche senza di essi.
setting-warn-input-capable = Evidenzia i dispositivi che possono fare da tastiera
setting-warn-input-capable-description = Segnala i dispositivi che potrebbero iniettare sequenze di tasti.
setting-journal-enabled = Mantieni una cronologia delle decisioni
setting-journal-enabled-description = Registra eventi e decisioni in { $path }.
setting-warn-on-health-problems = Avvisa sui problemi di configurazione
setting-warn-on-health-problems-description = Mostra un avviso quando USBGuard non è configurato per proteggere questo sistema.
setting-notify-on-auto-block = Avvisami quando un dispositivo viene rifiutato
setting-notify-on-auto-block-description = Notifica quando una regola permanente rifiuta un dispositivo senza chiedere, così non si limita a non funzionare in silenzio.
setting-show-tray-icon = Mostra l'icona di stato
setting-show-tray-icon-description = Mostra lo scudo nell'area di notifica. Disattivarlo non impedisce all'applicazione di sorvegliare i dispositivi.
setting-autostart = Avvia automaticamente all'accesso
setting-autostart-description = Aggiungi USB Guard alla sessione così sorveglia prima che tu colleghi qualcosa.
setting-start-minimized = Avvia senza aprire la finestra
setting-start-minimized-description = All'accesso, mostra solo l'icona di stato.
setting-run-in-background = Continua a funzionare quando la finestra è chiusa
setting-run-in-background-description = Chiudere la finestra lascia USB Guard in sorveglianza. Disattiva questa opzione e la chiusura termina l'applicazione, il che ferma anche le richieste.

section-behaviour = Comportamento
section-display = Visualizzazione
section-startup = Avvio e icona di stato
section-privacy = Cronologia

## Errors

error-service-unavailable = USBGuard non è in esecuzione
error-permission-denied = Non consentito
error-autostart = Impossibile modificare l'impostazione di avvio automatico: { $message }
error-no-tray = Questo desktop non ha un'area di notifica, quindi non è stato possibile mostrare l'icona di stato. USB Guard ha aperto la sua finestra.
error-cannot-remove-rule = La rimozione di una regola permanente richiede un'autorizzazione da amministratore, che non è stata concessa. Vedi «Puoi annullare una decisione permanente» nella pagina Stato.

## About

repository = Repository
support = Segnala un problema
version = Versione { $version }
