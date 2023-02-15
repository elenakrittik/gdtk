from pathlib import Path
from typing import List

import typer

from .tokens import SPECIAL_TOKENS, KEYWORD_TOKENS # , format_expression
from .utils import find_gdscript_files

def format_path(path: Path):
    if path.is_file():
        format_file(path)
    elif path.is_dir():
        for file in find_gdscript_files(path):
            format_file(file)

def format_file(path: Path):
    content = path.read_text()
    # TODO: Check if file is unchanged before formatting
    formatted = format_lines(content.splitlines())
    path.write_text('\n'.join(formatted))

def format_lines(lines: List[str]) -> List[str]:
    out: List[str] = []

    for line in lines:
        formatter = SPECIAL_TOKENS.get(line[0], None)

        if formatter != None:
            out.extend(formatter(line))
    
    return out
