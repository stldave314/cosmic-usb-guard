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
allow-once = Consenti una volta
always-allow = Consenti sempre
refresh = Aggiorna
copy = Copia
copy-command = Copia comando
open-app = Apri USB Guard
quit = Esci

## Navigation

page-devices = Dispositivi
page-thunderbolt = Thunderbolt
page-history = Cronologia
page-status = Stato
page-settings = Impostazioni

## Device list

devices-heading = Dispositivi collegati
devices-none = Nessun dispositivo USB
devices-none-description = Non è collegato nulla, oppure il servizio USBGuard non vede il bus USB di questo sistema.
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
bound-port-toggle = Vincolato a una porta USB
bound-port-description = La regola vale solo mentre il dispositivo è collegato alla porta { $port }. Altrove viene bloccato e ti verrà chiesto di nuovo.
bound-port-description-unbound = Fai valere la regola solo sulla porta a cui il dispositivo è collegato ora. Un dispositivo che ne copia l’identità dovrebbe trovarsi anch’esso su quella porta.
bound-port-unplugged = Collega il dispositivo per vincolarne la regola alla porta in cui si trova.

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
notify-thunderbolt = Dispositivo Thunderbolt in attesa
notify-thunderbolt-body = { $name } è collegato, ma non può accedere alla memoria del computer finché non lo consenti.
remember-decision = Ricorda questa decisione
bind-to-port = Vincola a questa porta USB ({ $port })
bind-to-port-description = Vale solo su questa porta. Collegato altrove, il dispositivo viene bloccato e ti verrà chiesto di nuovo.
prompt-wrong-port = La sua regola è vincolata alla porta { $bound }, ma ora è collegato alla porta { $port }.

## Warnings

warning-input-capable = Questo dispositivo può comportarsi come una tastiera e digitare per tuo conto.
warning-storage = Questo dispositivo espone spazio di archiviazione.
warning-network = Questo dispositivo presenta una scheda di rete, che può dirottare il tuo traffico.
warning-standing-conflict = Una regola permanente lo riporterà a «{ $target }» al prossimo collegamento.
warning-overriding-rule = Una regola più ampia, che compare prima nella politica, corrisponde anch’essa a questo dispositivo e viene controllata per prima, quindi può decidere al posto della regola propria del dispositivo: { $rule }
warning-no-hash = Il servizio USBGuard non ha fornito un hash del descrittore per questo dispositivo, quindi non è possibile vincolargli specificamente una regola permanente.

## Status and health

status-ok = Questo sistema è protetto
status-warning = La protezione ha dei problemi
status-critical = Questo sistema non è protetto
status-disconnected = Non connesso al servizio USBGuard
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
check-thunderbolt-security = I dispositivi Thunderbolt devono essere consentiti prima di collegarsi
check-thunderbolt-dma = La memoria è protetta dai dispositivi Thunderbolt
check-thunderbolt-service = Il servizio Thunderbolt (bolt) è in esecuzione
check-thunderbolt-permission = Puoi consentire i dispositivi Thunderbolt
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt e USB4

check-observed = Osservato: { $value }
observed-unknown = sconosciuto
observed-no-bus = nessuna connessione al bus di sistema
observed-responding = risponde
observed-granted = concesso
observed-password = richiede una password di amministratore
observed-asks-password = chiede una password, il che va bene
observed-refused = rifiutato
observed-polkit-unavailable = polkit non disponibile
observed-rules = regole nella politica: { $count }
observed-unreadable = non leggibile
observed-running = in esecuzione
observed-not-installed = non installato
observed-stopped = installato ma non in esecuzione
observed-on = attiva
observed-off = disattivata
observed-not-reported = non indicato
remedy-heading = Correggi eseguendo:
remedy-explanation = Copia il comando, incollalo in un terminale e premi Invio. Chiede la tua password e non stampa nulla se riesce. Poi premi Aggiorna per verificare che abbia funzionato.
remedy-policy-warning = Leggi prima: consente ogni dispositivo collegato in questo momento. Scollega tutto ciò che non riconosci prima di eseguirlo.
steps-heading = Correggilo nella configurazione del firmware:
steps-firmware-only = Questa impostazione è memorizzata nel firmware del computer, quindi nessun comando può cambiarla.
steps-restart = Salva il lavoro e riavvia il computer.
steps-open-setup = Durante l’avvio, premi il tasto che apre la configurazione del firmware. Di solito è indicato sullo schermo; i più comuni sono F1 o Invio (Lenovo), F2 (Dell, ASUS, Acer e la maggior parte degli altri), F10 (HP) e Canc (schede madri desktop).
steps-tb-find = Trova le impostazioni Thunderbolt o USB4, di solito in Security, Config o Advanced.
steps-tb-set = Imposta il livello di sicurezza su «User Authorization» o «Secure Connect». Evita «No Security» e «Legacy».
steps-dma-set = Attiva «Kernel DMA Protection» o l’opzione da cui dipende: «VT-d» su Intel, «AMD-Vi» o «IOMMU» su AMD. Di solito si trova in Security, Virtualization o Advanced.
steps-save = Salva le modifiche, esci e lascia avviare il computer. Poi premi Aggiorna in questa pagina.
steps-session-only = Polkit consente di usare USBGuard solo a chi è davanti al computer: la sessione locale attiva. Rifiuta una sessione via SSH o desktop remoto e una da cui si è passati a un altro utente.
steps-session-heading = Per correggerlo:
steps-session-local = Esegui USB Guard nella sessione aperta sul computer stesso.
steps-session-switch = Se hai cambiato utente, torna a questa sessione.
steps-session-refresh = Premi Aggiorna in questa pagina.

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
event-service-up = Il servizio USBGuard è diventato disponibile
event-service-down = Il servizio USBGuard è diventato non disponibile
event-health-problem = Problema di configurazione

actor-user = da te
actor-policy = dalle regole del servizio USBGuard
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
setting-show-hardwired-description = Includi i dispositivi che il servizio USBGuard segnala come integrati, che non si possono scollegare.
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
setting-warn-on-health-problems-description = Mostra un avviso quando questo sistema non è del tutto protetto.
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

error-service-unavailable = Il servizio USBGuard non è in esecuzione
error-permission-denied = Non consentito
error-autostart = Impossibile modificare l'impostazione di avvio automatico: { $message }
error-no-tray = Questo desktop non ha un'area di notifica, quindi non è stato possibile mostrare l'icona di stato. USB Guard ha aperto la sua finestra.
error-cannot-remove-rule = La rimozione di una regola permanente richiede un'autorizzazione da amministratore, che non è stata concessa. Vedi «Puoi annullare una decisione permanente» nella pagina Stato.
error-thunderbolt-permission = Non hai il permesso di autorizzare dispositivi Thunderbolt da questa sessione. La pagina Stato spiega il motivo in «Puoi consentire i dispositivi Thunderbolt».
error-bolt-unavailable = Il servizio Thunderbolt (bolt) non è in esecuzione. La pagina Stato mostra come avviarlo.
error-thunderbolt = Il servizio Thunderbolt ha segnalato un errore: { $message }

## About

repository = Repository
support = Segnala un problema
version = Versione { $version }

## Thunderbolt page

tb-heading = Thunderbolt e USB4
tb-security = Livello di sicurezza
tb-level-none = Nessuno: ogni dispositivo si collega senza che venga chiesto
tb-level-user = Autorizzazione dell’utente: ogni nuovo dispositivo deve essere consentito
tb-level-secure = Connessione sicura: ogni nuovo dispositivo deve essere consentito e quelli memorizzati dimostrano la propria identità con una chiave
tb-level-dponly = Solo DisplayPort: nessun dispositivo ottiene accesso PCIe
tb-level-usbonly = Solo USB: nessun dispositivo ottiene accesso PCIe
tb-level-nopcie = PCIe disattivato: nessun dispositivo ottiene accesso PCIe
tb-dma = Protezione DMA
tb-dma-on = Attiva: i dispositivi non possono leggere memoria che non è stata loro assegnata
tb-dma-off = Disattivata: un dispositivo consentito può leggere qualsiasi memoria
tb-auto-off = L’autorizzazione automatica è disattivata in bolt, quindi anche i dispositivi memorizzati attendono ogni volta di essere consentiti.
tb-never-asks = A questo livello di sicurezza non c’è nulla da consentire: non viene mai chiesto nulla.
tb-has-problems = C’è un problema con la configurazione Thunderbolt.
tb-open-status = Mostra come correggere
tb-prompt-heading = Consentire questo dispositivo Thunderbolt?
tb-prompt-description = { $name } è collegato. Finché non lo consenti, non può accedere alla memoria del computer né al bus PCIe.
tb-devices = Dispositivi
tb-no-devices = Non è stato collegato alcun dispositivo Thunderbolt o USB4.
tb-remembered = Memorizzato
tb-not-remembered = Non memorizzato
tb-status-waiting = In attesa di consenso
tb-status-authorized = Consentito
tb-status-error = Autorizzazione non riuscita
tb-status-working = Connessione…
tb-forget = Dimentica
tb-forget-note = Un dispositivo Thunderbolt collegato mantiene l’accesso finché non viene scollegato; non c’è modo di revocarlo prima. Dimenticare un dispositivo ha effetto al prossimo collegamento.
