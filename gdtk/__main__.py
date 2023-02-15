from pathlib import Path
from typing import Optional

import typer

app = typer.Typer()

@app.command("format")
def format_(path: Path):
    print(f"Formatting {path}!")

if __name__ == "__main__":
    app()