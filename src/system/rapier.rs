/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use mlua::prelude::*;
use rapier3d::control::{CharacterAutostep, CharacterLength, KinematicCharacterController};
use rapier3d::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.rapier", "info": "The Rapier API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let rapier = lua.create_table()?;

    rapier.set("new", lua.create_function(self::Rapier::new)?)?;

    table.set("rapier", rapier)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "rapier", "info": "An unique handle for a Rapier simulation." }
*/
#[derive(Default)]
struct Rapier {
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    debug_render: DebugRenderPipeline,
}

impl Rapier {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.rapier.new",
        "info": "Create a new Rapier simulation.",
        "result": [
            { "name": "rapier", "info": "Rapier simulation.", "kind": "rapier" }
        ]
    }
    */
    fn new(_: &Lua, _: ()) -> mlua::Result<Self> {
        Ok(Self::default())
    }
}

impl mlua::UserData for Rapier {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:cast_ray",
            "info": "Cast a ray.",
            "member": [
                { "name": "ray",    "info": "", "kind": "ray"    },
                { "name": "time",   "info": "", "kind": "number" },
                { "name": "filter", "info": "", "kind": "table"  }
            ],
            "result": [
                { "name": "pick", "info": "", "kind": "boolean" },
                { "name": "time", "info": "", "kind": "number"  }
            ]
        }
        */
        method.add_method_mut(
            "cast_ray",
            |lua, this, (ray, time, filter): (LuaValue, f32, LuaValue)| {
                let ray: raylib::math::Ray = lua.from_value(ray)?;
                let filter: ColliderHandle = lua.from_value(filter)?;

                if let Some(hit) = this.query_pipeline.cast_ray_and_get_normal(
                    &this.rigid_body_set,
                    &this.collider_set,
                    &rapier3d::geometry::Ray::new(
                        point![ray.position.x, ray.position.y, ray.position.z],
                        vector![ray.direction.x, ray.direction.y, ray.direction.z],
                    ),
                    time,
                    true,
                    QueryFilter::new().exclude_collider(filter),
                ) {
                    return lua.to_value(&hit.1);
                }

                Ok(mlua::Nil)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_set_shape",
            "info": "Set the collider shape.",
            "member": [
                { "name": "collider", "info": "Collider.", "kind": "table"    },
                { "name": "shape",    "info": "",          "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "collider_set_shape",
            |lua, this, (collider, min, max): (LuaValue, LuaValue, LuaValue)| {
                let collider: ColliderHandle = lua.from_value(collider)?;
                let min: Vector3 = lua.from_value(min)?;
                let max: Vector3 = lua.from_value(max)?;

                let c = this.collider_set.get_mut(collider).unwrap();

                c.set_shape(SharedShape::capsule(
                    point![min.x, min.y, min.z],
                    point![max.x, max.y, max.z],
                    0.5,
                ));

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:character_controller",
            "info": "Create a kinematic character controller.",
            "result": [
                { "name": "controller", "info": "Controller.", "kind": "table" },
                { "name": "collider",   "info": "Collider.",   "kind": "table" }
            ]
        }
        */
        method.add_method_mut("character_controller", |lua, this, _: ()| {
            let controller = KinematicCharacterController::default();

            let collider =
                ColliderBuilder::cuboid(0.5, 1.0, 0.5).translation(vector![4.0, 2.0, 0.0]);
            let collider = this.collider_set.insert(collider);

            Ok((
                lua.to_value(&controller).unwrap(),
                lua.to_value(&collider).unwrap(),
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:move_character_controller",
            "info": "Move a kinematic character controller.",
            "member": [
                { "name": "controller", "info": "Controller.", "kind": "table"    },
                { "name": "collider",   "info": "Collider.",   "kind": "table"    },
                { "name": "velocity",   "info": "Velocity.",   "kind": "vector_3" },
                { "name": "time_step",  "info": "Time step.",  "kind": "number"   }
            ],
            "result": [
                { "name": "translation", "info": "Translation.", 		"kind": "vector_3" },
                { "name": "floor", 		 "info": "Currently on floor.", "kind": "boolean"  },
                { "name": "slide", 		 "info": "Currently on slide.", "kind": "boolean"  }
            ]
        }
        */
        method.add_method_mut(
            "move_character_controller",
            |lua, this, (controller, collider, velocity, time_step): (LuaValue, LuaValue, LuaValue, f32)| {
                let controller: KinematicCharacterController = lua.from_value(controller)?;
                let collider: ColliderHandle = lua.from_value(collider)?;
                let velocity: Vector3 = lua.from_value(velocity)?;

                let c = this.collider_set.get(collider).unwrap();

                let corrected_movement = controller.move_shape(
                    time_step,            // The timestep length (can be set to SimulationSettings::dt).
                    &this.rigid_body_set, // The RigidBodySet.
                    &this.collider_set,   // The ColliderSet.
                    &this.query_pipeline, // The QueryPipeline.
                    c.shape(),            // The character’s shape.
                    c.position(),         // The character’s initial position.
                    vector![
                        velocity.x * time_step,
                        velocity.y * time_step,
                        velocity.z * time_step
                    ],
                    QueryFilter::default()
                        // Make sure the character we are trying to move isn’t considered an obstacle.
                        .exclude_collider(collider),
                    |collision| {
                    }, // We don’t care about events in this example.
                );

                let c = this.collider_set.get_mut(collider).unwrap();

                c.set_translation(c.translation() + corrected_movement.translation);

                Ok((
                    lua.to_value(&c.translation()).unwrap(),
                    corrected_movement.grounded,
                    corrected_movement.is_sliding_down_slope,
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_convex_mesh",
            "info": "Create a convex mesh collider out of a point cloud."
        }
        */
        method.add_method_mut("collider_convex_mesh", |lua, this, data: LuaValue| {
            let data: Vec<Vector3> = lua.from_value(data)?;
            let data: Vec<Point<f32>> = data.iter().map(|e| point![e.x, e.y, e.z]).collect();

            if let Some(collider) = ColliderBuilder::convex_hull(&data) {
                this.collider_set.insert(collider);
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:step",
            "info": "Step the Rapier simulation."
        }
        */
        method.add_method_mut("step", |_, this, _: ()| {
            this.physics_pipeline.step(
                &vector![0.0, -9.81, 0.0],
                &this.integration_parameters,
                &mut this.island_manager,
                &mut this.broad_phase,
                &mut this.narrow_phase,
                &mut this.rigid_body_set,
                &mut this.collider_set,
                &mut this.impulse_joint_set,
                &mut this.multibody_joint_set,
                &mut this.ccd_solver,
                Some(&mut this.query_pipeline),
                &(),
                &(),
            );

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:debug_render",
            "info": "Render the Rapier simulation."
        }
        */
        method.add_method_mut("debug_render", |_, this, _: ()| {
            this.debug_render.render(
                &mut DebugRender,
                &this.rigid_body_set,
                &this.collider_set,
                &this.impulse_joint_set,
                &this.multibody_joint_set,
                &this.narrow_phase,
            );

            Ok(())
        });
    }
}

struct DebugRender;

impl DebugRenderBackend for DebugRender {
    fn draw_line(
        &mut self,
        object: DebugRenderObject<'_>,
        a: nalgebra::OPoint<f32, nalgebra::Const<3>>,
        b: nalgebra::OPoint<f32, nalgebra::Const<3>>,
        color: [f32; 4],
    ) {
        unsafe {
            ffi::DrawLine3D(
                Vector3::new(a.x, a.y, a.z).into(),
                Vector3::new(b.x, b.y, b.z).into(),
                Color::new(
                    (255.0 * color[0]) as u8,
                    (255.0 * color[1]) as u8,
                    (255.0 * color[2]) as u8,
                    (255.0 * color[3]) as u8,
                )
                .into(),
            );
        }
    }
}
