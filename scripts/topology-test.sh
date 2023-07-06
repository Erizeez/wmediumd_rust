
nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-2 iperf3 -s &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-4 iperf3 -s &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-6 iperf3 -s &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-8 iperf3 -s &

sleep 5

nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-1 iperf3 -c 192.168.1.2 -t 60 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-3 iperf3 -c 192.168.2.2 -t 60 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-5 iperf3 -c 192.168.3.2 -t 60 &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec wmediumd-net-7 iperf3 -c 192.168.4.2 -t 60 &

# 等待用户输入ctrl-c，防止脚本退出
echo "Press Ctrl-C to exit"
while true; do sleep 1; done