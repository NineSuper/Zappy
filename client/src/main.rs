/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/14 09:50:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/14 10:28:45 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod app;
mod env;
mod settings;

use crate::app::AppState;
use crate::env::init_env;

fn	main()
{
	let app_state: AppState = AppState
	{
		settings: init_env(),
	};
	println!("{:?}", app_state.settings)
}
