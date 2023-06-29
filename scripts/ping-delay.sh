#!/bin/bash

# 设置参数
target="192.168.1.2"  # 替换为你要 ping 的目标主机
duration=20  # 统计延迟的时间长度（单位：秒）
interval=1  # 每次 ping 的间隔时间（单位：秒）

# 初始化变量
sum=0
count=0

# 循环执行 ping 命令并累加延迟
for ((i=0; i<duration; i+=interval)); do
    result=$(ping -c 1 -W 1 $target | grep "time=" | awk -F'=' '{print $4}' | awk '{print $1}')
    if [[ -n $result ]]; then
        sum=$(echo "$sum + $result" | bc)
        count=$((count + 1))
        echo "当前延迟：$result ms"
    fi
    sleep $interval
done

# 计算平均延迟
if [[ $count -gt 0 ]]; then
    average=$(echo "scale=2; $sum / $count" | bc)
    echo "平均延迟：$average ms"
else
    echo "无法获取延迟数据"
fi
