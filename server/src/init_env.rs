/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   init_env.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/30 15:06:42 by tde-los-          #+#    #+#             */
/*   Updated: 2025/04/30 17:49:10 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{char, env::args, process::exit};

fn	get_args() -> Vec<String>
{
	let mut args: Vec<String> = args().collect();
	args.remove(0);
	return args;
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

pub	fn	init_env()
{
    let	args: Vec<String> = get_args();
    let port = get_var(&args, 'p');
	let height = get_var(&args, 'x');
	let width = get_var(&args, 'y');
	let clients = get_var(&args, 'c');
	let time_unit = get_var(&args, 't');

	// DEBUG
    println!("Port : {}", port);
    println!("height : {}", height);
    println!("width : {}", width);
    println!("clients : {}", clients);
    println!("time_unit : {}", time_unit);
}

