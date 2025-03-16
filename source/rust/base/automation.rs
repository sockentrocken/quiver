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
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.automation", "info": "The automation API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let automation = lua.create_table()?;

    automation.set("new", lua.create_function(self::AutomationEvent::new)?)?;

    table.set("automation", automation)?;

    Ok(())
}

//================================================================

/* class
{ "version": "1.0.0", "name": "automation_event", "info": "An unique handle to an automation event list." }
*/
struct AutomationEvent(ffi::AutomationEventList);

unsafe impl Send for AutomationEvent {}

impl mlua::UserData for AutomationEvent {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("count", |_, this| Ok(this.0.count));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:save",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("save", |_: &Lua, this, path: String| unsafe {
            let path = Script::rust_to_c_string(&path)?;

            ffi::ExportAutomationEventList(this.0, path.as_ptr());

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:set_active",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_active", |_: &Lua, this, _: ()| unsafe {
            ffi::SetAutomationEventList(&mut this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:set_frame",
            "info": "TO-DO"
        }
        */
        method.add_method("set_frame", |_: &Lua, _, frame: i32| unsafe {
            ffi::SetAutomationEventBaseFrame(frame);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:start",
            "info": "TO-DO"
        }
        */
        method.add_method("start", |_: &Lua, _, _: ()| unsafe {
            ffi::StartAutomationEventRecording();
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:stop",
            "info": "TO-DO"
        }
        */
        method.add_method("stop", |_: &Lua, _, _: ()| unsafe {
            ffi::StopAutomationEventRecording();
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:play",
            "info": "TO-DO"
        }
        */
        method.add_method("play", |_: &Lua, this, frame: u32| unsafe {
            if frame < this.0.count {
                let event = *this.0.events.wrapping_add(frame as usize);
                ffi::PlayAutomationEvent(event);
                return Ok(());
            }

            Err(mlua::Error::runtime(
                "automation_event:play(): Invalid index.",
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "automation_event:get_event",
            "info": "TO-DO"
        }
        */
        method.add_method("get_event", |_: &Lua, this, frame: u32| unsafe {
            if frame < this.0.count {
                let event = *this.0.events.wrapping_add(frame as usize);
                return Ok((event.frame, event.type_, event.params));
            }

            Err(mlua::Error::runtime(
                "automation_event:get_event(): Invalid index.",
            ))
        });
    }
}

impl AutomationEvent {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.automation.new",
        "info": "TO-DO"
    }
    */
    fn new(lua: &Lua, path: Option<String>) -> mlua::Result<Self> {
        unsafe {
            let path = match path {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let list = ffi::LoadAutomationEventList(path);

            Ok(Self(list))
        }
    }
}

impl Drop for AutomationEvent {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadAutomationEventList(self.0);
        }
    }
}
