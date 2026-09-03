# SPDX-License-Identifier: GPL-3.0-or-later
# Portuguese, Brazil (Português do Brasil) translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and { $placeholders } must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.

app-title = USB Guard
app-description = Revise e controle quais dispositivos USB podem se conectar

## Generic actions

allow = Permitir
block = Bloquear
reject = Rejeitar
revoke = Revogar
forget = Esquecer regra
details = Detalhes
dismiss = Dispensar
refresh = Atualizar
copy = Copiar
open-app = Abrir o USB Guard
quit = Sair

## Navigation

page-devices = Dispositivos
page-history = Histórico
page-status = Status
page-settings = Configurações

## Device list

devices-heading = Dispositivos conectados
devices-none = Nenhum dispositivo USB
devices-none-description = Nada está conectado, ou o USBGuard não enxerga o barramento USB deste sistema.
devices-hidden = { $count } { $count ->
        [one] dispositivo oculto
       *[other] dispositivos ocultos
    } pelas suas configurações de exibição
devices-pending = { $count } { $count ->
        [one] dispositivo aguarda
       *[other] dispositivos aguardam
    } uma decisão
devices-remembered = Não conectados
devices-remembered-description = Dispositivos com uma regra permanente que não estão conectados. Altere ou remova a regra aqui.
device-internal = Dispositivo interno
device-internal-toggle = Dispositivo interno
device-internal-description = Faz parte desta máquina. Fica oculto da lista e nunca é perguntado. Isso não o autoriza — permita-o também se ele deve funcionar.
device-internal-no-hash = Este dispositivo não informa um hash de descritor, portanto não pode ser marcado como interno.

## Device state

state-allowed = Permitido
state-blocked = Bloqueado
state-rejected = Rejeitado
state-unknown = Desconhecido
state-pending = Aguardando decisão
state-disconnected = Não conectado

## Standing policy rules

standing-allow = Sempre permitido
standing-block = Sempre bloqueado
standing-reject = Sempre rejeitado
standing-other = Regra permanente: { $target }

## Device fields

field-name = Nome
field-usb-id = ID USB
field-serial = Número de série
field-port = Porta
field-hash = Hash do descritor
field-interfaces = Interfaces
field-connection = Conexão
field-status = Status
field-none = Não informado

## Decision prompt

prompt-heading = Permitir este dispositivo?
prompt-description = { $name } acabou de ser conectado. Ele permanece bloqueado até você decidir.
notify-new-device = Novo dispositivo USB conectado
notify-auto-blocked = Dispositivo USB recusado
notify-auto-blocked-body = { $name } foi recusado por uma regra permanente e não está disponível. Abra o USB Guard para mudar isso.
notify-manage = Gerenciar dispositivo
remember-decision = Lembrar desta decisão

## Warnings

warning-input-capable = Este dispositivo pode agir como teclado e digitar em seu nome.
warning-storage = Este dispositivo expõe armazenamento.
warning-network = Este dispositivo apresenta um adaptador de rede, que pode desviar o seu tráfego.
warning-standing-conflict = Uma regra permanente voltará isto para “{ $target }” na próxima vez que for conectado.
warning-no-hash = O USBGuard não informou um hash de descritor para este dispositivo, portanto uma regra permanente não pode ser fixada especificamente a ele.

## Status and health

status-ok = O USBGuard está protegendo este sistema
status-warning = O USBGuard está em execução com problemas
status-critical = Este sistema não está protegido
status-disconnected = Não conectado ao USBGuard
status-disconnected-description = { $reason }
status-checking = Verificando…

check-daemon-running = O serviço USBGuard está em execução
check-daemon-enabled = O serviço USBGuard inicia na inicialização
check-dbus-running = A interface D-Bus do USBGuard está em execução
check-dbus-enabled = A interface D-Bus do USBGuard inicia na inicialização
check-ipc-reachable = O serviço responde às solicitações
check-ipc-permission = Você pode tomar decisões de política
check-decisions-reversible = Você pode desfazer uma decisão permanente
check-inserted-policy = Novos dispositivos aguardam uma decisão
check-policy-not-empty = Uma política de dispositivos está configurada

check-observed = Observado: { $value }
remedy-heading = Corrija executando:

## History

history-heading = Histórico de decisões
history-empty = Nada registrado ainda
history-empty-description = Eventos de dispositivos e as decisões tomadas sobre eles aparecerão aqui.
history-clear = Limpar histórico
history-entries = { $count } { $count ->
        [one] entrada
       *[other] entradas
    }
history-filter-all = Todos os eventos
history-filter-decisions = Somente decisões

event-inserted = Conectado
event-removed = Desconectado
event-updated = Alterado
event-allowed = Permitido
event-blocked = Bloqueado
event-rejected = Rejeitado
event-revoked = Revogado
event-service-up = O USBGuard ficou disponível
event-service-down = O USBGuard ficou indisponível
event-health-problem = Problema de configuração

actor-user = por você
actor-policy = pela política do USBGuard
actor-external = fora deste aplicativo
actor-system = automático

## Hooks

hook-heading = Executar um programa ao conectar
hook-description = Só é executado depois que este dispositivo é permitido. Um dispositivo bloqueado nunca executa nada.
hook-program = Programa
hook-program-placeholder = /home/voce/bin/backup.sh
hook-arguments = Argumentos
hook-arguments-placeholder = Um por linha
hook-label = Nome
hook-label-placeholder = Backup
hook-enabled = Ativado
hook-save = Salvar ação
hook-remove = Remover ação
hook-none = Nenhum programa definido
hook-problem-not-set = Escolha um programa para executar.
hook-problem-not-absolute = Informe o caminho completo, começando com uma barra.
hook-problem-missing = Esse arquivo não existe.
hook-problem-not-executable = Esse arquivo não é executável. Execute: chmod +x
hook-variables = O programa recebe os detalhes do dispositivo como variáveis de ambiente: { $names }.

## Settings

setting-prompt-on-insert = Perguntar sobre dispositivos novos
setting-prompt-on-insert-description = Mostrar um pedido de decisão quando um dispositivo sem regra permanente for conectado.
setting-notify-on-insert = Enviar uma notificação da área de trabalho
setting-notify-on-insert-description = Notificar também, para que um pedido não passe despercebido quando o painel estiver oculto.
setting-auto-open-popup = Abrir a janela automaticamente
setting-auto-open-popup-description = Deixar o USB Guard abrir sua janela quando um dispositivo precisar de uma decisão.
setting-default-permanent = Lembrar decisões por padrão
setting-default-permanent-description = Marcar “Lembrar desta decisão” de antemão no pedido.
setting-show-hardwired = Mostrar dispositivos soldados
setting-show-hardwired-description = Incluir dispositivos que o USBGuard informa como fixos e que não podem ser desconectados.
setting-show-root-hubs = Mostrar hubs raiz
setting-show-root-hubs-description = Incluir os controladores host aos quais as portas USB estão ligadas.
setting-show-internal = Mostrar dispositivos internos
setting-show-internal-description = Incluir dispositivos que você marcou como parte desta máquina.
setting-show-disconnected = Mostrar dispositivos desconectados
setting-show-disconnected-description = Listar dispositivos que têm uma regra permanente mas não estão conectados, para que uma decisão possa ser alterada sem eles.
setting-warn-input-capable = Destacar dispositivos capazes de agir como teclado
setting-warn-input-capable-description = Sinalizar dispositivos que poderiam injetar pressionamentos de teclas.
setting-journal-enabled = Manter um histórico de decisões
setting-journal-enabled-description = Registrar eventos e decisões em { $path }.
setting-warn-on-health-problems = Avisar sobre problemas de configuração
setting-warn-on-health-problems-description = Mostrar um alerta quando o USBGuard não estiver configurado para proteger este sistema.
setting-notify-on-auto-block = Avisar quando um dispositivo for recusado
setting-notify-on-auto-block-description = Notificar quando uma regra permanente recusar um dispositivo sem perguntar, para que ele não apenas falhe em silêncio.
setting-show-tray-icon = Mostrar o ícone de status
setting-show-tray-icon-description = Exibir o escudo na área de notificação. Desativar isso não impede o aplicativo de vigiar os dispositivos.
setting-autostart = Iniciar automaticamente ao entrar
setting-autostart-description = Adicionar o USB Guard à sua sessão para que ele vigie antes de você conectar qualquer coisa.
setting-start-minimized = Iniciar sem abrir a janela
setting-start-minimized-description = Ao entrar, mostrar apenas o ícone de status.
setting-run-in-background = Continuar em execução ao fechar a janela
setting-run-in-background-description = Fechar a janela mantém o USB Guard vigiando. Desative isto e fechá-la encerra o aplicativo, o que também interrompe os pedidos.

section-behaviour = Comportamento
section-display = Exibição
section-startup = Inicialização e ícone de status
section-privacy = Histórico

## Errors

error-service-unavailable = O USBGuard não está em execução
error-permission-denied = Não permitido
error-autostart = Não foi possível alterar a configuração de início automático: { $message }
error-no-tray = Esta área de trabalho não tem área de notificação, portanto o ícone de status não pôde ser exibido. O USB Guard abriu sua janela em vez disso.
error-cannot-remove-rule = Remover uma regra permanente exige autorização de administrador, que não foi concedida. Veja “Você pode desfazer uma decisão permanente” na página Status.

## About

repository = Repositório
support = Relatar um problema
version = Versão { $version }
