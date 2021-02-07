# Draw a box

This crate contains constants for all 128 [unicode box-drawing characters](https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)) and makes it possible to find the one you need dynamically.

## Constants

The constants are separated into modules:

- `arc`
    - `╭`, `╮`, `╯`, `╰`
- `dashed`
    - `╌`, `╍`, `╎`, `╏`, `┄`, `┅`, `┆`, `┇`, `┈`, `┉`, `┊`, `┋`
- `diagonal`
    - `╱`, `╲`, `╳`
- `double`
    - `═`, `║`, `╒`, `╓`, `╔`, `╕`, `╖`, `╗`, `╘`, `╙`, `╚`, `╛`, `╜`, `╝`, `╞`, `╟`, `╠`, `╡`, `╢`, `╣`, `╤`, `╥`, `╦`, `╧`, `╨`, `╩`, `╪`, `╫`, `╬`
- `heavy`
    - `━`, `┃`, `┏`, `┓`, `┗`, `┛`, `┣`, `┫`, `┳`, `┻`, `╋`, `╸`, `╹`, `╺`, `╻`
- `light`
    - `─`, `│`, `┌`, `┐`, `└`, `┘`, `├`, `┤`, `┬`, `┴`, `┼`, `╴`, `╵`, `╶`, `╷`
- `light_and_heavy`
    - `┍`, `┎`, `┑`, `┒`, `┕`, `┖`, `┙`, `┚`, `┝`, `┞`, `┟`, `┠`, `┡`, `┢`, `┥`, `┦`, `┧`, `┨`, `┩`, `┪`, `┭`, `┮`, `┯`, `┰`, `┱`, `┲`, `┵`, `┶`, `┷`, `┸`, `┹`, `┺`, `┽`, `┾`, `┿`, `╀`, `╁`, `╂`, `╃`, `╄`, `╅`, `╆`, `╇`, `╈`, `╉`, `╊`, `╼`, `╽`, `╾`, `╿`

## Dynamically choosing a character

When drawing something thats more than just a simple box, having a way to get a character depending on its properties.
Any character that's only made up of light or heavy lines can be found using the `find_codepoint` function.
The function takes a `Weight` for each direction and returns the character fitting these weights.
Given the weights `Empty` (up), `Empty` (right), `Light` (down), and `Light` (left) the character `┐` would be chosen.

```rust
use draw_a_box::{find_codepoint, Weight};

let character = find_codepoint(Weight::Light, Weight::Heavy, Weight::Light, Weight::Heavy);
assert_eq!(character, "┿");
```

## License

Licensed under MIT license.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as above, without any additional terms or conditions.
