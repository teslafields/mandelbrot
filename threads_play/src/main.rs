extern crate libc;
extern crate signal_hook;

use std::fs::File;
use std::io;
use std::time;
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread::{sleep, spawn, JoinHandle};
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;


#[derive(Debug)]
struct CpuFreq {
    governor: String,
    driver: String,
    freq: f64
}

#[derive(Debug)]
struct CpuInfo {
    online: String,
    cpufreq: Vec<CpuFreq>
}

fn cpu_reader_thread(run_flag: Arc<RwLock<bool>>)
    -> (Receiver<CpuInfo>, JoinHandle<io::Result<()>>)
{
    let (tx, rx) = sync_channel(1);
    let handle = spawn(move || {
        let seconds = time::Duration::new(1, 0);
        while *run_flag.read().unwrap() {
            let mut cpufreq = Vec::new();
            cpufreq.push(CpuFreq { governor: String::new(),
                    driver: String::new(), freq: 0.0 });
            let mut cpuinf = CpuInfo{ online: String::new(), cpufreq };
            let mut f = File::open("/sys/devices/system/cpu/online")?;
            f.read_to_string(&mut cpuinf.online)?;
            if let Some(err) = tx.send(cpuinf).err() {
                println!("Error in tx: {:?}", err);
            }

            sleep(seconds);

        }
        Ok(())
    });
    (rx, handle)
}

fn display_info_thread(run_flag: Arc<RwLock<bool>>, rx: Receiver<CpuInfo>)
        ->  JoinHandle<()> {
    let handle = spawn(move || {
        while *run_flag.read().unwrap() {
            let datain: CpuInfo = rx.recv().unwrap();
            println!("Online CPUs: {:?}", datain.online.trim());
        }
    });
    handle
}

fn handle_incoming_signal(run_flag: Arc<RwLock<bool>>) -> Result<(), Error> {
    let mut signals = Signals::new(&[

        SIGHUP,
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ])?;
    'outer: loop {
        for signal in signals.pending() {
            match signal as libc::c_int {
                SIGHUP | SIGTERM | SIGINT | SIGQUIT => {
                    {
                        println!("Received signal {:?}", signal);
                        let mut flag = run_flag.write().unwrap();
                        *flag = false;
                    }
                    break 'outer;
                },
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let run_flag: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    let (rx, h1) = cpu_reader_thread(Arc::clone(&run_flag));
    let h2 = display_info_thread(Arc::clone(&run_flag), rx);
    handle_incoming_signal(Arc::clone(&run_flag))?;
    let _ = h1.join().unwrap();
    let _ = h2.join().unwrap();
    println!("Terminating. Bye bye");
    Ok(())
}
