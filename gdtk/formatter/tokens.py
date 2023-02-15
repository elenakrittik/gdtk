from typing import Callable, Dict, List, Union
from enum import Enum

from rich import print

class AnnotationParseMode(Enum):
    annotation_name = 0
    annotation_values = 1
    annotation_value = 2
    annotation_code = 3

def format_annotation(line: str) -> List[str]:
    out: List[str] = []

    annotations: List[Dict[str, Union[str, List[str]]]] = []
    annotation_name_start: int = -1
    mode: AnnotationParseMode = AnnotationParseMode.annotation_name
    current_annotation_idx: int = -1
    char_idx: int = -1
    annotation_value_start: int = -1

    for char_idx, char in enumerate(line):
        if mode == AnnotationParseMode.annotation_value:
            if char == '"':
                annotations[current_annotation_idx]["values"].append(line[annotation_value_start + 1:char_idx]) # type: ignore
                mode = AnnotationParseMode.annotation_values
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
            case '(':
                mode = AnnotationParseMode.annotation_values
                annotations[current_annotation_idx]["name"] = line[annotation_name_start + 1:char_idx]
            case '"':
                mode = AnnotationParseMode.annotation_value
                annotation_value_start = char_idx
            case ',':
                continue
            case ')':
                mode = AnnotationParseMode.annotation_name
            case ' ':
                if line[char_idx + 1] != '"':
                    annotations[current_annotation_idx]["name"] = line[annotation_name_start + 1:char_idx + 1]

    for ann in annotations:
        out.append("@{}({})".format(
            ann['name'],
            ', '.join(['"' + v + '"' for v in ann['values']])
        ))

    print(annotations)

    return out

SPECIAL_TOKENS: Dict[str, Callable[[str], List[str]]] = {
    "@": format_annotation,
}

KEYWORD_TOKENS = {

}
