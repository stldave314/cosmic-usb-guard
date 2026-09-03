#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Validate the desktop entries and AppStream metadata.
#
# Every finding is a hard failure. This is deliberately not a blanket
# `|| true`: a validator that always passes is not a validator.
#
# The `Categories=COSMIC;` deviation this used to tolerate went away with the
# panel applet — the app now publishes a StatusNotifierItem instead, and its
# one desktop entry is an ordinary application.
set -euo pipefail

cd "$(dirname "$0")/.."

for tool in desktop-file-validate appstreamcli; do
    command -v "$tool" >/dev/null 2>&1 || {
        echo "FAIL: $tool is required (install desktop-file-utils and appstream)" >&2
        exit 1
    }
done

fail=0

for file in res/*.desktop; do
    echo "== $file"
    output=$(desktop-file-validate "$file" 2>&1) && status=0 || status=$?

    # Drop hints, which are advisory; anything left is real.
    remaining=$(printf '%s\n' "$output" \
        | grep -v ': hint: ' \
        | grep -v '^$' || true)

    if [[ -n "$remaining" ]]; then
        printf '%s\n' "$remaining"
        echo "FAIL: $file"
        fail=1
    elif [[ $status -ne 0 ]]; then
        echo "OK (hints only)"
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
