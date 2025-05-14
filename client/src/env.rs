/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   env.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/14 10:24:23 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/14 10:24:59 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::settings::ClientSettings;

use std::env::args;
use std::process::exit;
use std::str::FromStr;

fn	get_args() -> Vec<String>
{
	let mut args: Vec<String> = args().collect();
	args.remove(0);
	return args;
}

fn	get_var<T: FromStr>(args: &[String], flag: char) -> T
{
	for i in 0..args.len()
	{
		if args[i] == format!("-{}", flag)
		{
			if let Some(var_str) = args.get(i + 1)
			{
				if !var_str.starts_with('-')
				{
					if let Ok(val) = var_str.parse::<T>()
					{
						return val;
					}
				}
			}
		}
	}
	eprintln!("Erreur: argument -{} non trouvé ou invalide", flag);
	exit(-1);
}

fn	get_optional_var<T: FromStr>(args: &[String], flag: char) -> Option<T>
{
	for i in 0..args.len()
	{
		if args[i] == format!("-{}", flag)
		{
			if let Some(var_str) = args.get(i + 1)
			{
				if !var_str.starts_with('-')
				{
					if let Ok(val) = var_str.parse::<T>()
					{
						return Some(val);
					}
				}
			}
		}
	}
	return None;
}

pub	fn	init_env() -> ClientSettings
{
	let	args: Vec<String> = get_args();
	let config = ClientSettings
	{
		team: get_var(&args, 'n'),
		ip: get_optional_var(&args, 'h').unwrap_or_else(|| "127.0.0.1".to_string()),
		port: get_var(&args, 'p'),
	};
	return config;
}
