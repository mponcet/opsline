# opsline

`opsline` is a powerline style prompt for your shell, inspired by 
[powerline-go](https://github.com/justjanne/powerline-go)

## Install

```
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`


### Bash

Add the following to your `.bashrc` :

```
function _update_ps1() {
    PS1="$(opsline --shell bash --config ~/.config/opsline/opsline.yaml)"
}

if [ "$TERM" != "linux" ] && command -v opsline 2>&1 >/dev/null; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
```

### Zsh

Add the following to your `.zshrc` :

```
function _update_ps1() {
    PS1="$(opsline --shell zsh --config ~/.config/opsline/opsline.yaml)"
}

if [ "$TERM" != "linux" ] && command -v opsline 2>&1 >/dev/null; then
    precmd_functions+=(_update_ps1)
fi
```

## Configuration

### Example

```
theme: gruvbox
segments:
  - cwd
  - readonly
  - git
  - kube
  - containers
  - root
cwd:
  dironly: true
kube:
  critical_contexts:
    - kind-prod
  context_aliases:
    - context: kind-prod
      alias: prod
containers:
  url: unix:/run/user/1000/podman/podman.sock
```

## Documentation

[Bash tips: Colors and formatting (ANSI/VT100 Control sequences)](https://misc.flogisoft.com/bash/tip_colors_and_formatting)

[Nerd Fonts](https://www.nerdfonts.com/)
