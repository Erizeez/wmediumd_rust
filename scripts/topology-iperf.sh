sudo ip netns exec wmediumd-net-1 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-2 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-3 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-4 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-5 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-6 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-7 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"
sudo ip netns exec wmediumd-net-8 iwconfig wlan0 mode ad-hoc essid "wifi-adhoc"

sudo ip netns exec wmediumd-net-1 ifconfig wlan0 192.168.1.1 up
sudo ip netns exec wmediumd-net-3 ifconfig wlan0 192.168.2.1 up
sudo ip netns exec wmediumd-net-5 ifconfig wlan0 192.168.3.1 up
sudo ip netns exec wmediumd-net-7 ifconfig wlan0 192.168.4.1 up

sleep 12
sudo ip netns exec wmediumd-net-2 ifconfig wlan0 192.168.1.2 up
sudo ip netns exec wmediumd-net-4 ifconfig wlan0 192.168.2.2 up
sudo ip netns exec wmediumd-net-6 ifconfig wlan0 192.168.3.2 up
sudo ip netns exec wmediumd-net-8 ifconfig wlan0 192.168.4.2 up

sleep 10

nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-1 iperf3 -c 192.168.1.2 -t 60 -b 400M -p 30001 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-2 iperf3 -s -p 30001&
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-3 iperf3 -c 192.168.2.2 -t 60 -b 400M -p 30002 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-4 iperf3 -s -p 30002 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-5 iperf3 -c 192.168.3.2 -t 60 -b 400M -p 30003 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-6 iperf3 -s -p 30003 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-7 iperf3 -c 192.168.4.2 -t 60 -b 400M -p 30004 &
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-8 iperf3 -s -p 30004 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-1 ping 192.168.1.2 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-3 ping 192.168.2.2 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-5 ping 192.168.3.2 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-7 ping 192.168.4.2 &


# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done