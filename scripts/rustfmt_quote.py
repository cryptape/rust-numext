#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""Format rust code inside "quote!()".

Try best to let rust code inside "quote!()" fake normal rust code and
process them by rustfmt.
"""

from __future__ import print_function

import sys
import os
import subprocess
import re
import io

RUSTFMT_PATH = os.getenv('RUSTFMT_PATH', 'rustfmt')
SPACE_PATTERN = re.compile(r' *')
START_PATTERN = re.compile(r'quote!\s*[({]$')
QUOTE_TOKEN_PATTERN = re.compile(r'#([_a-zA-Z][_a-zA-Z0-9]*)')

MOD_BLOCK_PATTERN = re.compile(r'(?m)^\s*    (?:impl[ <]|(?:pub )?trait)')
FN_BLOCK_PATTERN = re.compile(r'(?m)^\s*    (?:pub |const )?fn ')

MOD_START = "mod rustfmt {\n".encode('utf-8')
IMPL_START = "impl Rustfmt {\n".encode('utf-8')
FN_START = "fn rustfmt() {\n".encode('utf-8')
ENUM_START = "enum Rustfmt {\n".encode('utf-8')
MOD_END = "}\n".encode('utf-8')

# Code that should not run rustfmt in the quote block
BLACKLIST = re.compile(
    r'(?m)^\s*(?:#(?:errors|preludes|defuns|part_core)|}?\)\*)\s*$')


def write_block(block, indent, file):
    """Try to fake normal rust code so as to can process by rustfmt."""
    for _ in range(indent - 1):
        file.write(MOD_START)

    if block.endswith(",\n"):
        file.write(ENUM_START)
    elif MOD_BLOCK_PATTERN.search(block) is not None:
        file.write(MOD_START)
    elif FN_BLOCK_PATTERN.search(block) is not None:
        if 'self' in block:
            file.write(IMPL_START)
        else:
            file.write(MOD_START)
    else:
        file.write(FN_START)

    file.write(QUOTE_TOKEN_PATTERN.sub(u'Δ\\1', block).encode('utf-8'))

    for _ in range(indent):
        file.write(MOD_END)


def rustfmt_block(block, indent):
    """Execute rustfmt for a piece of rust code."""
    # Will skip code which rustfmt can't handle them properly.
    if BLACKLIST.search(block) is None:

        rustfmt = subprocess.Popen(
            RUSTFMT_PATH,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)

        write_block(block, indent, rustfmt.stdin)
        rustfmt.stdin.close()

        if rustfmt.wait() == 0:
            outlines = [
                l.decode('utf-8').replace(u'Δ', u'#')
                for l in rustfmt.stdout.readlines()
            ]
            return u''.join(outlines[indent:-indent])

        write_block(block, indent, sys.stderr)
        sys.stderr.write(rustfmt.stderr.read())

    return None


def rustfmt_quote(path):
    """Execute rustfmt for rust code inside "quote!()" in the provided file."""
    indent = 0
    end_pattern = re.compile('[})]')
    in_quote = False
    out_lines = []
    quote_lines = []
    changed = False

    with io.open(path, encoding='utf-8') as file:
        for line in file:
            if in_quote:
                if end_pattern.match(line) is not None:
                    in_quote = False
                    block = u''.join(quote_lines)
                    fmt_result = rustfmt_block(block, indent)
                    if fmt_result is not None and fmt_result != block:
                        changed = True
                        out_lines.append(fmt_result)
                    else:
                        out_lines.append(block)

                    out_lines.append(line)
                else:
                    quote_lines.append(line)
            else:
                out_lines.append(line)
                if START_PATTERN.search(line) is not None:
                    in_quote = True
                    quote_lines = []
                    leading_spaces = SPACE_PATTERN.match(line).group()
                    indent = int(len(leading_spaces) / 4) + 1
                    end_pattern = re.compile('^{}[)}}]'.format(leading_spaces))

    if changed:
        with io.open(path, 'w', encoding='utf-8') as file:
            for line in out_lines:
                file.write(line)

    return changed


def main():
    """Traverse files and execute rustfmt to each."""
    # Traverse the current directory, and format files which are ends with
    # ".rs" and not in directories named ".git" or "target".
    if len(sys.argv) == 1:
        for root, dirs, files in os.walk('.'):
            if '.git' in dirs:
                dirs.remove('.git')
            if 'target' in dirs:
                dirs.remove('target')

            for name in files:
                if name.endswith('.rs'):
                    path = os.path.join(root, name)
                    if rustfmt_quote(path):
                        print('rustfmt_quote {}'.format(path))

    # Format the provided files.
    else:
        for arg in sys.argv[1:]:
            rustfmt_quote(arg)


if __name__ == '__main__':
    main()
