#!/bin/bash

SESSION_NAME="mavinote"

tmux has-session -t $SESSION_NAME 2> /dev/null

if [ $? != 0 ]; then
    tmux new-session -d -s $SESSION_NAME

    tmux rename-window 'reax'
    tmux new-window -t $SESSION_NAME:1 -n 'build'
    tmux new-window -t $SESSION_NAME:2 -n 'backend'
    tmux new-window -t $SESSION_NAME:3 -n 'terminal'

    tmux select-window -t $SESSION_NAME:0
    tmux send-keys "cd reax" C-m
    tmux send-keys "nvim ." C-m

    tmux select-window -t $SESSION_NAME:1
    tmux send-keys "cd reax" C-m
    tmux send-keys "bash" C-m
    tmux send-keys "source ~/.cargo/env" C-m
    tmux split-window -h
    tmux select-pane -t 2
    tmux send-keys "cd backend" C-m
    tmux send-keys "cargo run" C-m

    tmux select-window -t $SESSION_NAME:2
    tmux send-keys "cd backend" C-m
    tmux send-keys "nvim ." C-m

    tmux select-window -t $SESSION_NAME:1
fi

tmux attach-session -t $SESSION_NAME
