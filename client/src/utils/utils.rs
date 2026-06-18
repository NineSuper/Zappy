/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kyfontan <kyfontan@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/06/18 17:12:12 by kyfontan          #+#    #+#             */
/*   Updated: 2026/06/18 17:14:52 by kyfontan         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
	#[arg(short, long)]
	n: String,

	#[arg(short, long)]
	p: usize,

	#[arg(short, long, default_value = "127.0.0.1")]
	h: String
}

