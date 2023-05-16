# nu_plugin_hcl

[nushell]: https://www.nushell.sh/
[plugin]: https://www.nushell.sh/contributor-book/plugins.html
[structured types]: https://www.nushell.sh/book/types_of_data.html

This is a [nushell] [plugin] to parse Hashicorp Configuration Language files into `nu` structured types.

# Installing

[add the plugin]: https://www.nushell.sh/book/plugins.html#adding-a-plugin
[`register`]: https://www.nushell.sh/book/commands/register.html

To [add the plugin] permanently, just install it and call [`register`] on it:

## Using Cargo

```bash
cargo install nu_plugin_hcl
register ~/.cargo/bin/nu_plugin_hcl
```

## Usage

```bash
open -r main.tf | from hcl | first 10
╭──────────┬──────────────────╮
│ provider │ {record 1 field} │
│ resource │ {record 1 field} │
╰──────────┴──────────────────╯
```
