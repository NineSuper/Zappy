/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   env.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/30 15:06:42 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/07 15:08:32 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::ServerSettings;
use std::{char, env::args, process::exit};
use colored::*;

fn	get_args() -> Vec<String>
{
	let mut args: Vec<String> = args().collect();
	args.remove(0);
	return args;
}

fn	print_env(port: u32, height: u32, width: u32, clients: u32, time_unit: u32, teams: &[String]) {
    let line = "🟩=============== Zappy Server ===============🟩";
    println!("{}", line.green().bold());

    println!(
        "{} {}",
        "🌐 IP Address :".green().bold(),
        format!("127.0.0.1:{}", port).bold().underline()
    );
    println!(
        "{} {}",
        "📏 Map Size   :".green().bold(),
        format!("{} x {} px", width, height).bold()
    );
    println!(
        "{} {}",
        "👥 Clients    :".green().bold(),
        format!("{}", clients).bold()
    );
    println!(
        "{} {}",
        "⏱️  Time Unit  :".green().bold(),
        format!("{}t", time_unit).bold()
    );
	println!(
		"{} {}",
		"🏳️  Teams      :".green().bold(),
		teams.join(", ").bold()
	);

    println!("{}", "================================================\n".green().bold());
}

fn	get_var(args: &[String], flag: char) -> u32
{
	for i in 0..args.len()
	{
		if args[i] == format!("-{}", flag)
		{
			if let Some(var_str) = args.get(i + 1)
			{
				if !var_str.starts_with('-')
				{
					if let Ok(vars) = var_str.parse::<u32>()
					{
						return vars;
					}
				}
			}
		}
	}
	println!("Erreur: argument -{} non trouvé ou invalide", flag);
	exit(-1);
}

fn	get_teams(args: &[String]) -> Vec<String>
{
	for i in 0..args.len()
	{
		if args[i] == "-n" {
			let mut	teams = Vec::new();
			let mut	j = i + 1;

			while j < args.len() && !args[j].starts_with('-')
			{
				teams.push(args[j].clone());
				j += 1;
			}
			if teams.is_empty()
			{
				println!("Erreur: au moins une équipe est requise après -n");
				exit(-1);
			}
			return teams;
		}
	}
	println!("Erreur: flag -n manquant");
	exit(-1);
}

pub	fn	init_env() -> ServerSettings
{
    let	args: Vec<String> = get_args();
    let port = get_var(&args, 'p');
	let height = get_var(&args, 'x');
	let width = get_var(&args, 'y');
	let connexion_max = get_var(&args, 'c');
	let time_unit = get_var(&args, 't');
	let teams = get_teams(&args);

	print_env(port, height, width, connexion_max, time_unit, &teams);
	return ServerSettings{port, width, height, connexion_max, time_unit, teams_name: teams};
}

