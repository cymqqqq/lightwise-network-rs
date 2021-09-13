
use std::collections::HashMap;
mod network {
    pub trait SockUtilTrait {
        fn connect(host: String, port: usize, b_async: bool, local_ip: String, local_port: usize) -> usize;
        fn listen(port: usize, local_ip: String, back_log: usize) -> usize;
        fn bind_udp_sock(port: usize, local_ip: String) -> usize;
        fn bind_sock(sock_fd: usize, local_ip: String, port: usize) -> usize;
        fn set_no_delay(sock_fd: usize, on: bool) -> usize;
        fn set_no_sigpipe(sock: usize) -> usize;
        fn set_no_blocked(sock: usize, noblock: bool) -> usize;
        fn set_recv_buf(sock: usize, size: usize) -> usize;
        fn set_send_buf(sock: usize, size: usize) -> usize;
        fn set_resuseable(sock_fd: usize, on: bool) -> usize;
        fn set_broadcast(sock_fd: usize, on: bool) -> usize;
        fn set_keep_alive(sock_fd: usize, on: bool) -> usize;
        fn set_multi_ttl(sock_fd: usize, ttl: usize) -> usize;
        fn set_multi_if(sock_fd: usize, str_local_ip: String) -> usize;
        fn set_multi_loop(sock_fd: usize, b_accept: bool) -> usize;
        fn join_multi_addr(sock_fd: usize, str_addr: String, str_local_ip: String) -> usize;
        fn leave_multi_addr(sock_fd: usize, str_addr: String, str_local_ip: String) -> usize;
        fn join_multi_addr_filter(sock_fd: usize, str_addr: String, str_src_ip: String, str_local_ip: String) -> usize;
        fn leave_multi_addr_filter(sock_fd: usize, str_addr: String, str_src_ip: String, str_local_ip: String) -> usize;
        fn get_sock_error(sock_fd: usize) -> usize;
        fn set_close_wait(sock_fd: usize, second: usize) -> usize;
        //fn getInterfaceList(&self) -> Vec<HashMap<String, String>>;
        fn get_local_ip(fd: usize) -> String;
        fn get_local_ip_v2(&self) -> String;
        fn get_local_port(fd: usize) -> usize;
        fn get_peer_ip(fd: usize) -> String;
        fn get_peer_port(fd: usize) -> usize;
        fn get_ifr_ip(ifr_name: String) -> String;
        fn get_ifr_name(localip: String) -> String;
        fn get_ifr_mask(ifr_name: String) -> String;
        fn get_ifr_brdaddr(ifr_name: String) -> String;
        fn is_same_lan(ip: String, dsr_ip: String) -> bool;
    }
}
