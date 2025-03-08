/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::script::*;
use crate::status::*;

//================================================================

use mlua::prelude::*;
use tokio::net::{TcpListener, TcpStream, UdpSocket};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.socket", "info": "The socket API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let socket = lua.create_table()?;

    socket.set("new_TCP_listen", lua.create_async_function(self::SocketTCPListen::new)?)?;
    socket.set("new_TCP_stream", lua.create_async_function(self::SocketTCPStream::new)?)?;
    socket.set("new_UDP",        lua.create_async_function(self::SocketUDP::new)?)?;

    table.set("socket", socket)?;

    Ok(())
}

//================================================================

/* class
{ "version": "1.0.0", "name": "socket_TCP_stream", "info": "An unique handle to a TCP (stream) socket in memory." }
*/
struct SocketTCPStream(TcpStream);

unsafe impl Send for SocketTCPStream {}

impl mlua::UserData for SocketTCPStream {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "socket_TCP_stream:set",
            "info": "TO-DO"
        }
        */
        method.add_async_method("set", |_: Lua, this, data: Vec<u8>| async move {
            this.0.try_write(&data)?;

            Ok(())
        });
    }
}

impl SocketTCPStream {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.socket.new_TCP_stream",
        "info": "TO-DO"
    }
    */
    async fn new(_: Lua, address: String) -> mlua::Result<Self> {
        let socket = TcpStream::connect(address).await?;

        Ok(Self(socket))
    }
}

//================================================================

/* class
{ "version": "1.0.0", "name": "socket_TCP_listen", "info": "An unique handle to a TCP (listen) socket in memory." }
*/
struct SocketTCPListen(TcpListener);

unsafe impl Send for SocketTCPListen {}

impl mlua::UserData for SocketTCPListen {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "socket_TCP:accept",
            "info": "TO-DO"
        }
        */
        method.add_async_method("accept", |_: Lua, this, _: ()| async move {
            let (socket, _) = this.0.accept().await?;

            Ok(SocketTCPStream(socket))
        });
    }
}

impl SocketTCPListen {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.socket.new_TCP_listen",
        "info": "TO-DO"
    }
    */
    async fn new(_: Lua, address: String) -> mlua::Result<Self> {
        let socket = TcpListener::bind(address).await?;

        Ok(Self(socket))
    }
}

//================================================================

/* class
{ "version": "1.0.0", "name": "socket_UDP", "info": "An unique handle to a UDP socket in memory." }
*/
struct SocketUDP(UdpSocket);

unsafe impl Send for SocketUDP {}

impl mlua::UserData for SocketUDP {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "socket_UDP:connect",
            "info": "TO-DO"
        }
        */
        method.add_async_method("connect", |_: Lua, this, address: String| async move {
            this.0.connect(address).await?;

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "socket_UDP:get",
            "info": "TO-DO"
        }
        */
        method.add_async_method("get", |_: Lua, this, _: ()| async move {
            let mut data = [0; 32];

            while let Ok(length) = this.0.try_recv(&mut data) {
                println!("{:?} bytes received", length);
            }

            Ok(data)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "socket_UDP:set",
            "info": "TO-DO"
        }
        */
        method.add_async_method("set", |_: Lua, this, data: Vec<u8>| async move {
            let len = this.0.send(&data).await?;
            println!("{:?} bytes sent", len);

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "socket_UDP:get_at",
            "info": "TO-DO"
        }
        */
        method.add_async_method("get_at", |_: Lua, this, _: ()| async move {
            let mut data = [0; 32];

            while let Ok((length, address)) = this.0.try_recv_from(&mut data) {
                println!("{:?} bytes received from {address}", length);
            }

            Ok(data)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "socket_UDP:set_at",
            "info": "TO-DO"
        }
        */
        method.add_async_method(
            "set_at",
            |_: Lua, this, (data, address): (Vec<u8>, String)| async move {
                let len = this.0.send_to(&data, address).await?;
                println!("{:?} bytes sent", len);

                Ok(())
            },
        );
    }
}

impl SocketUDP {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.socket.new_UDP",
        "info": "TO-DO"
    }
    */
    async fn new(_: Lua, address: String) -> mlua::Result<Self> {
        let sock = UdpSocket::bind(address).await?;

        Ok(Self(sock))
    }
}
