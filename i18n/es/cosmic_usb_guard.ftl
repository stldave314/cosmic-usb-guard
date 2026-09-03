# SPDX-License-Identifier: GPL-3.0-or-later
# Spanish (Español) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Revisar y controlar qué dispositivos USB pueden conectarse

## Generic actions

allow = Permitir
block = Bloquear
reject = Rechazar
revoke = Revocar
forget = Olvidar regla
details = Detalles
dismiss = Descartar
refresh = Actualizar
copy = Copiar
open-app = Abrir USB Guard
quit = Salir

## Navigation

page-devices = Dispositivos
page-history = Historial
page-status = Estado
page-settings = Ajustes

## Device list

devices-heading = Dispositivos conectados
devices-none = No hay dispositivos USB
devices-none-description = No hay nada conectado, o USBGuard no ve el bus USB de este sistema.
devices-hidden = { $count } { $count ->
        [one] dispositivo oculto
       *[other] dispositivos ocultos
    } por sus ajustes de visualización
devices-pending = { $count } { $count ->
        [one] dispositivo espera
       *[other] dispositivos esperan
    } una decisión
devices-remembered = No conectados
devices-remembered-description = Dispositivos con una regla permanente que no están conectados. Cambie o elimine la regla aquí.
device-internal = Dispositivo interno
device-internal-toggle = Dispositivo interno
device-internal-description = Forma parte de este equipo. Se oculta de la lista y nunca se pregunta por él. Esto no lo autoriza: permítalo también si debe funcionar.
device-internal-no-hash = Este dispositivo no informa de un hash de descriptor, así que no puede marcarse como interno.

## Device state

state-allowed = Permitido
state-blocked = Bloqueado
state-rejected = Rechazado
state-unknown = Desconocido
state-pending = Decisión pendiente
state-disconnected = No conectado

## Standing policy rules

standing-allow = Siempre permitido
standing-block = Siempre bloqueado
standing-reject = Siempre rechazado
standing-other = Regla permanente: { $target }

## Device fields

field-name = Nombre
field-usb-id = ID USB
field-serial = Número de serie
field-port = Puerto
field-hash = Hash del descriptor
field-interfaces = Interfaces
field-connection = Conexión
field-status = Estado
field-none = No informado

## Decision prompt

prompt-heading = ¿Permitir este dispositivo?
prompt-description = { $name } acaba de conectarse. Permanece bloqueado hasta que decida.
notify-new-device = Nuevo dispositivo USB conectado
notify-auto-blocked = Dispositivo USB rechazado
notify-auto-blocked-body = { $name } fue rechazado por una regla permanente y no está disponible. Abra USB Guard para cambiarlo.
notify-manage = Gestionar dispositivo
remember-decision = Recordar esta decisión

## Warnings

warning-input-capable = Este dispositivo puede actuar como teclado y escribir en su nombre.
warning-storage = Este dispositivo expone almacenamiento.
warning-network = Este dispositivo presenta un adaptador de red, que puede desviar su tráfico.
warning-standing-conflict = Una regla permanente lo devolverá a «{ $target }» la próxima vez que se conecte.
warning-no-hash = USBGuard no informó de un hash de descriptor para este dispositivo, así que una regla permanente no puede fijarse específicamente a él.

## Status and health

status-ok = USBGuard está protegiendo este sistema
status-warning = USBGuard está funcionando con problemas
status-critical = Este sistema no está protegido
status-disconnected = No conectado a USBGuard
status-disconnected-description = { $reason }
status-checking = Comprobando…

check-daemon-running = El servicio USBGuard está en ejecución
check-daemon-enabled = El servicio USBGuard se inicia al arrancar
check-dbus-running = La interfaz D-Bus de USBGuard está en ejecución
check-dbus-enabled = La interfaz D-Bus de USBGuard se inicia al arrancar
check-ipc-reachable = El servicio responde a las peticiones
check-ipc-permission = Puede tomar decisiones de política
check-decisions-reversible = Puede deshacer una decisión permanente
check-inserted-policy = Los dispositivos nuevos esperan una decisión
check-policy-not-empty = Hay una política de dispositivos configurada

check-observed = Observado: { $value }
remedy-heading = Corríjalo ejecutando:

## History

history-heading = Historial de decisiones
history-empty = Aún no hay nada registrado
history-empty-description = Los eventos de dispositivos y las decisiones tomadas sobre ellos aparecerán aquí.
history-clear = Borrar historial
history-entries = { $count } { $count ->
        [one] entrada
       *[other] entradas
    }
history-filter-all = Todos los eventos
history-filter-decisions = Solo decisiones

event-inserted = Conectado
event-removed = Desconectado
event-updated = Modificado
event-allowed = Permitido
event-blocked = Bloqueado
event-rejected = Rechazado
event-revoked = Revocado
event-service-up = USBGuard pasó a estar disponible
event-service-down = USBGuard dejó de estar disponible
event-health-problem = Problema de configuración

actor-user = por usted
actor-policy = por la política de USBGuard
actor-external = fuera de esta aplicación
actor-system = automático

## Hooks

hook-heading = Ejecutar un programa al conectarse
hook-description = Solo se ejecuta después de permitir este dispositivo. Un dispositivo bloqueado nunca ejecuta nada.
hook-program = Programa
hook-program-placeholder = /home/usuario/bin/copia.sh
hook-arguments = Argumentos
hook-arguments-placeholder = Uno por línea
hook-label = Nombre
hook-label-placeholder = Copia de seguridad
hook-enabled = Activado
hook-save = Guardar acción
hook-remove = Eliminar acción
hook-none = Ningún programa definido
hook-problem-not-set = Elija un programa para ejecutar.
hook-problem-not-absolute = Introduzca la ruta completa, empezando por una barra.
hook-problem-missing = Ese archivo no existe.
hook-problem-not-executable = Ese archivo no es ejecutable. Ejecute: chmod +x
hook-variables = El programa recibe los datos del dispositivo como variables de entorno: { $names }.

## Settings

setting-prompt-on-insert = Preguntar por los dispositivos nuevos
setting-prompt-on-insert-description = Mostrar una petición de decisión cuando se conecte un dispositivo sin regla permanente.
setting-notify-on-insert = Enviar una notificación de escritorio
setting-notify-on-insert-description = Notificar también, para no perder una petición cuando el panel esté oculto.
setting-auto-open-popup = Abrir la ventana automáticamente
setting-auto-open-popup-description = Dejar que USB Guard abra su ventana cuando un dispositivo necesite una decisión.
setting-default-permanent = Recordar las decisiones de forma predeterminada
setting-default-permanent-description = Marcar «Recordar esta decisión» de antemano en la petición.
setting-show-hardwired = Mostrar dispositivos soldados
setting-show-hardwired-description = Incluir dispositivos que USBGuard informa como cableados y que no pueden desconectarse.
setting-show-root-hubs = Mostrar concentradores raíz
setting-show-root-hubs-description = Incluir los controladores anfitrión de los que cuelgan los puertos USB.
setting-show-internal = Mostrar dispositivos internos
setting-show-internal-description = Incluir los dispositivos que ha marcado como parte de este equipo.
setting-show-disconnected = Mostrar dispositivos desconectados
setting-show-disconnected-description = Listar dispositivos que tienen una regla permanente pero no están conectados, para poder cambiar una decisión sin ellos.
setting-warn-input-capable = Destacar dispositivos que pueden actuar como teclado
setting-warn-input-capable-description = Señalar los dispositivos que podrían inyectar pulsaciones de teclas.
setting-journal-enabled = Mantener un historial de decisiones
setting-journal-enabled-description = Registrar eventos y decisiones en { $path }.
setting-warn-on-health-problems = Avisar de problemas de configuración
setting-warn-on-health-problems-description = Mostrar una alerta cuando USBGuard no esté configurado para proteger este sistema.
setting-notify-on-auto-block = Avisarme cuando se rechace un dispositivo
setting-notify-on-auto-block-description = Notificar cuando una regla permanente rechace un dispositivo sin preguntar, para que no falle en silencio.
setting-show-tray-icon = Mostrar el icono de estado
setting-show-tray-icon-description = Mostrar el escudo en el área de notificación. Desactivarlo no impide que la aplicación vigile los dispositivos.
setting-autostart = Iniciar automáticamente al iniciar sesión
setting-autostart-description = Añadir USB Guard a su sesión para que vigile antes de que conecte nada.
setting-start-minimized = Iniciar sin abrir la ventana
setting-start-minimized-description = Al iniciar sesión, mostrar solo el icono de estado.
setting-run-in-background = Seguir funcionando al cerrar la ventana
setting-run-in-background-description = Cerrar la ventana deja a USB Guard vigilando. Desactive esto y cerrarla saldrá de la aplicación, lo que también detiene las peticiones.

section-behaviour = Comportamiento
section-display = Visualización
section-startup = Inicio e icono de estado
section-privacy = Historial

## Errors

error-service-unavailable = USBGuard no está en ejecución
error-permission-denied = No permitido
error-autostart = No se pudo cambiar el ajuste de inicio automático: { $message }
error-no-tray = Este escritorio no tiene área de notificación, así que no se pudo mostrar el icono de estado. USB Guard abrió su ventana en su lugar.
error-cannot-remove-rule = Eliminar una regla permanente requiere autorización de administrador, que no se concedió. Consulte «Puede deshacer una decisión permanente» en la página Estado.

## About

repository = Repositorio
support = Informar de un problema
version = Versión { $version }
