use dbus::message::MatchRule;
use i3ipc_jl::{event::Event, reply::NodeType, I3Connection, I3EventListener, Subscription};
use std::{fs, thread, time::Duration};
use regex::Regex;
use std::{env, fs::File, io::BufWriter, process::Command};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use image::{ImageBuffer, RgbaImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    let param = &args[1];
    let args2 = if args.len() > 2 { vec![args[2].clone()] } else { vec![] };
    let args3 = if args.len() > 3 { vec![args[3].clone()] } else { vec![] };

    match param.as_str() {
        "renderws" => renderws(),
        "focusedws" => focusedws(),
        "urgentws" => urgentws(),
        "mode" => mode(),
        "title" => title(&args2),
        "icon_dump" => icon_dump(),
        "battery" => battery(&args2, &args3),
        "nm" => nm(&args2, &args3),
        _ => println!("unknown param"),
    }
}
fn nm(which: &[String], interval: &[String]) {
    let which = which.join(" ");
    let which = format!("{}",which.trim());
    fn get_network() {
        let cmd = Command::new("nmcli")
            .args(["-t", "-f", "TYPE,STATE,CONNECTION", "device"])
            .output()
            .expect("failed to execute nmcli");

        let net_prop = String::from_utf8_lossy(&cmd.stdout)
            .lines()
            .nth(0)
            .expect("")
            .trim()
            .to_string();

        let fields: Vec<&str> = net_prop.splitn(3, ':').collect();

        let device_type = fields[0];
        let state = fields[1];
        let connection = fields[2];

        if state == "connected" {
            if device_type == "wifi" || device_type == "wifi-p2pb" {
                println!("{}",connection)
            } else {
                println!("{}",device_type)
            }
        } else if state.contains("connecting") {
            println!("connecting")
        } else {
            println!("disconnected")
        }
    }
    fn get_wifi_strength() {
        let cmd = Command::new("nmcli")
            .args(["-t", "-f", "IN-USE,SIGNAL", "device", "wifi"])
            .output()
            .expect("failed to execute nmcli");

        let net_prop = String::from_utf8_lossy(&cmd.stdout).to_string();

        let lines: Vec<&str> = net_prop.lines().collect();
        let re: Regex = Regex::new(r"\*").unwrap();
        for (i, line) in lines.iter().enumerate() {
            if re.is_match(line) {
                println!("{}", &line[2..])
            }
        }
    }
    if which == "network" { 
        get_network();
        let dbus_conn = dbus::blocking::Connection::new_system().unwrap();
        
        let mut rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");
        rule.sender = Some("org.freedesktop.NetworkManager".into());

        dbus_conn.add_match(rule, move |_: (), _, msg| {
            let iface: String = msg.read1().unwrap_or_default();

            if iface.starts_with("org.freedesktop.NetworkManager.Device") {
                get_network();
            }
            true
        }).expect("failed to add signal match");
        loop {
            dbus_conn.process(Duration::from_millis(1000)).unwrap();
        }
    } else if which == "wifi_strength" {
        let interval = string_to_usize(interval).unwrap();
        loop {
            get_wifi_strength();
            
            thread::sleep(Duration::from_secs(interval as u64));
        }
    }
}

fn battery(which: &[String], interval: &[String]) {
    fn read_sys_value(path: &str) -> Option<f64> {
        fs::read_to_string(path).ok()?.trim().parse::<f64>().ok()
    }
    let interval = string_to_usize(interval).unwrap();
    let which = which.join(" ");
    let which = format!("{}",which.trim());
    if which == "time" {
        loop {
            let energy_now = read_sys_value("/sys/class/power_supply/BAT0/energy_now");
            let energy_now: f32 = energy_now.unwrap() as f32;
            
            let energy_full = read_sys_value("/sys/class/power_supply/BAT0/energy_full");
            let energy_full: f32 = energy_full.unwrap() as f32;
            
            let power_now = read_sys_value("/sys/class/power_supply/BAT0/power_now"); 
            let power_now: f32 = power_now.unwrap() as f32;
            
            let status = match fs::read_to_string("/sys/class/power_supply/BAT0/status") {
                Ok(ok) => ok.to_string(),
                Err(err) => { "Unknown".to_string() }
            };
            let status = format!("{}",status.trim());
            
            let time_hours = if status == "Discharging" {
                energy_now / power_now
            } else if status == "Charging" {
                (energy_full - energy_now) / power_now
            } else {
                0.0
            };
            let total_minutes = (time_hours * 60.0).round() as u64;
            let hours: u64 = (total_minutes / 60).to_string().chars().take(2).collect::<String>().parse().unwrap();
            let minutes: u64 = (total_minutes % 60).to_string().chars().take(2).collect::<String>().parse().unwrap();

            println!("{:02}h:{:02}m", hours, minutes);
            
            thread::sleep(Duration::from_secs(interval as u64));
        }
    } else if which == "status" {
        loop {
            let status = match fs::read_to_string("/sys/class/power_supply/BAT0/status") {
                Ok(ok) => ok.to_string(),
                Err(err) => { "Unknown".to_string() }
            };
            let status = format!("{}",status.trim());
            println!("{}",status);

            thread::sleep(Duration::from_secs(interval as u64));
        }
    } else if which == "capacity" {
        loop {
            let status = match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                Ok(ok) => ok.to_string(),
                Err(err) => { "Unknown".to_string() }
            };
            let status = format!("{}",status.trim());
            println!("{}",status);

            thread::sleep(Duration::from_secs(interval as u64));
        }
    }
}

fn renderws() {
    fn get() {
        let mut connection = I3Connection::connect().expect("failed to connect to i3");
        let workspaces = connection.get_workspaces().expect("failed to get workspaces");
        
        let workspaces_str = format!("{:#?}", workspaces);
        let re: Regex = Regex::new(r"num:\s*(\d+)").unwrap();
        
        let nums: Vec<u32> = re
            .captures_iter(&workspaces_str)
            .map(|cap| cap[1].parse::<u32>().unwrap())
            .collect();
        println!("{:?}", nums)
    }
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    listener.subscribe(&[Subscription::Workspace]);
    get();
    for event in listener.listen() {
        get()
    }
}
fn focusedws() {
    fn get() { 
        let mut connection = I3Connection::connect().expect("failed to connect to i3");
        let workspaces = connection.get_workspaces().expect("failed to get workspaces");
        
        let workspaces_str = format!("{:#?}", workspaces);
        let lines: Vec<&str> = workspaces_str.lines().collect();
        let re: Regex = Regex::new(r"focused:\s*true").unwrap();
        for (i, line) in lines.iter().enumerate() {
            if re.is_match(line) && i >= 3 {
                let target_line = lines[i - 3];
                
                let focused_str = format!("{}", target_line.trim());
                let re = Regex::new(r"num:\s*(\d+)").unwrap();

                if let Some(caps) = re.captures(&focused_str) {
                    let num = &caps[1];
                    println!("{}", num)
                }
            }
        }
    }
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    listener.subscribe(&[Subscription::Workspace]);
    get();
    for event in listener.listen() {
        get()
    }
}
fn urgentws() {
    fn get() { 
        let mut connection = I3Connection::connect().expect("failed to connect to i3");
        let workspaces = connection.get_workspaces().expect("failed to get workspaces");
        
        let workspaces_str = format!("{:#?}", workspaces);
        let lines: Vec<&str> = workspaces_str.lines().collect();
        let re: Regex = Regex::new(r"urgent:\s*true").unwrap();
        for (i, line) in lines.iter().enumerate() {
            if re.is_match(line) && i >= 4 {
                let target_line = lines[i - 4];
                
                let focused_str = format!("{}", target_line.trim());
                let re = Regex::new(r"num:\s*(\d+)").unwrap();

                if let Some(caps) = re.captures(&focused_str) {
                    let num = &caps[1];
                    println!("{}", num)
                }
            }
        }
    }
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    listener.subscribe(&[Subscription::Window]);
    for event in listener.listen() {
        get()
    }
}
fn mode() { 
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    listener.subscribe(&[Subscription::Mode]);
    for event in listener.listen() {
        if let Ok(Event::ModeEvent(mode_event)) = event {
            println!("{}", mode_event.change);
        }
    }
}
fn title(length: &[String]) {
    let len = string_to_usize(length).unwrap();
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    let subs = [
        Subscription::Workspace,
        Subscription::Window,
    ];
    listener.subscribe(&subs).expect("failed to subscribe");
    for event in listener.listen() {
        let mut connection = I3Connection::connect().unwrap();
        let tree = connection.get_tree().unwrap();

        let mut stack = vec![tree];

        while let Some(node) = stack.pop() {
            if node.focused {
                if node.nodetype == NodeType::Workspace {
                    println!("_nil_focused");
                } else {
                    let title = format!("{}", node.name.as_deref().unwrap());
                    let cut_title = title.chars().take(len).collect::<String>();
                    if title.chars().count() > len {
                        println!("{}…", cut_title);
                    } else {
                        println!("{}", title);
                    }
                }
            }
            stack.extend(node.nodes);
            stack.extend(node.floating_nodes);
        }
    }
}
fn icon_dump() {
    let mut listener = I3EventListener::connect().expect("failed to connect listener");
    let subs = [
        Subscription::Workspace,
        Subscription::Window,
    ];
    listener.subscribe(&subs).expect("failed to subscribe");
    for event in listener.listen() {
        let mut connection = I3Connection::connect().unwrap();
        let tree = connection.get_tree().unwrap();

        let mut stack = vec![tree];
        let mut window_id: Option<u32> = None;

        while let Some(node) = stack.pop() {
            if node.focused {
                if node.nodetype != NodeType::Workspace {
                    if let Some(id) = node.window {
                        window_id = Some(id as u32);
                        break;
                    }
                }
            }
            stack.extend(node.nodes);
            stack.extend(node.floating_nodes);
        }

        let window_id = match window_id {
            Some(id) => id,
            None => 0
        };

        if window_id != 0 {
            let (conn, screen_num) = RustConnection::connect(None).unwrap();
            let screen = &conn.setup().roots[screen_num];
            let atom = conn.intern_atom(false, b"_NET_WM_ICON").unwrap().reply().unwrap().atom;
            let prop = conn.get_property(false, window_id, atom, AtomEnum::CARDINAL, 0, u32::MAX)
                .unwrap().reply().unwrap();

            if prop.format != 32 {
                std::fs::remove_file("/tmp/current_icon.png").ok();
                println!("0");
                continue;
            }

            let data: Vec<u32> = prop.value32().expect("expected 32-bit data").collect();
            
            let mut i = 0;
            let mut largest = (0, 0, vec![]);

            while i + 2 < data.len() {
                let w = data[i] as usize;
                let h = data[i + 1] as usize;
                let len = w * h;

                if i + 2 + len > data.len() {
                    break;
                }

                if w * h > largest.0 * largest.1 {
                    largest = (w, h, data[i + 2..i + 2 + len].to_vec());
                }

                i += 2 + len;
            }

            let (width, height, pixels) = largest;
            assert!(pixels.len() >= width * height, "not enough pixels");

            let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);
            for y in 0..height {
                for x in 0..width {
                    let pixel = pixels[y * width + x];
                    let a = ((pixel >> 24) & 0xFF) as u8;
                    let r = ((pixel >> 16) & 0xFF) as u8;
                    let g = ((pixel >> 8) & 0xFF) as u8;
                    let b = (pixel & 0xFF) as u8;
                    img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
                }
            }

            let mut file = BufWriter::new(File::create("/tmp/current_icon.png").unwrap());
            image::DynamicImage::ImageRgba8(img)
                .write_to(&mut file, image::ImageFormat::Png)
                .unwrap();
            println!("/tmp/current_icon.png");
        } else {
            println!("{}",window_id);
        }
    }
}
fn string_to_usize(string: &[String]) -> Result<usize, ()> {
    let string = string.join("");
    let usize = match string.parse::<usize>() {
        Ok(n) => return Ok(n),
        Err(e) => return Err(())
    };
}
