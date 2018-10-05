#!/bin/bash
[ -z "${TMUX}" ] && tmux;

tmux split-window "cd splash-web && ember s --proxy=http://localhost:8001";
tmux resize-pane -y 15
tmux split-window -h "cd splash-server && cargo run --bin run_server";
tmux select-pane -t $TMUX_PANE