use std::{fs, error::Error, env, thread, time::Duration};

extern crate systemstat;
use systemstat::{System, Platform};

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut monitor_ram = false;
        let mut monitor_cpu = false;
        if args.len() < 2 {
            return Err("User did not specify what to monitor");
        }
        let length = args.len() - 1;
        for i in 0..length {
            if args[i] == "-r" {
                monitor_ram = true;
            }
            if args[i] == "-c" {
                monitor_cpu = true;
            }
        }
        return Ok(Config {monitor_ram, monitor_cpu})
    }
}


pub struct Config {
    pub monitor_ram: bool,
    pub monitor_cpu: bool,
}

pub struct MemData {
    pub total: u64,
    pub free: u64,
}

impl MemData {
    fn new(total_input : u64, free_input : u64) -> MemData {
        MemData {total : total_input, free : free_input}
    }

    pub fn get_ram() -> MemData {
        let sys = System::new();
        
        let output = match sys.memory() {
            Ok(mem) => {
                // gets the total and converts it into a gigabyte value
                let total_ram: u64 = systemstat::data::
                ByteSize::gb(mem.total.as_u64()).as_u64();
    
                // gets the free amount and converts into as gigabyte value
                let free_ram = (systemstat::data::ByteSize::gb
                    (mem.free.as_u64())).as_u64() ;
    
                MemData::new(total_ram, free_ram)
                }
            Err(mem) => {
                eprintln!("\nError Getting Memory: {}", mem);
                MemData::new(0, 0)
                }
            };
        output 
        }
    
}

pub struct CpuData {
    // percent out of 100%
    pub user  : u8,
    pub nice  : u8,
    pub system  : u8,
    pub interrupt : u8,
    pub idle  : u8,
}

impl CpuData {
    fn new (user  : u8, nice  : u8, system  : u8, interrupt : u8, idle  : u8) -> CpuData {
        CpuData {user : user, nice : nice, system : system, interrupt: interrupt, idle: idle } 
    }

    fn convert_to_percentage (input : &mut CpuData) -> &mut CpuData {
        input.user      = input.user * 0u8;
        // input.nice      : 100,
        // input.system    : 100,
        // input.interrupt : 100,
        // input.idle      : 100,
        input
    }

    pub fn get_cpu() -> CpuData {
        let sys = System::new();
        
        let output = match sys.cpu_load_aggregate() {
            Ok(cpu)=> {
                // Measuring CPU load
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                CpuData {
                    user      : (cpu.user      as u8) * 100,
                    nice      : (cpu.nice      as u8) * 100,
                    system    : (cpu.system    as u8) * 100,
                    interrupt : (cpu.interrupt as u8) * 100,
                    idle      : (cpu.idle      as u8) * 100,
                }
            },
            Err(x) =>  {
                eprintln!("\nCPU load: error: {}", x);
                    CpuData {
                    user      : 0,
                    nice      : 0,
                    system    : 0,
                    interrupt : 0,
                    idle      : 0,
                    }
                }
            };
            output
        }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_config() {
        let args = vec!["temp first arg, usually will be the file executable".to_string(), 
                        "-r".to_string(), 
                        "-c".to_string()];
        let test = Config { monitor_ram: true,
             monitor_cpu: true
            };
        assert_eq!(test.monitor_ram, Config::new(&args).unwrap().monitor_ram);
    }

    #[test]
    fn valid_file() {
        let output = MemData::get_ram();
        println!("{}", output.total);
        ()
    }

    #[test]
    fn valid_ram() {
        MemData::get_ram();
        ()
    }

    #[test]
    fn valid_cpu() {
        CpuData::get_cpu();
        ()
    }
    #[test]
    fn create_cpu_notif() {
    let display : CpuData = CpuData::get_cpu();
    libnotify::init("myapp").unwrap();
    let output_user = display.user.to_string();
    // Init libnotify
    // Create a new notification (doesn't show it yet)
    let n = libnotify::Notification::new(&output_user[..],
                                         Some(""),
                                         None);

    // Show the notification
    n.show().unwrap();
    libnotify::uninit();
    }

    #[test]
    fn create_ram_notif() {
    let display : MemData = get_ram();
    libnotify::init("myapp").unwrap();
    let output_user = display.total.to_string();
    // Init libnotify
    // Create a new notification (doesn't show it yet)
    let message = String::from("There are : ") + &output_user + &String::from(" bytes of RAM in use");

    let n = libnotify::Notification::new(&message[..],
                                         Some(""),
                                         None);

    // Show the notification
    n.show().unwrap();
    libnotify::uninit();
    }

    

}