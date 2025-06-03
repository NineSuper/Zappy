/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   env.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/30 15:06:42 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/27 12:28:55 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::ServerSettings;
use colored::*;
use std::{char, env::args, process::exit};

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    return args;
}

fn print_env(config: ServerSettings) {
    let line = "🟩=============== Zappy Server ===============🟩";
    println!("{}", line.green().bold());

    println!(
        "{} {}",
        "🌐 IP Address :".green().bold(),
        format!("127.0.0.1:{}", config.port).bold().underline()
    );
    println!(
        "{} {}",
        "📏 Map Size   :".green().bold(),
        format!("{} x {} px", config.width, config.height).bold()
    );
    println!(
        "{} {}",
        "👥 Connexion Max :".green().bold(),
        format!("{}", config.connexion_max).bold()
    );
    println!(
        "{} {}",
        "⏱️  Time Unit  :".green().bold(),
        format!("{}t", config.time_unit).bold()
    );
    println!(
        "{} {}",
        "🏳️  Teams      :".green().bold(),
        config.teams_name.join(", ").bold()
    );

    println!(
        "{}",
        "================================================\n"
            .green()
            .bold()
    );
}

fn get_var(args: &[String], flag: char) -> u32 {
    for i in 0..args.len() {
        if args[i] == format!("-{}", flag) {
            if let Some(var_str) = args.get(i + 1) {
                if !var_str.starts_with('-') {
                    if let Ok(vars) = var_str.parse::<u32>() {
                        if vars < 1 {
                            println!("Erreur: argument -{} ne peut pas être inférieur à 1", flag);
                            exit(-1);
                        }
                        return vars;
                    }
                }
            }
        }
    }
    println!("Erreur: argument -{} non trouvé ou invalide", flag);
    exit(-1);
}

fn get_var_time(args: &[String], flag: char) -> f64 {
    for i in 0..args.len() {
        if args[i] == format!("-{}", flag) {
            if let Some(var_str) = args.get(i + 1) {
                if !var_str.starts_with('-') {
                    if let Ok(vars) = var_str.parse::<f64>() {
                        if vars < 1.0 {
                            println!("Erreur: argument -{} ne peut pas être inférieur à 1", flag);
                            exit(-1);
                        }
                        return vars;
                    }
                }
            }
        }
    }
    println!("Erreur: argument -{} non trouvé ou invalide", flag);
    exit(-1);
}

fn get_teams(args: &[String]) -> Vec<String> {
    for i in 0..args.len() {
        if args[i] == "-n" {
            let mut teams = Vec::new();
            let mut j = i + 1;

            while j < args.len() && !args[j].starts_with('-') {
                teams.push(args[j].clone());
                j += 1;
            }
            if teams.is_empty() {
                println!("Erreur: au moins une équipe est requise après -n");
                exit(-1);
            }
            return teams;
        }
    }
    println!("Erreur: flag -n manquant");
    exit(-1);
}

pub fn init_env() -> ServerSettings {
    let args: Vec<String> = get_args();
    let config = ServerSettings {
        port: get_var(&args, 'p'),
        height: get_var(&args, 'x'),
        width: get_var(&args, 'y'),
        connexion_max: get_var(&args, 'c'),
        time_unit: get_var_time(&args, 't'),
        teams_name: get_teams(&args),
    };
    // print_env(config.clone());
    return config;
}
