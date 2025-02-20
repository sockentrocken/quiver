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

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use crate::script::Script;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.thread", "info": "The thread API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let thread = lua.create_table()?;

    thread.set("new", lua.create_function(self::Thread::new)?)?;

    table.set("thread", thread)?;

    Ok(())
}

//================================================================

/* class
{
    "version": "1.0.0",
    "name": "thread",
    "info": "TO-DO"
}
*/
#[derive(Clone)]
pub struct ThreadTable {
    tx: Sender<ThreadVariable>,
    hash: HashMap<String, String>,
}

impl ThreadTable {
    fn new(tx: Sender<ThreadVariable>) -> mlua::Result<Self> {
        Ok(Self {
            tx,
            hash: HashMap::new(),
        })
    }
}

impl mlua::UserData for ThreadTable {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("tx", |_, this, _: ()| {
            this.tx
                .send(ThreadVariable::Table(this.hash.clone()))
                .unwrap();
            Ok(())
        });

        method.add_method_mut("set", |_, this, (k, v): (String, String)| {
            this.hash.insert(k, v);
            Ok(())
        });
    }
}

#[derive(Serialize, Deserialize)]
enum ThreadVariable {
    Integer(i64),
    Number(f64),
    String(String),
    Table(HashMap<String, String>),
}

/* class
{
    "version": "1.0.0",
    "name": "thread",
    "info": "TO-DO"
}
*/
#[derive(Clone)]
pub struct ChannelSend(Sender<ThreadVariable>, Arc<Mutex<bool>>);

impl ChannelSend {
    fn new(tx: Sender<ThreadVariable>, sentinel: Arc<Mutex<bool>>) -> mlua::Result<Self> {
        Ok(Self(tx, sentinel))
    }
}

impl mlua::UserData for ChannelSend {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("tx", |lua, this, value: LuaValue| {
            this.0.send(ThreadVariable::Number(1.0)).unwrap();
            Ok(())
        });

        method.add_method_mut("table", |_, this, _: ()| {
            Ok(ThreadTable::new(this.0.clone()))
        });

        method.add_method_mut("active", |_, this, _: ()| {
            let lock = this.1.lock().unwrap();
            Ok(*lock)
        });
    }
}

/* class
{
    "version": "1.0.0",
    "name": "thread",
    "info": "TO-DO"
}
*/
pub struct Thread {
    handle: Option<JoinHandle<()>>,
    rx: Receiver<ThreadVariable>,
    sentinel: Arc<Mutex<bool>>,
}

impl Thread {
    fn new(lua: &Lua, (code, data): (String, mlua::Variadic<LuaValue>)) -> mlua::Result<Self> {
        let sentinel = Arc::new(Mutex::new(true));
        let (tx, rx): (Sender<ThreadVariable>, Receiver<ThreadVariable>) = mpsc::channel();
        let tx = ChannelSend::new(tx, sentinel.clone()).unwrap();
        let script_data = lua.app_data_ref::<crate::script::ScriptData>().unwrap();
        let info = script_data.info.clone();

        let handle = thread::spawn(move || {
            let channel = tx.clone();
            let script = Script::clone(&info, &code).unwrap();

            script
                .main
                .call::<bool>((channel, data))
                .map_err(|e| e.to_string())
                .unwrap();
        });

        Ok(Self {
            handle: Some(handle),
            rx,
            sentinel,
        })
    }
}

impl mlua::UserData for Thread {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("join", |_, this, _: ()| {
            if let Some(handle) = this.handle.take() {
                if let Ok(a) = handle.join() {
                    return Ok(a);
                }
            }

            Ok(())
        });

        method.add_method_mut("rx", |lua, this, _: ()| {
            if let Ok(rx) = this.rx.recv() {
                lua.to_value(&rx)
            } else {
                Ok(mlua::Nil)
            }
        });

        method.add_method_mut("active", |_, this, _: ()| {
            let mut lock = this.sentinel.lock().unwrap();
            *lock = false;
            Ok(())
        });
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
