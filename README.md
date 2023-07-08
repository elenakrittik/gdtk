# gdtk

`gdtk` aims to be *the*<sup>tm</sup> ultimate<sup>tm</sup>, all-in-one<sup>tm</sup> toolkit<sup>tm</sup> for Godot developers. No matter what you're trying to build,
`gdtk` is here to help. It can\* manage your dependencies (both addons and native extensions like godot-rust), auto-format your GDScript
according to the official style guide and C# using CSharpier, do semantic anylysis to type-check and lint your code.
It also provides an interactive REPL to quickly test your thoughts, a robust testing framework with first-class integration with Godot's UI,
an advanced LSP implementation to use all `gdtk`'s goodies right from your IDE, and a preprocessor to <!-- embrace, --> extend <!-- and extinguish -->
GDScript's syntax for additional features like macros, typed dictionaries and more with your own modifiers. And all of this awesomeness is backed
by a super friendly CLI written in Rust.

## Planned features*

this is essentially a public todo-list

- [ ] Parser
- [ ] Formatter
- [ ] Linter
- [ ] Project and dependecy manager
- - [ ] Might also include support for GPM.
- [ ] Interpreter (with REPL)
- [ ] Testing framework
- - [ ] + godot addon
- [ ] Preprocessor
- [ ] LSP implementation

## Credits

Greatly inspired by these projects:
- Pawel Lampe's [godot-gdscript-toolkit](https;//github.com/Scony/godot-gdscript-toolkit)
- Paul Hocker's [gs-project-manager](https://gitlab.com/godot-stuff/gs-project-manager/)
