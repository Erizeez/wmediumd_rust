#!/bin/bash

sudo iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo iwconfig wlan1 mode ad-hoc essid "wifi-adhoc"


sudo ifconfig wlan1 192.168.2.1 up
sleep 20
sudo ifconfig wlan0 192.168.2.2 up


# nohup gnome-terminal --disable-factory -- sudo ip netns exec ns0 bash &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec ns1 bash &

