# SPDX-License-Identifier: GPL-3.0-or-later
# French (Français) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Examiner et contrôler quels périphériques USB peuvent se connecter

## Generic actions

allow = Autoriser
block = Bloquer
reject = Rejeter
revoke = Révoquer
forget = Oublier la règle
details = Détails
dismiss = Ignorer
refresh = Actualiser
copy = Copier
open-app = Ouvrir USB Guard
quit = Quitter

## Navigation

page-devices = Périphériques
page-history = Historique
page-status = État
page-settings = Paramètres

## Device list

devices-heading = Périphériques connectés
devices-none = Aucun périphérique USB
devices-none-description = Rien n'est connecté, ou USBGuard ne voit pas le bus USB de ce système.
devices-hidden = { $count } { $count ->
        [one] périphérique masqué
       *[other] périphériques masqués
    } par vos paramètres d'affichage
devices-pending = { $count } { $count ->
        [one] périphérique attend
       *[other] périphériques attendent
    } une décision
devices-remembered = Non connectés
devices-remembered-description = Périphériques ayant une règle permanente mais non branchés. Modifiez ou supprimez la règle ici.
device-internal = Périphérique interne
device-internal-toggle = Périphérique interne
device-internal-description = Fait partie de cette machine. Masqué de la liste et jamais soumis à une question. Cela ne l'autorise pas — autorisez-le aussi s'il doit fonctionner.
device-internal-no-hash = Ce périphérique ne fournit aucune empreinte de descripteur ; il ne peut donc pas être marqué comme interne.

## Device state

state-allowed = Autorisé
state-blocked = Bloqué
state-rejected = Rejeté
state-unknown = Inconnu
state-pending = Décision en attente
state-disconnected = Non connecté

## Standing policy rules

standing-allow = Toujours autorisé
standing-block = Toujours bloqué
standing-reject = Toujours rejeté
standing-other = Règle permanente : { $target }

## Device fields

field-name = Nom
field-usb-id = Identifiant USB
field-serial = Numéro de série
field-port = Port
field-hash = Empreinte du descripteur
field-interfaces = Interfaces
field-connection = Connexion
field-status = État
field-none = Non communiqué

## Decision prompt

prompt-heading = Autoriser ce périphérique ?
prompt-description = { $name } vient d'être connecté. Il reste bloqué jusqu'à votre décision.
notify-new-device = Nouveau périphérique USB connecté
notify-auto-blocked = Périphérique USB refusé
notify-auto-blocked-body = { $name } a été refusé par une règle permanente et n'est pas disponible. Ouvrez USB Guard pour changer cela.
notify-manage = Gérer le périphérique
remember-decision = Mémoriser cette décision

## Warnings

warning-input-capable = Ce périphérique peut se faire passer pour un clavier et saisir du texte en votre nom.
warning-storage = Ce périphérique expose un stockage.
warning-network = Ce périphérique présente un adaptateur réseau, capable de détourner votre trafic.
warning-standing-conflict = Une règle permanente rétablira « { $target } » à la prochaine connexion.
warning-no-hash = USBGuard n'a communiqué aucune empreinte de descripteur pour ce périphérique ; une règle permanente ne peut donc pas lui être rattachée précisément.

## Status and health

status-ok = USBGuard protège ce système
status-warning = USBGuard fonctionne avec des problèmes
status-critical = Ce système n'est pas protégé
status-disconnected = Non connecté à USBGuard
status-disconnected-description = { $reason }
status-checking = Vérification…

check-daemon-running = Le service USBGuard est actif
check-daemon-enabled = Le service USBGuard démarre au démarrage
check-dbus-running = L'interface D-Bus d'USBGuard est active
check-dbus-enabled = L'interface D-Bus d'USBGuard démarre au démarrage
check-ipc-reachable = Le service répond aux requêtes
check-ipc-permission = Vous pouvez prendre des décisions de politique
check-decisions-reversible = Vous pouvez annuler une décision permanente
check-inserted-policy = Les nouveaux périphériques attendent une décision
check-policy-not-empty = Une politique de périphériques est configurée

check-observed = Observé : { $value }
remedy-heading = Corrigez-le en exécutant :

## History

history-heading = Historique des décisions
history-empty = Rien d'enregistré pour l'instant
history-empty-description = Les événements de périphériques et les décisions prises à leur sujet apparaîtront ici.
history-clear = Effacer l'historique
history-entries = { $count } { $count ->
        [one] entrée
       *[other] entrées
    }
history-filter-all = Tous les événements
history-filter-decisions = Décisions uniquement

event-inserted = Connecté
event-removed = Déconnecté
event-updated = Modifié
event-allowed = Autorisé
event-blocked = Bloqué
event-rejected = Rejeté
event-revoked = Révoqué
event-service-up = USBGuard est devenu disponible
event-service-down = USBGuard est devenu indisponible
event-health-problem = Problème de configuration

actor-user = par vous
actor-policy = par la politique USBGuard
actor-external = hors de cette application
actor-system = automatique

## Hooks

hook-heading = Exécuter un programme à la connexion
hook-description = Ne s'exécute qu'après l'autorisation de ce périphérique. Un périphérique bloqué n'exécute jamais rien.
hook-program = Programme
hook-program-placeholder = /home/vous/bin/sauvegarde.sh
hook-arguments = Arguments
hook-arguments-placeholder = Un par ligne
hook-label = Nom
hook-label-placeholder = Sauvegarde
hook-enabled = Activé
hook-save = Enregistrer l'action
hook-remove = Supprimer l'action
hook-none = Aucun programme défini
hook-problem-not-set = Choisissez un programme à exécuter.
hook-problem-not-absolute = Saisissez le chemin complet, commençant par une barre oblique.
hook-problem-missing = Ce fichier n'existe pas.
hook-problem-not-executable = Ce fichier n'est pas exécutable. Exécutez : chmod +x
hook-variables = Le programme reçoit les détails du périphérique via des variables d'environnement : { $names }.

## Settings

setting-prompt-on-insert = Demander pour les nouveaux périphériques
setting-prompt-on-insert-description = Afficher une demande de décision quand un périphérique sans règle permanente est connecté.
setting-notify-on-insert = Envoyer une notification de bureau
setting-notify-on-insert-description = Notifier également, afin qu'une demande ne soit pas manquée quand le tableau de bord est masqué.
setting-auto-open-popup = Ouvrir la fenêtre automatiquement
setting-auto-open-popup-description = Laisser USB Guard ouvrir sa fenêtre lorsqu'un périphérique nécessite une décision.
setting-default-permanent = Mémoriser les décisions par défaut
setting-default-permanent-description = Cocher « Mémoriser cette décision » à l'avance dans la demande.
setting-show-hardwired = Afficher les périphériques soudés
setting-show-hardwired-description = Inclure les périphériques qu'USBGuard signale comme câblés et qui ne peuvent pas être débranchés.
setting-show-root-hubs = Afficher les concentrateurs racine
setting-show-root-hubs-description = Inclure les contrôleurs hôtes auxquels les ports USB sont rattachés.
setting-show-internal = Afficher les périphériques internes
setting-show-internal-description = Inclure les périphériques que vous avez marqués comme faisant partie de cette machine.
setting-show-disconnected = Afficher les périphériques déconnectés
setting-show-disconnected-description = Lister les périphériques ayant une règle permanente mais non branchés, afin qu'une décision puisse être modifiée sans eux.
setting-warn-input-capable = Signaler les périphériques pouvant faire clavier
setting-warn-input-capable-description = Mettre en évidence les périphériques capables d'injecter des frappes.
setting-journal-enabled = Conserver un historique des décisions
setting-journal-enabled-description = Enregistrer les événements et décisions dans { $path }.
setting-warn-on-health-problems = Avertir des problèmes de configuration
setting-warn-on-health-problems-description = Afficher une alerte quand USBGuard n'est pas configuré pour protéger ce système.
setting-notify-on-auto-block = Me prévenir quand un périphérique est refusé
setting-notify-on-auto-block-description = Notifier lorsqu'une règle permanente refuse un périphérique sans demander, pour qu'il ne se contente pas d'échouer en silence.
setting-show-tray-icon = Afficher l'icône d'état
setting-show-tray-icon-description = Afficher le bouclier dans la zone de notification. Le désactiver n'empêche pas l'application de surveiller les périphériques.
setting-autostart = Démarrer automatiquement à la connexion
setting-autostart-description = Ajouter USB Guard à votre session afin qu'il surveille avant que vous ne branchiez quoi que ce soit.
setting-start-minimized = Démarrer sans ouvrir la fenêtre
setting-start-minimized-description = À la connexion, n'afficher que l'icône d'état.
setting-run-in-background = Continuer à fonctionner quand la fenêtre est fermée
setting-run-in-background-description = Fermer la fenêtre laisse USB Guard en surveillance. Désactivez ceci et la fermeture quitte l'application, ce qui arrête aussi les demandes.

section-behaviour = Comportement
section-display = Affichage
section-startup = Démarrage et icône d'état
section-privacy = Historique

## Errors

error-service-unavailable = USBGuard n'est pas en cours d'exécution
error-permission-denied = Non autorisé
error-autostart = Impossible de modifier le paramètre de démarrage automatique : { $message }
error-no-tray = Ce bureau n'a pas de zone de notification ; l'icône d'état n'a pas pu être affichée. USB Guard a ouvert sa fenêtre à la place.
error-cannot-remove-rule = La suppression d'une règle permanente nécessite une autorisation d'administrateur, qui n'a pas été accordée. Voir « Vous pouvez annuler une décision permanente » sur la page État.

## About

repository = Dépôt
support = Signaler un problème
version = Version { $version }
