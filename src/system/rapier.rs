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
use rapier3d::control::KinematicCharacterController;
use rapier3d::prelude::*;
use raylib::prelude::*;
use serde::Deserialize;

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
                { "name": "filter_rigid", "info": "", "kind": "table"  },
                { "name": "filter_collider", "info": "", "kind": "table"  }
            ],
            "result": [
                { "name": "pick", "info": "", "kind": "boolean" },
                { "name": "time", "info": "", "kind": "number"  }
            ]
        }
        */
        method.add_method_mut(
            "cast_ray",
            |lua, this, (ray, time, filter_rigid, filter_collider): (LuaValue, f32, LuaValue, LuaValue)| {
                let ray: raylib::math::Ray = lua.from_value(ray)?;
                let filter_rigid: Vec<RigidBodyHandle> = lua.from_value(filter_rigid)?;
                let filter_collider: Vec<ColliderHandle> = lua.from_value(filter_collider)?;
                let mut query = QueryFilter::new();

                for f in filter_rigid {
                    query = query.exclude_rigid_body(f);
                }

                for f in filter_collider {
                    query = query.exclude_collider(f);
                }

                if let Some(hit) = this.query_pipeline.cast_ray_and_get_normal(
                    &this.rigid_body_set,
                    &this.collider_set,
                    &rapier3d::geometry::Ray::new(
                        point![ray.position.x, ray.position.y, ray.position.z],
                        vector![ray.direction.x, ray.direction.y, ray.direction.z],
                    ),
                    time,
                    true,
                    query,
                ) {
                    let collider = this.collider_set.get(hit.0).unwrap();

                    return Ok((
                        collider.user_data,
                        lua.to_value(&collider)?,
                        lua.to_value(&hit.1)?,
                    ));
                }

                Ok((u128::MAX, mlua::Nil, mlua::Nil))
            },
        );

        #[derive(Deserialize)]
        struct RigidBodyInfo {
            kind: RigidBodyType,
            user: Option<u128>,
            position: Option<Vector3>,
            rotation: Option<Vector3>,
            lin_velocity: Option<Vector3>,
            ang_velocity: Option<Vector3>,
            gravity_scale: Option<f32>,
            can_sleep: Option<bool>,
            continous: Option<bool>,
        }

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:create_rigid_body",
            "info": "Create a rigid body.",
            "member": [
                { "name": "data", "info": "Rigid body data.", "kind": "rigid_body_info" }
            ],
            "result": [
                { "name": "rigid_body", "info": "Rigid body.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_rigid_body", |lua, this, data: LuaValue| {
            let info: RigidBodyInfo = lua.from_value(data)?;

            let mut rigid_body = match info.kind {
                RigidBodyType::Dynamic => RigidBodyBuilder::dynamic(),
                RigidBodyType::Fixed => RigidBodyBuilder::fixed(),
                RigidBodyType::KinematicPositionBased => {
                    RigidBodyBuilder::kinematic_position_based()
                }
                RigidBodyType::KinematicVelocityBased => {
                    RigidBodyBuilder::kinematic_velocity_based()
                }
            };

            if let Some(data) = info.user {
                rigid_body = rigid_body.user_data(data);
            }
            if let Some(data) = info.position {
                rigid_body = rigid_body.translation(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.rotation {
                rigid_body = rigid_body.rotation(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.lin_velocity {
                rigid_body = rigid_body.linvel(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.ang_velocity {
                rigid_body = rigid_body.angvel(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.gravity_scale {
                rigid_body = rigid_body.gravity_scale(data);
            }
            if let Some(data) = info.can_sleep {
                rigid_body = rigid_body.can_sleep(data);
            }
            if let Some(data) = info.continous {
                rigid_body = rigid_body.ccd_enabled(data);
            }

            lua.to_value(&this.rigid_body_set.insert(rigid_body))
        });

        #[derive(Deserialize)]
        enum ColliderKind {
            Ball(f32),
            Cube(f32, f32, f32),
        }

        #[derive(Deserialize)]
        struct ColliderInfo {
            kind: ColliderKind,
            user: Option<u128>,
            rigid_body: Option<RigidBodyHandle>,
            position: Option<Vector3>,
            rotation: Option<Vector3>,
            density: Option<f32>,
            friction: Option<f32>,
            sensor: Option<bool>,
        }

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:create_collider",
            "info": "Create a collider.",
            "member": [
                { "name": "data", "info": "Collider data.", "kind": "collider_info" }
            ],
            "result": [
                { "name": "collider", "info": "Collider.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_collider", |lua, this, data: LuaValue| {
            let info: ColliderInfo = lua.from_value(data)?;

            let mut collider = match info.kind {
                ColliderKind::Ball(x) => ColliderBuilder::ball(x),
                ColliderKind::Cube(x, y, z) => ColliderBuilder::cuboid(x, y, z),
            };

            if let Some(data) = info.user {
                collider = collider.user_data(data);
            }
            if let Some(data) = info.position {
                collider = collider.translation(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.rotation {
                collider = collider.rotation(vector![data.x, data.y, data.z]);
            }
            if let Some(data) = info.density {
                collider = collider.density(data);
            }
            if let Some(data) = info.friction {
                collider = collider.friction(data);
            }
            if let Some(data) = info.sensor {
                collider = collider.sensor(data);
            }

            if let Some(rigid_body) = info.rigid_body {
                lua.to_value(&this.collider_set.insert_with_parent(
                    collider,
                    rigid_body,
                    &mut this.rigid_body_set,
                ))
            } else {
                lua.to_value(&this.collider_set.insert(collider))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:create_sphere",
            "info": ""
        }
        */
        method.add_method_mut("create_sphere", |lua, this, index: u128| {
            let rigid_body = RigidBodyBuilder::dynamic()
                .user_data(index)
                .translation(vector![0.0, 10.0, 0.0])
                .build();
            let collider = ColliderBuilder::ball(0.5)
                .user_data(index)
                .restitution(0.7)
                .build();
            let rigid_body = this.rigid_body_set.insert(rigid_body);
            let collider = this.collider_set.insert_with_parent(
                collider,
                rigid_body,
                &mut this.rigid_body_set,
            );

            Ok((lua.to_value(&rigid_body)?, lua.to_value(&collider)?))
        });

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
            "name": "rapier:set_rigid_body_data",
            "info": ""
        }
        */
        method.add_method_mut(
            "set_rigid_body_data",
            |lua, this, (rigid_body, data): (LuaValue, LuaValue)| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;
                let data: Vector3 = lua.from_value(data)?;

                let rigid_body = this.rigid_body_set.get_mut(rigid_body).unwrap();

                rigid_body.set_linvel(vector![data.x, data.y, data.z], true);

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_rigid_body_data",
            "info": ""
        }
        */
        method.add_method_mut("get_rigid_body_data", |lua, this, rigid_body: LuaValue| {
            let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;

            lua.to_value(&this.rigid_body_set.get(rigid_body).unwrap())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_collider_data",
            "info": ""
        }
        */
        method.add_method_mut("get_collider_data", |lua, this, collider: LuaValue| {
            let collider: ColliderHandle = lua.from_value(collider)?;

            lua.to_value(&this.collider_set.get(collider).unwrap())
        });

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
                { "name": "controller", "info": "Controller.",           "kind": "table"    },
                { "name": "collider",   "info": "Collider.",             "kind": "table"    },
                { "name": "velocity",   "info": "Velocity.",             "kind": "vector_3" },
                { "name": "time_step",  "info": "Time step.",            "kind": "number"   }
            ],
            "result": [
                { "name": "translation", "info": "Translation.", 		"kind": "vector_3" },
                { "name": "floor", 		 "info": "Currently on floor.", "kind": "boolean"  },
                { "name": "slide", 		 "info": "Currently on slide.", "kind": "boolean"  },
                { "name": "collision",   "info": "Collision list.",     "kind": "table"    }
            ]
        }
        */
        method.add_method_mut(
            "move_character_controller",
            |lua,
             this,
             (controller, collider, velocity, time_step): (
                LuaValue,
                LuaValue,
                LuaValue,
                f32,
            )| {
                let controller: KinematicCharacterController = lua.from_value(controller)?;
                let collider: ColliderHandle = lua.from_value(collider)?;
                let velocity: Vector3 = lua.from_value(velocity)?;

                let mut collision_list = vec![];
                let mut translation_list = vec![];

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
                        translation_list.push(collision.hit.normal1);
                        collision_list.push(collision)
                    }, // We don’t care about events in this example.
                );

                controller.solve_character_collision_impulses(
                    time_step,
                    &mut this.rigid_body_set,
                    &this.collider_set,
                    &this.query_pipeline,
                    c.shape(),
                    c.mass(),
                    &collision_list,
                    QueryFilter::default().exclude_collider(collider),
                );

                let c = this.collider_set.get_mut(collider).unwrap();

                c.set_translation(c.translation() + corrected_movement.translation);

                Ok((
                    lua.to_value(&c.translation())?,
                    corrected_movement.grounded,
                    corrected_movement.is_sliding_down_slope,
                    lua.to_value(&translation_list)?
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
        _object: DebugRenderObject<'_>,
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
