sudo ip netns exec wmediumd-net-1 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-2 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-3 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-4 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"

sudo ip netns exec wmediumd-net-1 ifconfig wlan0 192.168.1.1 up
sudo ip netns exec wmediumd-net-3 ifconfig wlan0 192.168.2.1 up

sleep 12
sudo ip netns exec wmediumd-net-2 ifconfig wlan0 192.168.1.2 up
sudo ip netns exec wmediumd-net-4 ifconfig wlan0 192.168.2.2 up


nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-1 bash &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-2 bash &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-3 bash &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-4 bash &

# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done