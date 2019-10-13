use std::net::{IpAddr, SocketAddr};

fn main() {
    // 本地环回环地址
    // 127.0.0.1 ---> 127.255.255.254
    // 网络号全为0表示：该IP是一个保留IP，本网络（this）。
    // 主机号全为0表示：所有主机
    // 网络号为127（01111111）保留作为本地软件环回测试（loopback test）本主机的进程间的通信之用。
    // 若主机发送一个目的地址为环回地址的IP数据报，则本主机中的协议软件就处理该数据报中数据，而不会将数据报发送到任何网络。
    // 目的地址为环回地址的IP数据报永远不会出现在任何网络上，因为网络号为127的地址根本不是网络地址。
    let local: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(local.is_loopback());
    //assert_eq!(local.is_loopback(), true);

    println!("sasd");
}
