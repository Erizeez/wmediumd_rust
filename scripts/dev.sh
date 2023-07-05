#!/bin/bash

nohup gnome-terminal --disable-factory -- top &
nohup gnome-terminal --disable-factory -- watch -n 1 "sudo dmesg | tail -n 20" &
nohup gnome-terminal --disable-factory -- ./topology-env.sh &
nohup gnome-terminal --disable-factory &


# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done