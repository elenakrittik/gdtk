# Godot ToolKit

`gdtk` is a versatile toolkit for Godot developers. No matter what you're trying to build, it's here to help.
It can\* manage your dependencies (both addons and native extensions like godot-rust), format your GDScript
according to the official style guide, and do semantic anylysis to type-check and lint your code. It also provides
an interactive REPL to quickly test your thoughts, a robust testing framework with first-class Godot integration,
an advanced LSP implementation to access all of `gdtk`'s goodies right from your IDE, and a powerful preprocessor
to <!-- embrace, --> extend <!-- and extinguish --> GDScript syntax for additional features like macros, typed
dictionaries and more with your own modifiers. And all of this speedy awesomeness is backed by a super friendly,
extensible CLI.

## Planned features*

this is essentially a public roadmap

- [ ] Parser
- - [x] Lexer
- - [ ] Actual parser
- [ ] Formatter
- [ ] Linter
- [ ] Project and dependency manager
- - [ ] Addon dependencies
- - [ ] Native dependencies
- - [ ] GPM dependencies
- [ ] Interpreter
- - [ ] REPL
- [ ] Testing framework
- - [ ] Godot integration
- [ ] Preprocessor
- [ ] LSP implementation

## Credits

Greatly inspired by these projects:
- Pawel Lampe's [godot-gdscript-toolkit](https;//github.com/Scony/godot-gdscript-toolkit)
- Paul Hocker's [gs-project-manager](https://gitlab.com/godot-stuff/gs-project-manager/)
