# ansi

This is a project where I attempt to recreate the ansi markup language I made in Python, but in Rust. The idea is that you use `[]` to denote
macros and styling. `*` and `_` represent bold and underline respectively. `@` denotes a color macro, `~` a hyperlink macro, and `$` a global
character escape macro. All macros continue until the end of the string or until the are closed with the same symbol they were opened with.
Color macros (`@`) have the most complex syntax. `@` followed by `F` represents foreground/text color. If it is instead followed by `B` it
represents background color. If neither `F` or `B` are provided then the color applies to both text and background. After these characters
you define a color. It can be one of the system colors: `black`, `red`, `green`, `yellow`, `blue`, `cyan`, `magenta`, or `white`. Beyond this
you can define a XTerm code, rgb code, or a hex code. XTerm codes are `0-255`, rgb codes are `r,g,b` with each of `r`, `g`, and `b` being a
value `0-255`. Hex codes are prefixed with `#` and can be either 3 characters or 6 `#FFF` or `#FFFFFF`. Urls are a simlier syntax. You just
put the url you want to use for the hyperlink after the `~` character. This will look something like `~https://example.com`. Global escape
macros take no additional syntax. All macros ignore whitespace except for the `F` or `B` that follow the color (`@`) symbol.
