
use port_scanner::scan_ports_range;


use std::process::Command;


pub fn run_main() -> Result<(), Box<dyn std::error::Error>> {

  let mut free_qmp_port_num: u16 = 4000;
  for open_port in scan_ports_range(4000..9000) {
    free_qmp_port_num = open_port;
    break;
  }
  println!("Using port {:?} for controlling the VM", free_qmp_port_num);

  let t_free_qmp_port_num = free_qmp_port_num;
  std::thread::spawn(move || {
    control_vm_t(t_free_qmp_port_num);
  });

  Command::new("qemu-system-x86_64")
      .args([
        "-bios", "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd",
        // "-drive", "format=qcow2,file=/mnt/scratch/vms/enice-win11/WinDev2401Eval.qcow2",
        "-enable-kvm",
        "-m", "8200M",
        "-cpu", "host",
        "-smp", "2",
        "-machine", "type=pc,accel=kvm,kernel_irqchip=on",
        "-nic", "user,id=winnet0,id=mynet0,net=192.168.90.0/24,dhcpstart=192.168.90.10",
        "-net", "nic,model=virtio",
        "-boot", "c",
        "-vga", "virtio",
        "-display", "gtk,gl=on",
        "-qmp", "tcp:localhost:4444,server,wait=on"
      ])
      .status()?;


  Ok(())
}

pub fn control_vm_t(free_qmp_port_num: u16) {
  use qapi::{qga, Qga};
  let socket_addr = format!("127.0.0.1:{}", free_qmp_port_num);
  loop {
    std::thread::sleep(std::time::Duration::from_millis(250));
    match std::net::TcpStream::connect(&socket_addr) {
      Err(e) => {
        eprintln!("Waiting for QEMU: {:?}", e);
      }
      Ok(tcp_socket) => {
        eprintln!("Found QEMU at {:?}", &socket_addr);

        let mut qga = qapi::Qga::from_stream(&tcp_socket);

        let sync_value = &tcp_socket as *const _ as usize as i32;
        if let Err(e) = qga.guest_sync(sync_value) {
          eprintln!("Handshake failed: {:?}", e);
          continue;
        }

        let info = qga.execute(&qapi::qga::guest_info { }).unwrap();
        println!("Guest Agent version: {}", info.version);


        break;
      }
    }
  }
}




