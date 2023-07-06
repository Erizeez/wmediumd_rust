#!/bin/bash

# 定义函数，用于清除之前的修改
function cleanup {
    sudo ip netns delete ns0
    sudo ip netns delete ns1
    sudo rmmod mac80211_hwsim
    sudo modprobe -r mac80211_hwsim
}

# 注册清除函数，当脚本退出时自动执行
trap cleanup EXIT

# 创建两个网络命名空间
sudo ip netns add ns0
sudo ip netns add ns1

sudo ip link add veth1 type veth peer name veth2
sudo ip link set veth1 netns ns0
sudo ip link set veth2 netns ns1

sudo ip netns exec ns0 ip addr add 192.168.1.1/24 dev veth1
sudo ip netns exec ns1 ip addr add 192.168.1.2/24 dev veth2

sudo ip netns exec ns0 ip link set veth1 up
sudo ip netns exec ns1 ip link set veth2 up


nohup gnome-terminal --disable-factory -- sudo ip netns exec ns0 bash &
nohup gnome-terminal --disable-factory -- sudo ip netns exec ns1 bash &

# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done