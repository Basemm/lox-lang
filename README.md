# lox-lang

Lox language interpreter and VM - Based on Crafting Interpreters Book (Robert Nystrom)

# TODO

- [ ] Scanner should use an array and foreach instead of match also no need to advance with one char
- [ ] Add peek until
- [ ] Separate error handling from scanner & parser
- [ ] Create separate error and report methods to allow aggregation of errors
- [ ] Show error message with file, line number and column
- [ ] Test cases
- [ ] Change Scanner to work with streams instead of reading the whole file in memory
- [ ] Think about lazily streaming the Scanner and Parser like rust iterators?
- [ ] Review & compare speed against https://github.com/munificent/craftinginterpreters/wiki/Lox-implementations
