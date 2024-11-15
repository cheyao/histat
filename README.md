# Histat

The history sumarizer for zsh

```zsh
$ histat
579 ls
447 cd
426 git
319 sudo
295 ninja
```

## Usage

```
$ histat --help
Zsh history stats

Usage: histat [OPTIONS]

Options:
  -f, --file <FILE>    [default: ~/.zsh_history]
  -l, --lines <LINES>  [default: 5]
  -h, --help           Print help
  -V, --version        Print version
```

NOTE: The history file defaults to `~/.zsh_history` because there is no way to access the $HISTFILE shell variable
