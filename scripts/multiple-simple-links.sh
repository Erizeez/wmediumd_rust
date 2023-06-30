#!/bin/bash

# 定义函数，用于清除之前的修改
function cleanup {
    sudo ip netns delete ns00
    sudo ip netns delete ns01
    sudo ip netns delete ns10
    sudo ip netns delete ns11
    sudo rmmod mac80211_hwsim
    sudo modprobe -r mac80211_hwsim
}

# 注册清除函数，当脚本退出时自动执行
trap cleanup EXIT

# 先加载/lib/module里默认的mac80211模块,再通过insmod加载mac80211_hwsim模块
sudo modprobe mac80211
cd ../driver
sudo insmod mac80211_hwsim.ko radios=4

sleep 5

# 创建两个网络命名空间
sudo ip netns add ns00
sudo ip netns add ns01
sudo ip netns add ns10
sudo ip netns add ns11

# 配置ad-hoc模式
sudo iwconfig wlan0 mode ad-hoc essid "wifi-adhoc-0"
sudo iwconfig wlan1 mode ad-hoc essid "wifi-adhoc-0"
sudo iwconfig wlan2 mode ad-hoc essid "wifi-adhoc-1"
sudo iwconfig wlan3 mode ad-hoc essid "wifi-adhoc-1"

# wlan0 - 02:00:00:00:00:00 - 42:00:00:00:00:00
# wlan1 - 02:00:00:00:01:00 - 42:00:00:00:01:00
# wlan2 - 02:00:00:00:02:00 - 42:00:00:00:02:00
# wlan3 - 02:00:00:00:03:00 - 42:00:00:00:03:00

# 获取动态生成的phy列表
phy_names=$(iw dev | grep -o "phy#[0-9]*" | awk -F# '{print $2}')
echo $phy_names;
IFS=' ' read -r -a array <<< $(echo $phy_names)
phy0="phy"${array[3]}
phy1="phy"${array[2]}
phy2="phy"${array[1]}
phy3="phy"${array[0]}
echo $phy0
echo $phy1
echo $phy2
echo $phy3

sudo iw phy $phy0 set netns name ns00
sudo iw phy $phy1 set netns name ns01
sudo iw phy $phy2 set netns name ns10
sudo iw phy $phy3 set netns name ns11

sudo ip netns exec ns00 ifconfig wlan0 192.168.1.1 up
sudo ip netns exec ns10 ifconfig wlan2 192.168.2.1 up
sleep 14

sudo ip netns exec ns01 ifconfig wlan1 192.168.1.2 up
sudo ip netns exec ns11 ifconfig wlan3 192.168.2.2 up

nohup gnome-terminal --disable-factory --working-directory=/home/parallels/ns00 -- sudo ip netns exec ns00 bash &
nohup gnome-terminal --disable-factory --working-directory=/home/parallels/ns01 -- sudo ip netns exec ns01 bash &
nohup gnome-terminal --disable-factory --working-directory=/home/parallels/ns10 -- sudo ip netns exec ns10 bash &
nohup gnome-terminal --disable-factory --working-directory=/home/parallels/ns11 -- sudo ip netns exec ns11 bash &


# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done