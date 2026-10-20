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
allow-once = Permitir una vez
always-allow = Permitir siempre
refresh = Actualizar
copy = Copiar
copy-command = Copiar comando
open-app = Abrir USB Guard
quit = Salir

## Navigation

page-devices = Dispositivos
page-thunderbolt = Thunderbolt
page-history = Historial
page-status = Estado
page-settings = Ajustes

## Device list

devices-heading = Dispositivos conectados
devices-none = No hay dispositivos USB
devices-none-description = No hay nada conectado, o el servicio USBGuard no puede ver el bus USB de este sistema.
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
bound-port-toggle = Vinculado a un puerto USB
bound-port-description = La regla solo se aplica mientras el dispositivo esté conectado al puerto { $port }. En cualquier otro lugar se bloquea y se le vuelve a preguntar.
bound-port-description-unbound = Hacer que la regla solo se aplique en el puerto donde el dispositivo está conectado ahora. Un dispositivo que copie su identidad también tendría que estar en ese puerto.
bound-port-unplugged = Conecte el dispositivo para vincular su regla al puerto en el que esté.

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
notify-thunderbolt = Dispositivo Thunderbolt en espera
notify-thunderbolt-body = { $name } está conectado, pero no puede acceder a la memoria del equipo hasta que lo permita.
remember-decision = Recordar esta decisión
bind-to-port = Vincular a este puerto USB ({ $port })
bind-to-port-description = Solo se aplica en este puerto. Conectado en cualquier otro, el dispositivo se bloquea y se le vuelve a preguntar.
prompt-wrong-port = Su regla está vinculada al puerto { $bound }, pero ahora está conectado al puerto { $port }.

## Warnings

warning-input-capable = Este dispositivo puede actuar como teclado y escribir en su nombre.
warning-storage = Este dispositivo expone almacenamiento.
warning-network = Este dispositivo presenta un adaptador de red, que puede desviar su tráfico.
warning-standing-conflict = Una regla permanente lo devolverá a «{ $target }» la próxima vez que se conecte.
warning-overriding-rule = Una regla más amplia, anterior en la política, también coincide con este dispositivo y se comprueba primero, así que puede decidir en lugar de la regla propia del dispositivo: { $rule }
warning-no-hash = El servicio USBGuard no informó de un hash de descriptor para este dispositivo, así que no se le puede vincular específicamente una regla permanente.

## Status and health

status-ok = Este sistema está protegido
status-warning = La protección tiene problemas
status-critical = Este sistema no está protegido
status-disconnected = Sin conexión con el servicio USBGuard
status-disconnected-description = { $reason }
status-checking = Comprobando…

check-daemon-running = El servicio USBGuard está en ejecución
check-daemon-enabled = El servicio USBGuard se inicia con el sistema
check-dbus-running = La interfaz D-Bus de USBGuard está en ejecución
check-dbus-enabled = La interfaz D-Bus de USBGuard se inicia con el sistema
check-ipc-reachable = El servicio responde a las peticiones
check-ipc-permission = Puede tomar decisiones de política
check-decisions-reversible = Puede deshacer una decisión permanente
check-inserted-policy = Los dispositivos nuevos esperan una decisión
check-policy-not-empty = Hay una política de dispositivos configurada
check-thunderbolt-security = Los dispositivos Thunderbolt deben permitirse antes de conectarse
check-thunderbolt-dma = La memoria está protegida frente a dispositivos Thunderbolt
check-thunderbolt-service = El servicio Thunderbolt (bolt) está en ejecución
check-thunderbolt-permission = Puede permitir dispositivos Thunderbolt
status-group-usbguard = USB
status-group-thunderbolt = Thunderbolt y USB4

check-observed = Observado: { $value }
observed-unknown = desconocido
observed-no-bus = sin conexión con el bus del sistema
observed-responding = responde
observed-granted = concedido
observed-password = requiere una contraseña de administrador
observed-asks-password = pide una contraseña, lo cual es correcto
observed-refused = rechazado
observed-polkit-unavailable = polkit no disponible
observed-rules = reglas en la política: { $count }
observed-unreadable = no se pudo leer
observed-running = en ejecución
observed-not-installed = no instalado
observed-stopped = instalado pero no en ejecución
observed-on = activado
observed-off = desactivado
observed-not-reported = no indicado
remedy-heading = Corríjalo ejecutando:
remedy-explanation = Copie el comando, péguelo en una terminal y pulse Intro. Le pedirá su contraseña y no mostrará nada si tiene éxito. Después pulse Actualizar para comprobar que funcionó.
remedy-policy-warning = Lea esto primero: permite todos los dispositivos conectados en este momento. Desconecte lo que no reconozca antes de ejecutarlo.
steps-heading = Corríjalo en la configuración del firmware:
steps-firmware-only = Este ajuste se guarda en el firmware del equipo, así que ningún comando puede cambiarlo.
steps-restart = Guarde su trabajo y reinicie el equipo.
steps-open-setup = Mientras arranca, pulse la tecla que abre la configuración del firmware. Suele mostrarse en pantalla; las habituales son F1 o Intro (Lenovo), F2 (Dell, ASUS, Acer y la mayoría), F10 (HP) y Supr (placas de escritorio).
steps-tb-find = Busque los ajustes de Thunderbolt o USB4, normalmente en Security, Config o Advanced.
steps-tb-set = Ponga el nivel de seguridad en «User Authorization» o «Secure Connect». Evite «No Security» y «Legacy».
steps-dma-set = Active «Kernel DMA Protection» o la opción de la que depende: «VT-d» en Intel, «AMD-Vi» o «IOMMU» en AMD. Suele estar en Security, Virtualization o Advanced.
steps-save = Guarde los cambios, salga y deje que el equipo arranque. Después pulse Actualizar en esta página.
steps-session-only = Polkit solo permite usar USBGuard a la persona que está ante el equipo: la sesión local activa. Rechaza una sesión por SSH o escritorio remoto, y una de la que se ha cambiado.
steps-session-heading = Para corregirlo:
steps-session-local = Ejecute USB Guard en la sesión del propio equipo.
steps-session-switch = Si cambió de usuario, vuelva a esta sesión.
steps-session-refresh = Pulse Actualizar en esta página.

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
event-service-up = El servicio USBGuard pasó a estar disponible
event-service-down = El servicio USBGuard dejó de estar disponible
event-health-problem = Problema de configuración

actor-user = por usted
actor-policy = por las reglas del servicio USBGuard
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
setting-show-hardwired-description = Incluir los dispositivos que el servicio USBGuard indica como integrados, que no se pueden desconectar.
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
setting-warn-on-health-problems-description = Mostrar una alerta cuando este sistema no esté totalmente protegido.
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

error-service-unavailable = El servicio USBGuard no está en ejecución
error-permission-denied = No permitido
error-autostart = No se pudo cambiar el ajuste de inicio automático: { $message }
error-no-tray = Este escritorio no tiene área de notificación, así que no se pudo mostrar el icono de estado. USB Guard abrió su ventana en su lugar.
error-cannot-remove-rule = Eliminar una regla permanente requiere autorización de administrador, que no se concedió. Consulte «Puede deshacer una decisión permanente» en la página Estado.
error-thunderbolt-permission = No tiene permiso para autorizar dispositivos Thunderbolt desde esta sesión. La página de estado explica el motivo en «Puede permitir dispositivos Thunderbolt».
error-bolt-unavailable = El servicio Thunderbolt (bolt) no está en ejecución. La página de estado muestra cómo iniciarlo.
error-thunderbolt = El servicio Thunderbolt informó de un error: { $message }

## About

repository = Repositorio
support = Informar de un problema
version = Versión { $version }

## Thunderbolt page

tb-heading = Thunderbolt y USB4
tb-security = Nivel de seguridad
tb-level-none = Ninguno: todos los dispositivos se conectan sin preguntar
tb-level-user = Autorización del usuario: cada dispositivo nuevo debe permitirse
tb-level-secure = Conexión segura: cada dispositivo nuevo debe permitirse y los recordados demuestran su identidad con una clave
tb-level-dponly = Solo DisplayPort: ningún dispositivo obtiene acceso PCIe
tb-level-usbonly = Solo USB: ningún dispositivo obtiene acceso PCIe
tb-level-nopcie = PCIe desactivado: ningún dispositivo obtiene acceso PCIe
tb-dma = Protección DMA
tb-dma-on = Activada: los dispositivos no pueden leer memoria que no se les haya asignado
tb-dma-off = Desactivada: un dispositivo permitido puede leer cualquier memoria
tb-auto-off = La autorización automática está desactivada en bolt, así que los dispositivos recordados también esperan a ser permitidos cada vez.
tb-never-asks = En este nivel de seguridad no hay nada que permitir: nunca se pregunta por los dispositivos.
tb-has-problems = Hay un problema con la configuración de Thunderbolt.
tb-open-status = Mostrar cómo corregirlo
tb-prompt-heading = ¿Permitir este dispositivo Thunderbolt?
tb-prompt-description = { $name } está conectado. Hasta que lo permita, no puede acceder a la memoria del equipo ni al bus PCIe.
tb-devices = Dispositivos
tb-no-devices = No se ha conectado ningún dispositivo Thunderbolt ni USB4.
tb-remembered = Recordado
tb-not-remembered = No recordado
tb-status-waiting = Esperando permiso
tb-status-authorized = Permitido
tb-status-error = La autorización falló
tb-status-working = Conectando…
tb-forget = Olvidar
tb-forget-note = Un dispositivo Thunderbolt conectado conserva su acceso hasta que se desconecta; no hay forma de retirarlo antes. Olvidar un dispositivo surte efecto la próxima vez que se conecte.
