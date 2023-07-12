#ifndef __MAC80211_HWSIM_TLV_H
#define __MAC80211_HWSIM_TLV_H

// Allocate buffer for TLV, 1 page size default

// Parse TLVs from shared memory, need to copy to local buffer first

// Parse TLVs, reuse /linux/lib/nlattr.c
static struct nlattr **
hwsim_genl_family_rcv_msg_attrs_parse(const struct genl_family *family,
                                      const struct nlattr *head,
                                      int len);
// Unparse TLVs, recv shared memory ponter to copy to

EXPORT_SYMBOL(hwsim_genl_family_rcv_msg_attrs_parse);

#endif /* __MAC80211_HWSIM_H */