## post

Example usage:

```bash
post "note text"
> posted !

post view
> entry 0
> entry 1
> ...
> entry 10

post view --top=2
> entry 0
> entry 1

post view --tail=2
> entry n-1
> entry n

post yank 
> yanked entry 0 to clipboard

post yank [--target, -t] 2
> yanked entry 2 to clipboard

post pop [--target, -t] 2
> yanked and deleted entry 2 to clipboard

post delete [--target, -t] 2
> deleted entry 2

post clear
> deleted all entries

post clear --top=3
> deleted the latest 3 entries

post clear --tail=3
> deleted the oldest 3 entries

```



