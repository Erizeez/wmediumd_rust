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

# 加载mac80211_hwsim模块，创建两个radio
sudo modprobe mac80211
cd ../driver
sudo insmod mac80211_hwsim.ko

sleep 5

# 创建两个网络命名空间
sudo ip netns add ns0
sudo ip netns add ns1

sudo iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo iwconfig wlan1 mode ad-hoc essid "wifi-adhoc"

phy_names=$(iw dev | grep -o "phy#[0-9]*" | awk -F# '{print $2}')
echo $phy_names;
IFS=' ' read -r -a array <<< $(echo $phy_names)
phy0="phy"${array[1]}
phy1="phy"${array[0]}
echo $phy0
echo $phy1
sudo iw phy $phy0 set netns name ns0
sudo iw phy $phy1 set netns name ns1

#sudo ifconfig wlan1 192.168.1.2 up
#sudo ifconfig wlan0 192.168.1.1 up
sudo ip netns exec ns0 ifconfig wlan0 192.168.1.1 up
sleep 12

sudo ip netns exec ns1 ifconfig wlan1 192.168.1.2 up

nohup gnome-terminal --disable-factory -- sudo ip netns exec ns0 bash &
nohup gnome-terminal --disable-factory -- sudo ip netns exec ns1 iperf3 -s &

# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done
