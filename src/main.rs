use colored::*;
use rpassword::read_password;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

const PASSCODE: &str = "@seleste3580##2005##waithaka";

fn main() {
    println!("{}", "Seleste v1.0 Ethical Hacking Suite".bright_cyan().bold());
    println!("{}", "Enter passcode to continue:".yellow());

    let input = read_password().expect("Failed to read password");

    if input != PASSCODE {
        println!("{}", "Access Denied!".red().bold());
        return;
    }

    println!("{}", "Access Granted ✔".green().bold());

    let tools = vec![
        "Nmap", "SQLmap", "Netcat", "Wireshark", "John the Ripper",
        "theHarvester", "Aircrack-ng", "Reaver", "Wifite", "Kismet",
        "Ettercap", "Hashcat", "Metasploit", "Recon-ng", "DNSenum",
        "Nikto", "OpenVAS", "Wpscan", "Lynis", "Searchsploit",
        "BeEF", "Bettercap", "MITMf", "Volatility", "Autopsy",
        "Maltego", "Armitage", "ExploitDB", "Fern WiFi Cracker", "Xspy"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a tool to launch")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    let chosen_tool = tools[selection];
    println!("Launching {}...", chosen_tool.green());

    match chosen_tool {
        "Nmap" => run("nmap"),
        "SQLmap" => run("sqlmap"),
        "Netcat" => run("nc"),
        "Wireshark" => run("wireshark"),
        "John the Ripper" => run("john"),
        "theHarvester" => run("theHarvester"),
        "Aircrack-ng" => run("aircrack-ng"),
        "Reaver" => run("reaver"),
        "Wifite" => run("wifite"),
        "Kismet" => run("kismet"),
        "Ettercap" => run("ettercap"),
        "Hashcat" => run("hashcat"),
        "Metasploit" => run("msfconsole"),
        "Recon-ng" => run("recon-ng"),
        "DNSenum" => run("dnsenum"),
        "Nikto" => run("nikto"),
        "OpenVAS" => run("openvas"),
        "Wpscan" => run("wpscan"),
        "Lynis" => run("lynis"),
        "Searchsploit" => run("searchsploit"),
        "BeEF" => run("beef-xss"),
        "Bettercap" => run("bettercap"),
        "MITMf" => run("mitmf"),
        "Volatility" => run("volatility"),
        "Autopsy" => run("autopsy"),
        "Maltego" => run("maltego"),
        "Armitage" => run("armitage"),
        "ExploitDB" => run("searchsploit"),
        "Fern WiFi Cracker" => run("fern-wifi-cracker"),
        "Xspy" => run("xspy"),
        _ => println!("{} is not yet linked to a command.", chosen_tool.red()),
    }
}

fn run(cmd: &str) {
    match Command::new(cmd).spawn() {
        Ok(_) => println!("{} launched successfully.", cmd),
        Err(_) => println!("Failed to launch {}. Is it installed?", cmd),
    }
}
