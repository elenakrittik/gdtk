from typing import Callable, Dict, List, Union
from enum import Enum

from rich import print

from .code import format_code

class AnnotationParseMode(Enum):
    annotation_name = 0
    annotation_values = 1
    annotation_value = 2
    annotation_code = 3
    unknown = 4

def format_comment(line: str) -> List[str]:
    return ["# " + line.removeprefix("#").strip()]

def format_annotation(line: str) -> List[str]:
    out: List[str] = []

    annotations: List[Dict[str, Union[str, List[str]]]] = []
    annotation_name_start: int = -1
    mode: AnnotationParseMode = AnnotationParseMode.unknown
    current_annotation_idx: int = -1
    char_idx: int = -1
    annotation_value_start: int = -1
    code_lines: List[str] = []
    stop: bool = False

    for char_idx, char in enumerate(line):
        if mode == AnnotationParseMode.annotation_value:
            if char == '"':
                annotations[current_annotation_idx]["values"].append(line[annotation_value_start + 1:char_idx]) # type: ignore
                mode = AnnotationParseMode.annotation_values
                continue
            else:
                continue

        match char:
            case '@':
                annotation_name_start = char_idx
                annotations.append({
                    "name": "",
                    "values": []
                })
                current_annotation_idx += 1
                mode = AnnotationParseMode.annotation_name
            case '(':
                mode = AnnotationParseMode.annotation_values
                annotations[current_annotation_idx]["name"] = line[annotation_name_start + 1:char_idx]
            case '"':
                mode = AnnotationParseMode.annotation_value
                annotation_value_start = char_idx
            case ',':
                continue
            case ')':
                mode = AnnotationParseMode.unknown
            case ' ':
                if line[char_idx + 1] != '"' and annotations[current_annotation_idx]["name"].isspace(): # type: ignore
                    annotations[current_annotation_idx]["name"] = line[annotation_name_start + 1:char_idx + 1]
            case _:
                if mode == AnnotationParseMode.unknown:
                    mode = AnnotationParseMode.annotation_code
                    to_format = line[char_idx:].strip()

                    if not to_format.is_empty():
                        if to_format[0] == "#":
                            code_lines = format_comment(to_format)
                        else:
                            code_lines = format_code(to_format)

                    stop = True
        
        if stop:
            break

    for ann in annotations:
        out.append("@{}({})".format(
            ann['name'],
            ', '.join(['"' + v + '"' for v in ann['values']])
        ))

    out.extend(code_lines)

    return out

SPECIAL_TOKENS: Dict[str, Callable[[str], List[str]]] = {
    "@": format_annotation,
    "#": format_comment,
}
