# rust-marks

Small bookmarks utility for the terminal. Navigate to common folders with ease.


# Usage

```
Usage:
    marks
    marks <bookmark>
    marks --add=<bookmark>
    marks --delete=<bookmark>
    marks --keys
    marks --check
    marks --clean
    marks --version
    marks --help

Options:
    -k, --keys               Show keys.
    -a, --add=<bookmark>     Add bookmark for current directory
    -d, --delete=<bookmark>  Delete bookmark.
    -h, --help               Show this message.
    --check                  Check for tags pointing to non.
    --clean                  Delete non existing bookmarks.
    --version                Print version information.

```


# ZSH integration

Add this code to `.zshrc`

```sh
g() {
    if [ $# -eq 0 ]; then
        rust-marks
    else
        case "$1" in

        -*)
            rust-marks "$@"
            ;;
        *)
            cd "$(rust-marks $@)"
            ;;
        esac
    fi
}
```

And add this to a file named `_g` inside a completion folder. for zsh.


```zsh
#compdef g

_arguments "1: :($(rust-marks --keys))"
_arguments "2: :($(rust-marks --keys))"
```

On my computer, I've added `~/.dotfiles/zshcomp` to this path like this:
```zsh
fpath=(~/.dotfiles/zshcomp $fpath)
```