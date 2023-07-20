#!/bin/bash

# 定义函数，用于清除之前的修改
function cleanup {
    sudo rmmod mac80211_hwsim
    sudo modprobe -r mac80211
}

# 注册清除函数，当脚本退出时自动执行
trap cleanup EXIT

# 加载mac80211_hwsim模块，创建两个radio
sudo modprobe mac80211
cd ../driver-new
sudo insmod mac80211_hwsim.ko


# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done
