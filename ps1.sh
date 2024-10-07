#!/bin/bash

# for testing purpose
function _update_ps1() {
    PS1="$(~/dev/opsline/target/debug/opsline --config ~/.config/opsline/opsline.yaml)"
}

if [ "$TERM" != "linux" ] && command -v opsline; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
