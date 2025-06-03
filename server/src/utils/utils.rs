/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 23:42:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/08 00:57:28 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::Color;
use rand::prelude::IndexedRandom;
use rand::rng;

pub fn get_random_color() -> Color {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    *colors.choose(&mut rng()).unwrap()
}
