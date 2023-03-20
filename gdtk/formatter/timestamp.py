from pathlib import Path

from platformdirs import PlatformDirs as Dirs

from gdtk import (
    __title__ as appname,
    __author__ as appauthor,
    __version__ as appversion,
)

dirs = Dirs(
    appname=appname,
    appauthor=appauthor,
    version=appversion,
    opinion=False,
)


class Timestamp:
    path: Path
    data: dict[str, int]

    def __init__(self, path: Path) -> None:
        self.path = path

    def load(self) -> None:
        pass
