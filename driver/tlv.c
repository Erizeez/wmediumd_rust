#include <linux/kernel.h>
#include <linux/module.h>

static inline int hwsim_nlmsg_parse(const struct nlattr *head, int len,
                                    struct nlattr *tb[], int maxtype,
                                    const struct nla_policy *policy)
{
    return __nla_parse(tb, maxtype, head,
                       len, policy, NL_VALIDATE_STRICT,
                       NULL);
}

static struct nlattr **
hwsim_genl_family_rcv_msg_attrs_parse(const struct genl_family *family,
                                      const struct nlattr *head,
                                      int len)
{
    enum netlink_validation validate = ops->validate & no_strict_flag ? NL_VALIDATE_LIBERAL : NL_VALIDATE_STRICT;
    struct nlattr **attrbuf;
    int err;

    attrbuf = kmalloc_array(HWSIM_ATTR_MAX + 1,
                            sizeof(struct nlattr *), GFP_KERNEL);
    if (!attrbuf)
        return ERR_PTR(-ENOMEM);

    err = __nlmsg_parse(head, len, attrbuf, HWSIM_ATTR_MAX, hwsim_genl_policy,
                        validate);
    if (err)
    {
        kfree(attrbuf);
        return ERR_PTR(err);
    }
    return attrbuf;
}

EXPORT_SYMBOL(hwsim_genl_family_rcv_msg_attrs_parse);