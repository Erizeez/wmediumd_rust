#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>

#define DEV_NAME "/dev/phy12"
#define MEM_SIZE 4096

int main()
{
    int fd;
    char *mem;
    int ret;

    // 打开设备文件
    fd = open(DEV_NAME, O_RDWR);
    if (fd == -1)
    {
        perror("open");
        exit(1);
    }

    // // 映射一段 4096 字节内存
    mem = mmap(NULL, MEM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (mem == MAP_FAILED)
    {
        perror("mmap");
        exit(1);
    }

    printf("Read from mmap: %s\n", mem);

    // // 将字符串 "Hello, mmap!" 写入设备内存
    // snprintf(mem, 12, "Hello, mmap!");

    // 输出读取到的映射内存内容
    printf("Read from mmap: %s\n", mem);

    // 解除内存映射
    ret = munmap(mem, MEM_SIZE);
    if (ret == -1)
    {
        perror("munmap");
        exit(1);
    }

    // 关闭设备文件
    close(fd);

    return 0;
}