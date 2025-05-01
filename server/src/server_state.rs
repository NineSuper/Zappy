/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server_state.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/01 22:20:01 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug)]
pub struct ServerConfig
{
    pub _port: u32,
    pub width: u32,
    pub height: u32,
    pub _clients: u32,
    pub _time_unit: u32,
    pub _teams: Vec<String>,
}
