function _update_ps1() {
    # PS1="$(target/debug/opsline -theme gruvbox -hostname-only-if-ssh -mode patched -cwd-mode dironly -modules venv,host,ssh,cwd,perms,git,hg,jobs,exit,root,kube)"
    PS1="$(~/dev/opsline/target/debug/opsline --shell bash --segment-cwd dironly --segment-kube --segment-git --segment-root)"

    # Uncomment the following line to automatically clear errors after showing
    # them once. This not only clears the error for powerline-go, but also for
    # everything else you run in that shell. Don't enable this if you're not
    # sure this is what you want.

    #set "?"
}

if [ "$TERM" != "linux" ] && [ -f "$GOPATH/bin/powerline-go" ]; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
