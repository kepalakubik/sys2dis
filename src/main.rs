mod config;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use std::{thread, fs, time::Duration};

fn main() {
	let config = config::load_or_create_config();
	let start_timestamp = System::boot_time() as i64;
	
	println!("-= sys2dis - System to Discord =-");
	println!("App ID  : {}", config.app_id);
    println!("Interval: {} secs", config.update_interval);

    let mut client = DiscordIpcClient::new(&config.app_id);

    // Wait until it connected to Discord
    loop {
        if connect_discord(&mut client) {
       		break;
        }

        eprintln!("[!] Attempt failed. Will retry in 60 seconds...");
        thread::sleep(Duration::from_secs(60));
    }

    // Main loop
    loop {
	    match update_activity(&mut client, &config, start_timestamp) {
	        Ok(_) => {}
	        Err(e) => {
				eprintln!("[!] Error update activity: {}. Will reconnect...", e);
				let _ = client.close();
				thread::sleep(Duration::from_secs(5));

				if !connect_discord(&mut client) {
					eprintln!("[!] Attempt failed. Will retry in 60 seconds...");
        			thread::sleep(Duration::from_secs(60));
           			continue;
				}
			}
	    }
					
		thread::sleep(Duration::from_secs(config.update_interval));
    }
}

struct SystemStats {
    cpu_usage: f32,
    cpu_temp: f32,
    ram_percent: f32,
    swap_percent: f32,
    distro: String,
    kernel: String,
}

impl SystemStats {
    fn collect() -> Self {
	   	let mut sys = System::new_with_specifics(
	   		RefreshKind::default()
	       		.with_cpu(CpuRefreshKind::default().with_cpu_usage())
	       		.with_memory(MemoryRefreshKind::everything())
	   	);
		let components = Components::new_with_refreshed_list();
	    
	   	// CPU need two sampling for accuration
	    sys.refresh_all();
	    thread::sleep(Duration::from_millis(500));
	    sys.refresh_cpu_usage();

		// CPU usage
	    let cpus = sys.cpus();
        let cpu_usage = if !cpus.is_empty() {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        // CPU temperature
        let cpu_temp = components
        	.iter()
        	.find(|c| {
         		let label = c.label().to_lowercase();
         		label.contains("cpu") || label.contains("core 0") || label.contains("k10temp") || label.contains("coretemp")
         	})
         	.and_then(|c| c.temperature())
         	.unwrap_or(0.0);

        // RAM usage
        let ram_used = sys.used_memory();
        let ram_total = sys.total_memory();
        let ram_percent = if ram_total > 0 {
        	(ram_used as f32 / ram_total as f32) * 100.0
        } else {
            0.0
        };

        // Swap usage
        let swap_used = sys.used_swap();
        let swap_total = sys.total_swap();
        let swap_percent = if swap_total > 0 {
        	(swap_used as f32 / swap_total as f32) * 100.0
        } else {
            0.0
        };

        Self {
            cpu_usage,
            cpu_temp,
            ram_percent,
            swap_percent,
            distro: get_distro_name(),
            kernel: get_kernel_version(),
        }
    }

    /// Detail line — max ~128 karakter
    fn details_line(&self) -> String {
        format!(
            "CPU: {:.1}% | Temp: {:.1}°C",
            self.cpu_usage,
            self.cpu_temp
        )
    }

    /// State line — max ~128 karakter
    fn state_line(&self) -> String {
        format!(
            "RAM: {} | Swap: {}",
            format!("{:.1}%", self.ram_percent),
            format!("{:.1}%", self.swap_percent)
        )
    }

    /// Large image tooltip
    fn large_tooltip(&self) -> String {
    	format!("{} | Kernel {}", self.distro, self.kernel)
    }
}

/// Connect to Discord with reconnect attempt
fn connect_discord(client: &mut DiscordIpcClient) -> bool {
	for attempt in 1..=5 {
		match client.connect() {
			Ok(_) => {
				println!("[+] Connected!");
				return true;
			}
			Err(e) => {
				eprintln!(
					"[!] Failed to connect: {}. Will try again in 10 secs... (attempt {}/5)",
					e, attempt,
				);
				thread::sleep(Duration::from_secs(10));
			}
		}
	}

	false
}

/// Update Discord activity
fn update_activity(client: &mut DiscordIpcClient, config: &config::Config, start_timestamp: i64) -> Result<(), Box<dyn std::error::Error>> {
	let stats = SystemStats::collect();
	let details = stats.details_line();
	let state = stats.state_line();
	let large_tip = stats.large_tooltip();
	//let small_tip = stats.small_tooltip();

	let payload = activity::Activity::new()
		.details(&details)
		.state(&state)
		.activity_type(activity::ActivityType::Watching)
		.timestamps(activity::Timestamps::new().start(start_timestamp))
		.assets(
			activity::Assets::new()
				.large_image(&config.large_image)
				.large_text(&large_tip)
				.small_image(&config.small_image)
				//.small_text(&small_tip)
		);

	client.set_activity(payload)?;
	Ok(())
}

/// Get distro name
fn get_distro_name() -> String {
	if let Ok(content) = fs::read_to_string("/etc/os-release") {
		for line in content.lines() {
			if line.starts_with("PRETTY_NAME") {
				let val = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
				return val.to_string();
			}
		}
	}

	"Linux".to_string()
}

/// Get kernel version
fn get_kernel_version() -> String {
	if let Ok(version) = fs::read_to_string("/proc/version") {
		let parts: Vec<&str> = version.split_whitespace().collect();
		if parts.len() >= 3 {
			return parts[2].to_string();
		}
	}

	System::kernel_version().unwrap_or_else(|| "unknown".to_string())
}