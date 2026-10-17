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
allow-once = Autoriser une fois
always-allow = Toujours autoriser
refresh = Actualiser
copy = Copier
copy-command = Copier la commande
open-app = Ouvrir USB Guard
quit = Quitter

## Navigation

page-devices = Périphériques
page-thunderbolt = Thunderbolt
page-history = Historique
page-status = État
page-settings = Paramètres

## Device list

devices-heading = Périphériques connectés
devices-none = Aucun périphérique USB
devices-none-description = Rien n'est branché, ou le service USBGuard ne voit pas le bus USB de ce système.
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
bound-port-toggle = Lié à un port USB
bound-port-description = La règle ne s’applique que lorsque l’appareil est branché sur le port { $port }. Ailleurs, il est bloqué et la question vous est reposée.
bound-port-description-unbound = Faire en sorte que la règle ne s’applique que sur le port où l’appareil est branché maintenant. Un appareil qui copie son identité devrait lui aussi être sur ce port.
bound-port-unplugged = Branchez l’appareil pour lier sa règle au port où il se trouve.

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
notify-thunderbolt = Appareil Thunderbolt en attente
notify-thunderbolt-body = { $name } est connecté, mais ne peut pas accéder à la mémoire de l’ordinateur tant que vous ne l’autorisez pas.
remember-decision = Mémoriser cette décision
bind-to-port = Lier à ce port USB ({ $port })
bind-to-port-description = S’applique uniquement sur ce port. Branché ailleurs, l’appareil est bloqué et la question vous est reposée.
prompt-wrong-port = Sa règle est liée au port { $bound }, mais il est maintenant branché sur le port { $port }.

## Warnings

warning-input-capable = Ce périphérique peut se faire passer pour un clavier et saisir du texte en votre nom.
warning-storage = Ce périphérique expose un stockage.
warning-network = Ce périphérique présente un adaptateur réseau, capable de détourner votre trafic.
warning-standing-conflict = Une règle permanente rétablira « { $target } » à la prochaine connexion.
warning-overriding-rule = Une règle plus large, placée plus tôt dans la politique, correspond aussi à cet appareil et est vérifiée en premier : elle peut donc décider à la place de la règle propre à l’appareil : { $rule }
warning-no-hash = Le service USBGuard n'a pas fourni de hachage de descripteur pour cet appareil ; une règle permanente ne peut donc pas lui être liée spécifiquement.

## Status and health

status-ok = Ce système est protégé
status-warning = La protection rencontre des problèmes
status-critical = Ce système n'est pas protégé
status-disconnected = Non connecté au service USBGuard
status-disconnected-description = { $reason }
status-checking = Vérification…

check-daemon-running = Le service USBGuard est actif
check-daemon-enabled = Le service USBGuard démarre avec le système
check-dbus-running = L'interface D-Bus d'USBGuard est active
check-dbus-enabled = L'interface D-Bus d'USBGuard démarre avec le système
check-ipc-reachable = Le service répond aux requêtes
check-ipc-permission = Vous pouvez prendre des décisions de politique
check-decisions-reversible = Vous pouvez annuler une décision permanente
check-inserted-policy = Les nouveaux périphériques attendent une décision
check-policy-not-empty = Une politique de périphériques est configurée
check-thunderbolt-security = Les appareils Thunderbolt doivent être autorisés avant de se connecter
check-thunderbolt-dma = La mémoire est protégée contre les appareils Thunderbolt
check-thunderbolt-service = Le service Thunderbolt (bolt) est en cours d’exécution
check-thunderbolt-permission = Vous pouvez autoriser les appareils Thunderbolt
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt et USB4

check-observed = Observé : { $value }
observed-unknown = inconnu
observed-no-bus = aucune connexion au bus système
observed-responding = répond
observed-granted = accordé
observed-password = nécessite un mot de passe administrateur
observed-refused = refusé
observed-polkit-unavailable = polkit indisponible
observed-rules = règles dans la politique : { $count }
observed-unreadable = illisible
observed-running = en cours d’exécution
observed-not-installed = non installé
observed-stopped = installé mais arrêté
observed-on = activé
observed-off = désactivé
observed-not-reported = non signalé
remedy-heading = Corrigez-le en exécutant :
remedy-explanation = Copiez la commande, collez-la dans un terminal et appuyez sur Entrée. Elle demande votre mot de passe et n’affiche rien en cas de réussite. Appuyez ensuite sur Actualiser pour vérifier que cela a fonctionné.
remedy-policy-warning = À lire d’abord : cela autorise tous les appareils branchés en ce moment. Débranchez tout ce que vous ne reconnaissez pas avant de l’exécuter.
steps-heading = Corrigez-le dans la configuration du micrologiciel :
steps-firmware-only = Ce réglage est stocké dans le micrologiciel de l’ordinateur ; aucune commande ne peut le modifier.
steps-restart = Enregistrez votre travail et redémarrez l’ordinateur.
steps-open-setup = Pendant le démarrage, appuyez sur la touche qui ouvre la configuration du micrologiciel. Elle est généralement affichée à l’écran ; les plus courantes sont F1 ou Entrée (Lenovo), F2 (Dell, ASUS, Acer et la plupart des autres), F10 (HP) et Suppr (cartes mères de bureau).
steps-tb-find = Trouvez les réglages Thunderbolt ou USB4, généralement sous Security, Config ou Advanced.
steps-tb-set = Réglez le niveau de sécurité sur « User Authorization » ou « Secure Connect ». Évitez « No Security » et « Legacy ».
steps-dma-set = Activez « Kernel DMA Protection » ou l’option dont elle dépend : « VT-d » sur Intel, « AMD-Vi » ou « IOMMU » sur AMD. Généralement sous Security, Virtualization ou Advanced.
steps-save = Enregistrez les modifications, quittez et laissez l’ordinateur démarrer. Appuyez ensuite sur Actualiser sur cette page.
steps-session-only = Polkit ne laisse utiliser USBGuard qu’à la personne devant l’ordinateur : la session locale active. Il refuse une session ouverte par SSH ou bureau à distance, et une session que l’on a quittée en changeant d’utilisateur.
steps-session-heading = Pour corriger :
steps-session-local = Exécutez USB Guard dans la session ouverte sur l’ordinateur lui-même.
steps-session-switch = Si vous avez changé d’utilisateur, revenez à cette session.
steps-session-refresh = Appuyez sur Actualiser sur cette page.

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
event-service-up = Le service USBGuard est devenu disponible
event-service-down = Le service USBGuard est devenu indisponible
event-health-problem = Problème de configuration

actor-user = par vous
actor-policy = par les règles du service USBGuard
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
setting-show-hardwired-description = Inclure les appareils que le service USBGuard signale comme intégrés, qui ne peuvent pas être débranchés.
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
setting-warn-on-health-problems-description = Afficher une alerte quand ce système n'est pas entièrement protégé.
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

error-service-unavailable = Le service USBGuard n'est pas en cours d'exécution
error-permission-denied = Non autorisé
error-autostart = Impossible de modifier le paramètre de démarrage automatique : { $message }
error-no-tray = Ce bureau n'a pas de zone de notification ; l'icône d'état n'a pas pu être affichée. USB Guard a ouvert sa fenêtre à la place.
error-cannot-remove-rule = La suppression d'une règle permanente nécessite une autorisation d'administrateur, qui n'a pas été accordée. Voir « Vous pouvez annuler une décision permanente » sur la page État.
error-thunderbolt-permission = Vous n’êtes pas autorisé à autoriser des appareils Thunderbolt. La page État propose une solution sous « Vous pouvez autoriser les appareils Thunderbolt ».
error-bolt-unavailable = Le service Thunderbolt (bolt) n’est pas en cours d’exécution. La page État indique comment le démarrer.
error-thunderbolt = Le service Thunderbolt a signalé une erreur : { $message }

## About

repository = Dépôt
support = Signaler un problème
version = Version { $version }

## Thunderbolt page

tb-heading = Thunderbolt et USB4
tb-security = Niveau de sécurité
tb-level-none = Aucune : tout appareil se connecte sans que la question soit posée
tb-level-user = Autorisation de l’utilisateur : chaque nouvel appareil doit être autorisé
tb-level-secure = Connexion sécurisée : chaque nouvel appareil doit être autorisé, et les appareils mémorisés prouvent leur identité avec une clé
tb-level-dponly = DisplayPort uniquement : aucun appareil n’obtient d’accès PCIe
tb-level-usbonly = USB uniquement : aucun appareil n’obtient d’accès PCIe
tb-level-nopcie = PCIe désactivé : aucun appareil n’obtient d’accès PCIe
tb-dma = Protection DMA
tb-dma-on = Activée : les appareils ne peuvent pas lire la mémoire qui ne leur a pas été attribuée
tb-dma-off = Désactivée : un appareil autorisé peut lire n’importe quelle mémoire
tb-auto-off = L’autorisation automatique est désactivée dans bolt : les appareils mémorisés attendent eux aussi d’être autorisés à chaque fois.
tb-never-asks = À ce niveau de sécurité, il n’y a rien à autoriser : la question n’est jamais posée.
tb-has-problems = La configuration Thunderbolt présente un problème.
tb-open-status = Montrer comment corriger
tb-prompt-heading = Autoriser cet appareil Thunderbolt ?
tb-prompt-description = { $name } est connecté. Tant que vous ne l’autorisez pas, il ne peut accéder ni à la mémoire de l’ordinateur ni au bus PCIe.
tb-devices = Appareils
tb-no-devices = Aucun appareil Thunderbolt ou USB4 n’a été connecté.
tb-remembered = Mémorisé
tb-not-remembered = Non mémorisé
tb-status-waiting = En attente d’autorisation
tb-status-authorized = Autorisé
tb-status-error = Échec de l’autorisation
tb-status-working = Connexion…
tb-forget = Oublier
tb-forget-note = Un appareil Thunderbolt connecté conserve son accès jusqu’à ce qu’il soit débranché ; il n’existe aucun moyen de le lui retirer plus tôt. Oublier un appareil prend effet à sa prochaine connexion.
