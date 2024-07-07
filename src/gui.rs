
use std::process::Command;


pub fn run_main() -> Result<(), Box<dyn std::error::Error>> {

  std::thread::spawn(move || {
    control_vm_t();
  });

  Command::new("qemu-system-x86_64")
      .args([
        "-bios", "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd",
        "-drive", "format=qcow2,file=/mnt/scratch/vms/enice-win11/WinDev2401Eval.qcow2",
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
      ])
      .status()?;


  Ok(())
}

pub fn control_vm_t() {
  loop {
    println!("Control T");
    std::thread::sleep(std::time::Duration::from_millis(2400));
  }
}




