#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Validate the desktop entries and AppStream metadata.
#
# One known deviation is tolerated, and only that one: the applet declares
# `Categories=COSMIC;`, which desktop-file-validate rejects because COSMIC is
# not a registered category and does not start with `X-`. Every applet shipped
# by System76 declares it the same way, and `cosmic-panel` is the consumer, so
# following the spec here would mean diverging from the desktop we target.
#
# Everything else is a hard failure. This is deliberately not a blanket
# `|| true`: a validator that always passes is not a validator.
set -euo pipefail

cd "$(dirname "$0")/.."

for tool in desktop-file-validate appstreamcli; do
    command -v "$tool" >/dev/null 2>&1 || {
        echo "FAIL: $tool is required (install desktop-file-utils and appstream)" >&2
        exit 1
    }
done

# The single tolerated message, matched narrowly so an unrelated Categories
# problem still fails.
KNOWN='unregistered value "COSMIC"'

fail=0

for file in res/*.desktop; do
    echo "== $file"
    output=$(desktop-file-validate "$file" 2>&1) && status=0 || status=$?

    # Drop hints (advisory) and the one known deviation; anything left is real.
    remaining=$(printf '%s\n' "$output" \
        | grep -v ': hint: ' \
        | grep -vF "$KNOWN" \
        | grep -v '^$' || true)

    if [[ -n "$remaining" ]]; then
        printf '%s\n' "$remaining"
        echo "FAIL: $file"
        fail=1
    elif [[ $status -ne 0 ]]; then
        echo "OK (only the known COSMIC category deviation)"
    else
        echo "OK"
    fi
done

for file in res/*.metainfo.xml; do
    echo "== $file"
    if appstreamcli validate --no-net --explain "$file"; then
        echo "OK"
    else
        echo "FAIL: $file"
        fail=1
    fi
done

if [[ $fail -ne 0 ]]; then
    echo
    echo "RESULT: FAILED"
    exit 1
fi
echo
echo "RESULT: metadata is valid"
