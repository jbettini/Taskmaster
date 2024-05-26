/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:49:43 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/26 06:09:25 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod server {
    pub mod taskmasterd;
}

fn main() {
    server::taskmasterd::taskmasterd();
}