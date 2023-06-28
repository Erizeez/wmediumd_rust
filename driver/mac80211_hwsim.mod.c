#include <linux/module.h>
#define INCLUDE_VERMAGIC
#include <linux/build-salt.h>
#include <linux/elfnote-lto.h>
#include <linux/vermagic.h>
#include <linux/compiler.h>

BUILD_SALT;
BUILD_LTO_INFO;

MODULE_INFO(vermagic, VERMAGIC_STRING);
MODULE_INFO(name, KBUILD_MODNAME);

__visible struct module __this_module
__section(".gnu.linkonce.this_module") = {
	.name = KBUILD_MODNAME,
	.init = init_module,
#ifdef CONFIG_MODULE_UNLOAD
	.exit = cleanup_module,
#endif
	.arch = MODULE_ARCH_INIT,
};

#ifdef CONFIG_RETPOLINE
MODULE_INFO(retpoline, "Y");
#endif

static const struct modversion_info ____versions[]
__used __section("__versions") = {
	{ 0x76a006e9, "module_layout" },
	{ 0xcf9c4c64, "param_ops_bool" },
	{ 0x292ca627, "param_ops_int" },
	{ 0xf7deefc2, "simple_attr_release" },
	{ 0xedd8a2eb, "debugfs_attr_write" },
	{ 0x628d0c83, "debugfs_attr_read" },
	{ 0x67558d0a, "no_llseek" },
	{ 0xb36f06, "eth_validate_addr" },
	{ 0x209ac09, "eth_mac_addr" },
	{ 0xcb91a8ee, "unregister_netdev" },
	{ 0x2a5ea9ef, "rhashtable_destroy" },
	{ 0xa77903e3, "unregister_pernet_device" },
	{ 0x17620892, "platform_driver_unregister" },
	{ 0xdf54a8f7, "netlink_unregister_notifier" },
	{ 0xf031d858, "unregister_virtio_driver" },
	{ 0x9d374e15, "free_netdev" },
	{ 0x6e720ff2, "rtnl_unlock" },
	{ 0xc3b5f63e, "register_netdevice" },
	{ 0x2335b3c9, "dev_alloc_name" },
	{ 0xc7a4fbed, "rtnl_lock" },
	{ 0x9d2c9bd2, "alloc_netdev_mqs" },
	{ 0xa5296a4f, "__class_create" },
	{ 0x6abef229, "register_virtio_driver" },
	{ 0x1c35ccac, "genl_unregister_family" },
	{ 0xfa599bb2, "netlink_register_notifier" },
	{ 0x760f19ff, "genl_register_family" },
	{ 0x6f821d, "__platform_driver_register" },
	{ 0x2a4b6f99, "register_pernet_device" },
	{ 0x4b5acf74, "rhashtable_init" },
	{ 0xb202f0d7, "rht_bucket_nested_insert" },
	{ 0xe0313d71, "rhashtable_insert_slow" },
	{ 0x1c646129, "debugfs_create_file" },
	{ 0x46860613, "debugfs_create_dir" },
	{ 0xdc2cc4a7, "regulatory_hint" },
	{ 0xa8a6235e, "ieee80211_register_hw" },
	{ 0x2d0684a9, "hrtimer_init" },
	{ 0x54496b4, "schedule_timeout_interruptible" },
	{ 0xfd6ccf2b, "wiphy_apply_custom_regulatory" },
	{ 0xcefb0c9f, "__mutex_init" },
	{ 0xc6f46339, "init_timer_key" },
	{ 0xffeedf6a, "delayed_work_timer_fn" },
	{ 0x52855641, "device_bind_driver" },
	{ 0xf855a41b, "device_create" },
	{ 0xcb09cd31, "ieee80211_alloc_hw_nm" },
	{ 0x9eacf8a5, "kstrndup" },
	{ 0xe2d5255a, "strcmp" },
	{ 0x37a0cba, "kfree" },
	{ 0x1afa94e0, "virtqueue_get_vring_size" },
	{ 0x41ed3709, "get_random_bytes" },
	{ 0x29c346, "ieee80211_probereq_get" },
	{ 0x3ac6c2d6, "ieee80211_tx_prepare_skb" },
	{ 0x4b3f4342, "ieee80211_csa_finish" },
	{ 0xaf9e2c3a, "ieee80211_beacon_cntdwn_is_complete" },
	{ 0x43927a26, "ieee80211_get_buffered_bc" },
	{ 0x23ae031e, "ieee80211_beacon_get_tim" },
	{ 0x22d91210, "ieee80211_get_tx_rates" },
	{ 0x5ef6d9bb, "skb_queue_tail" },
	{ 0xed5d64c4, "nla_put_64bit" },
	{ 0x500ceea8, "virtqueue_add_outbuf" },
	{ 0xee7525e7, "virtqueue_kick" },
	{ 0x9fbb8c6f, "virtqueue_add_inbuf" },
	{ 0xb320cc0e, "sg_init_one" },
	{ 0xffb7c514, "ida_free" },
	{ 0xd0d156e9, "__rht_bucket_nested" },
	{ 0xf3baf8dc, "__free_pages" },
	{ 0x2e5fe036, "__skb_ext_put" },
	{ 0x7098b464, "skb_add_rx_frag" },
	{ 0xfc8f970c, "alloc_pages" },
	{ 0x296695f, "refcount_warn_saturate" },
	{ 0x42cb2fe4, "dst_release" },
	{ 0x99861cf3, "skb_copy" },
	{ 0xd2800691, "nf_conntrack_destroy" },
	{ 0xb788fb30, "gic_pmr_sync" },
	{ 0x7c5ca732, "virtqueue_get_buf" },
	{ 0x56470118, "__warn_printk" },
	{ 0x4b0a3f52, "gic_nonsecure_priorities" },
	{ 0x3c5d543a, "hrtimer_start_range_ns" },
	{ 0x15ba50a6, "jiffies" },
	{ 0xe25c0af1, "class_destroy" },
	{ 0x9c1e5bf5, "queued_spin_lock_slowpath" },
	{ 0x7b4627a9, "cpu_hwcap_keys" },
	{ 0x14b89635, "arm64_const_caps_ready" },
	{ 0xb0c467f1, "skb_push" },
	{ 0xa6c0c756, "skb_copy_expand" },
	{ 0x837b7b09, "__dynamic_pr_debug" },
	{ 0x3afa0fb0, "ieee80211_tx_status_irqsafe" },
	{ 0x75dec8ed, "ieee80211_rx_irqsafe" },
	{ 0x1267f683, "skb_unlink" },
	{ 0xf1db1704, "nla_memcpy" },
	{ 0xc4f0da12, "ktime_get_with_offset" },
	{ 0x1f857679, "ether_setup" },
	{ 0x5487440f, "netlink_unicast" },
	{ 0x6d2fc5a6, "net_namespace_list" },
	{ 0xc5b6f236, "queue_work_on" },
	{ 0x2d3385d3, "system_wq" },
	{ 0x69fd2608, "ieee80211_free_hw" },
	{ 0xe2c86d6e, "device_unregister" },
	{ 0x2f100592, "device_release_driver" },
	{ 0x473ec7d5, "ieee80211_unregister_hw" },
	{ 0x5ed5f867, "debugfs_remove" },
	{ 0xdcbffd89, "__alloc_skb" },
	{ 0x828e22f4, "hrtimer_forward" },
	{ 0xb7f990e9, "rht_bucket_nested" },
	{ 0x449ad0a7, "memcmp" },
	{ 0x92997ed8, "_printk" },
	{ 0x1d24c881, "___ratelimit" },
	{ 0xe6d2458e, "do_trace_netlink_extack" },
	{ 0x8f1dc23d, "netlink_broadcast" },
	{ 0xdd3d1846, "init_net" },
	{ 0xab2a3d81, "genl_notify" },
	{ 0xa648e561, "__ubsan_handle_shift_out_of_bounds" },
	{ 0xc56a41e6, "vabits_actual" },
	{ 0x76a76b30, "ieee80211_remain_on_channel_expired" },
	{ 0xc1a5b9e6, "netif_rx" },
	{ 0x33859ee1, "skb_put" },
	{ 0xdc8bdbc5, "__netdev_alloc_skb" },
	{ 0x4829a47e, "memcpy" },
	{ 0x576ffa26, "skb_trim" },
	{ 0x9df0af7d, "genlmsg_put" },
	{ 0xdcb764ad, "memset" },
	{ 0x98cf60b3, "strlen" },
	{ 0x54b1fac6, "__ubsan_handle_load_invalid_value" },
	{ 0x3ac84f9, "ieee80211_free_txskb" },
	{ 0xd2e6565b, "skb_dequeue" },
	{ 0x46a4b118, "hrtimer_cancel" },
	{ 0x25a78d62, "ieee80211_stop_tx_ba_cb_irqsafe" },
	{ 0x87a21cb3, "__ubsan_handle_out_of_bounds" },
	{ 0x37befc70, "jiffies_to_msecs" },
	{ 0x7d95ae5a, "ieee80211_scan_completed" },
	{ 0x9fa7184a, "cancel_delayed_work_sync" },
	{ 0x7f02188f, "__msecs_to_jiffies" },
	{ 0x35706a33, "ieee80211_ready_on_channel" },
	{ 0x5f351c5b, "ieee80211_queue_delayed_work" },
	{ 0x3213f038, "mutex_unlock" },
	{ 0x4dfa8d4b, "mutex_lock" },
	{ 0x8da6585d, "__stack_chk_fail" },
	{ 0x21c4236d, "cfg80211_vendor_cmd_reply" },
	{ 0xb87cdd9a, "__cfg80211_alloc_reply_skb" },
	{ 0x7867152c, "__cfg80211_send_event_skb" },
	{ 0xfacd8c28, "nla_put" },
	{ 0xa2f9b886, "__cfg80211_alloc_event_skb" },
	{ 0x420964e3, "__nla_parse" },
	{ 0xe91d09ae, "ieee80211_iterate_active_interfaces_atomic" },
	{ 0x3c3fce39, "__local_bh_enable_ip" },
	{ 0x8d9c6764, "ieee80211_radar_detected" },
	{ 0x3b7cc2fe, "simple_attr_open" },
	{ 0xbf9de55e, "__dynamic_dev_dbg" },
	{ 0x3c12dfe, "cancel_work_sync" },
	{ 0x9f47216b, "virtqueue_detach_unused_buf" },
	{ 0x87770ada, "kfree_skb_reason" },
	{ 0xe7a02573, "ida_alloc_range" },
	{ 0x2d5f69b3, "rcu_read_unlock_strict" },
	{ 0x4d12b0f5, "consume_skb" },
};

MODULE_INFO(depends, "cfg80211,mac80211");

MODULE_ALIAS("virtio:d0000001Dv*");

MODULE_INFO(srcversion, "318D4AEB502F52EF8263EFC");
