# personalized PS1 prompt
PS1="\W \e[01;31m$\e[m "

# If not running interactively, don't do anything
case $- in
    *i*) ;;
      *) return;;
esac

# don't put duplicate lines or lines starting with space in the history.
# See bash(1) for more options
HISTCONTROL=ignoreboth

# append to the history file, don't overwrite it
shopt -s histappend

# for setting history length see HISTSIZE and HISTFILESIZE in bash(1)
HISTSIZE=1000
HISTFILESIZE=2000

# custom
alias python="python3"
alias l="ls -lah"
alias ..="cd .."

# git aliases
alias gs="git status"
alias gc="git commit -m "
alias ga="git add "
alias gp="git push"
alias gpl="git pull"
alias gl="git log --pretty=oneline"

# check the window size after each command and, if necessary
# update the values of LINES and COLUMNS.
shopt -s checkwinsize

# make less more friendly for non-text input files, see lesspipe(1)
[ -x /usr/bin/lesspipe ] && eval "$(SHELL=/bin/sh lesspipe)"


# Alias definitions in bash_aliases
if [ -f ~/work/scripts/.bash_aliases ]; then
    . ~/work/scripts/.bash_aliases
fi

# enable programmable completion features (you don't need to enable
# this, if it's already enabled in /etc/bash.bashrc and /etc/profile
# sources /etc/bash.bashrc).
if ! shopt -oq posix; then
  if [ -f /usr/share/bash-completion/bash_completion ]; then
    . /usr/share/bash-completion/bash_completion
  elif [ -f /etc/bash_completion ]; then
    . /etc/bash_completion
  fi
fi


export PATH=$PATH:/usr/local/go/bin:/usr/go/bin:/home/dev/go/bin/:/home/dev/.local/bin
