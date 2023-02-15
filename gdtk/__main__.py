from pathlib import Path
from typing import Optional

import typer

from gdtk.formatter import format_path

app = typer.Typer()

@app.command("format")
def format_(
    path: Path = typer.Option(
        ...,
        exists=True,
        writable=True,
        readable=True,
        allow_dash=False,
    )
):
    format_path(path)

@app.command("lint")
def lint():
    print("linting")

if __name__ == "__main__":
    app()
