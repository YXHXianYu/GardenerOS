# 操作系统 实验4

> 21301114 俞贤皓
>
> 环境（实验0~2, 4）：Arch Linux 6.5.3-arch1-1
>
> 环境（实验3）：Ubuntu 22.04.3 LTS (WSL)

## 1. 实验步骤

### 1.0 备注

* 我在实验2中，使用了0.2.0-alpha.2版本的rustsbi-qemu完成了实验，所以之后的实验我也均用最新版的rustsbi-qemu完成。
  * 我会把我遇到与新版rustsbi-qemu有关的问题 **加粗**

### 1.1 实现应用程序

* 添加环境代码
  * <img src="./README/image-20231201111642153.png" alt="image-20231201111642153" style="zoom:67%;" />
* 添加应用程序代码
  * <img src="./README/image-20231201111702267.png" alt="image-20231201111702267" style="zoom:67%;" />
* 编译
  * <img src="./README/image-20231201111729744.png" alt="image-20231201111729744" style="zoom:67%;" />
* 测试应用程序
  * <img src="./README/image-20231201111811978.png" alt="image-20231201111811978" style="zoom:67%;" />

### 1.2 多道程序的加载

* 根据文档编写代码
  * <img src="./README/image-20231201113113456.png" alt="image-20231201113113456" style="zoom:67%;" />

### 1.3 任务的设计与实现

* 根据文档编写代码
  * <img src="./README/image-20231201114300640.png" alt="image-20231201114300640" style="zoom:67%;" />

### 1.4 实现sys_yield与sys_exit的系统调用

* 根据文档编写代码
  * <img src="./README/image-20231201114624070.png" alt="image-20231201114624070" style="zoom:67%;" />

### 1.5 修改其他部分的代码

* 根据文档编写代码
  * <img src="./README/image-20231201115222850.png" alt="image-20231201115222850" style="zoom:67%;" />

### 1.6 结果

* 成功！
  * <img src="./README/image-20231201115458801.png" alt="image-20231201115458801" style="zoom:67%;" />

## 2. 思考问题

### 2.1 应用程序的加载

* `user/build.py`
  * 该脚本实现了多次cargo build特定应用程序，并且将每个程序的入口地址以 0x20000 为间隔设置。
* `user/Makefile`
  * 该脚本与旧版的区别在于，该脚本调用了 `build.py` 进行编译

### 2.2 多道程序的设计与实现



### 2.3 多道程序操作系统中的任务，它与进程和线程有什么区别和联系



## 3. Git提交截图

* [仓库链接](https://github.com/YXHXianYu/GardenerOS)
* 

## 4. 其他说明
