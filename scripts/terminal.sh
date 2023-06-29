#!/bin/bash

nohup gnome-terminal --disable-factory -- sudo ip netns exec ns0 bash &
# nohup gnome-terminal --disable-factory -- sudo ip netns exec ns1 bash &