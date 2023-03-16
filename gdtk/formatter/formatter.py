from pathlib import Path
from typing import List

from rich import print
from rich.progress import (
    Progress,
    SpinnerColumn,
    TextColumn,
    BarColumn,
    TaskProgressColumn,
)

from .tokens import SPECIAL_TOKENS
# from .code import KEYWORD_TOKENS, format_expression
from .utils import find_gdscript_files


def make_progress(ft: str = None):
    return Progress(
        SpinnerColumn(
            finished_text=ft
        ),
        TextColumn("[progress.description]{task.description}"),
        BarColumn(),
        TaskProgressColumn(),
    )


def format_path(path: Path):
    if path.is_file():
        files = [path]
    elif path.is_dir():
        files = find_gdscript_files(path)

    progress = make_progress()

    with progress:
        task = progress.add_task(
            "[green]Formatting {} file{}".format(
                len(files),
                's' if len(files) > 1 else '',
            ),
            total=len(files),
        )

        filesi = iter(files)

        while not progress.finished:
            file = next(filesi)

            format_file(file)

            print(f"  [green]Formatted[/] [blue]{file}[/]")
            progress.update(task, advance=1.0)

        progress.stop()

        print("  [green]Formatted {} file{}".format(
            len(files),
            's' if len(files) > 1 else '',
        ))


def format_file(path: Path):
    content = path.read_text()
    # TODO: Check if file is unchanged before formatting
    formatted = format_lines(content.splitlines())
    path.write_text('\n'.join(formatted))


def format_lines(lines: List[str]) -> List[str]:
    out: List[str] = []

    for line in lines:
        formatter = SPECIAL_TOKENS.get(line[0], None)

        if formatter is not None:
            out.extend(formatter(line))
            continue

        raise RuntimeError(f"No formatter available for line: '{line}'")
    return out
