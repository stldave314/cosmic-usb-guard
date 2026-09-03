#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Generate locale files from the English fallback plus a translation table.

The English file is the structural template: comments, section headings and
message order are copied verbatim, and only the *values* are replaced. That
keeps every locale in the same shape as the fallback, so a reviewer comparing
two files sees only the translated text, and it makes it impossible to lose a
key or reorder one by hand.

Placeholders are checked rather than trusted: a translation that drops or
renames a `{ $name }` is rejected here instead of misbehaving at runtime in
that one language, which is the failure mode `tests/i18n.rs` exists to catch.
"""

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
EN = ROOT / "i18n" / "en" / "cosmic_usb_guard.ftl"

HEADER = """# SPDX-License-Identifier: GPL-3.0-or-later
# {language} translation for cosmic-usb-guard.
#
# MACHINE TRANSLATED, NOT REVIEWED BY A NATIVE SPEAKER.
# Several of these strings are security warnings, where a mistranslation could
# mislead someone about what a device can do. Corrections are very welcome:
# https://github.com/stldave314/cosmic-usb-guard/issues
#
# Keys and {{ $placeholders }} must match i18n/en/cosmic_usb_guard.ftl exactly;
# `tests/i18n.rs` enforces that. Regenerate with scripts/i18n/generate.py.
"""

KEY = re.compile(r"^([a-z][a-z0-9-]*) = (.*)$")
PLACEHOLDER = re.compile(r"\{\s*\$([a-z_]+)\s*\}")


def parse_english():
    """Message id -> (start_line, end_line) over the English file's lines."""
    lines = EN.read_text().splitlines()
    spans, current, start = {}, None, None
    for index, line in enumerate(lines):
        match = KEY.match(line)
        if match:
            if current:
                spans[current] = (start, index)
            current, start = match.group(1), index
        elif current and not line.startswith((" ", "\t")):
            spans[current] = (start, index)
            current = None
    if current:
        spans[current] = (start, len(lines))
    return lines, spans


def placeholders(text):
    return sorted(set(PLACEHOLDER.findall(text)))


def render(locale, language, table):
    lines, spans = parse_english()
    out, index = [HEADER.format(language=language)], 0
    problems = []

    while index < len(lines):
        line = lines[index]
        match = KEY.match(line)
        if not match:
            # Comments, section headings and blank lines pass through, except
            # the English file's own header block which the locale replaces.
            if index > 7:
                out.append(line)
            index += 1
            continue

        key = match.group(1)
        start, end = spans[key]
        english = "\n".join(lines[start:end]).split(" = ", 1)[1]

        if key not in table:
            problems.append(f"{locale}: missing translation for {key!r}")
            index = end
            continue

        translated = table[key]
        if placeholders(english) != placeholders(translated):
            problems.append(
                f"{locale}: {key!r} placeholders changed: "
                f"{placeholders(english)} -> {placeholders(translated)}"
            )
        out.append(f"{key} = {translated}")
        index = end

    extra = sorted(set(table) - set(spans))
    problems.extend(f"{locale}: {key!r} is not a key in the fallback" for key in extra)
    return "\n".join(out).rstrip() + "\n", problems


def main():
    table_path = pathlib.Path(sys.argv[1])
    tables = json.loads(table_path.read_text())

    all_problems = []
    for locale, entry in tables.items():
        text, problems = render(locale, entry["language"], entry["messages"])
        all_problems.extend(problems)
        if problems:
            continue
        target = ROOT / "i18n" / locale / "cosmic_usb_guard.ftl"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)
        print(f"wrote {target.relative_to(ROOT)}")

    for problem in all_problems:
        print(problem, file=sys.stderr)
    return 1 if all_problems else 0


if __name__ == "__main__":
    sys.exit(main())
