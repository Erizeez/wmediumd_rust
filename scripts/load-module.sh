#!/bin/bash

cd /usr/src/linux-source-5.15.0/linux-source-5.15.0/
cd lib/crypto
insmod libarc4.ko && cd ../..
cd net/wireless
insmod cfg80211.ko && cd ../..
cd net/mac80211
insmod mac80211.ko radios=2 && cd ../..