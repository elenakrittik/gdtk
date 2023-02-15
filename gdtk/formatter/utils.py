from pathlib import Path
from typing import List

def find_gdscript_files(path: Path, indent = 0) -> List[Path]:
    files: List[Path] = []

    for entry in path.iterdir():
        if entry.is_file() and entry.name.endswith(".gd"):
            files.append(entry)
        elif entry.is_dir():
            files.extend(find_gdscript_files(entry))

    return files
