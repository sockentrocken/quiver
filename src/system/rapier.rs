/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

use rapier3d::control::CharacterLength;
use std::sync::{Arc, Mutex};

use mlua::prelude::*;
use rapier3d::{
    control::{CharacterAutostep, KinematicCharacterController},
    parry,
    prelude::*,
};
use raylib::prelude::*;
use serde::Serialize;

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
    integration_parameter: IntegrationParameters,
    simulation_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    event_handler: QuiverHandler,
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

    fn insert_collider(
        &mut self,
        lua: &Lua,
        collider: ColliderBuilder,
        rigid_body: Option<LuaValue>,
    ) -> mlua::Result<LuaValue> {
        let collider = collider
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .active_collision_types(ActiveCollisionTypes::all());

        if let Some(rigid_body) = rigid_body {
            let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;

            lua.to_value(&self.collider_set.insert_with_parent(
                collider,
                rigid_body,
                &mut self.rigid_body_set,
            ))
        } else {
            lua.to_value(&self.collider_set.insert(collider))
        }
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
                { "name": "ray",           "info": "Ray to cast.", "kind": "ray"     },
                { "name": "length",        "info": "Ray length.",  "kind": "number"  },
                { "name": "solid",         "info": "",             "kind": "boolean" },
                { "name": "exclude_rigid", "info": "",             "kind": "table"   }
            ],
            "result": [
                { "name": "rigid_body", "info": "Rigid body handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "cast_ray",
            |lua,
             this,
             (ray, length, solid, exclude_rigid, exclude_collider): (
                LuaValue,
                f32,
                bool,
                Option<LuaValue>,
                Option<LuaValue>,
            )| {
                let ray: raylib::math::Ray = lua.from_value(ray)?;
                let ray = rapier3d::geometry::Ray::new(
                    point![ray.position.x, ray.position.y, ray.position.z],
                    vector![ray.direction.x, ray.direction.y, ray.direction.z],
                );

                let mut filter = QueryFilter::default();

                if let Some(rigid) = exclude_rigid {
                    filter = filter.exclude_rigid_body(lua.from_value(rigid)?);
                }

                if let Some(collider) = exclude_collider {
                    filter = filter.exclude_collider(lua.from_value(collider)?);
                }

                if let Some((handle, time)) = this.query_pipeline.cast_ray(
                    &this.rigid_body_set,
                    &this.collider_set,
                    &ray,
                    length,
                    solid,
                    filter,
                ) {
                    return Ok((lua.to_value(&handle)?, time));
                }

                Ok((mlua::Nil, 0.0))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:cast_ray_normal",
            "info": "Cast a ray, and also get the normal information..",
            "member": [
                { "name": "ray",           "info": "Ray to cast.", "kind": "ray"     },
                { "name": "length",        "info": "Ray length.",  "kind": "number"  },
                { "name": "solid",         "info": "",             "kind": "boolean" },
                { "name": "exclude_rigid", "info": "",             "kind": "table"   }
            ],
            "result": [
                { "name": "rigid_body", "info": "Rigid body handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "cast_ray_normal",
            |lua,
             this,
             (ray, length, solid, exclude_rigid, exclude_collider): (
                LuaValue,
                f32,
                bool,
                Option<LuaValue>,
                Option<LuaValue>,
            )| {
                let ray: raylib::math::Ray = lua.from_value(ray)?;
                let ray = rapier3d::geometry::Ray::new(
                    point![ray.position.x, ray.position.y, ray.position.z],
                    vector![ray.direction.x, ray.direction.y, ray.direction.z],
                );

                let mut filter = QueryFilter::default();

                if let Some(rigid) = exclude_rigid {
                    filter = filter.exclude_rigid_body(lua.from_value(rigid)?);
                }

                if let Some(collider) = exclude_collider {
                    filter = filter.exclude_collider(lua.from_value(collider)?);
                }

                if let Some((handle, normal)) = this.query_pipeline.cast_ray_and_get_normal(
                    &this.rigid_body_set,
                    &this.collider_set,
                    &ray,
                    length,
                    solid,
                    filter,
                ) {
                    return Ok((
                        lua.to_value(&handle)?,
                        normal.normal.x,
                        normal.normal.y,
                        normal.normal.z,
                    ));
                }

                Ok((mlua::Nil, 0.0, 0.0, 0.0))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:test_intersect_cuboid_cuboid",
            "info": ""
        }
        */
        method.add_method_mut(
            "test_intersect_cuboid_cuboid",
            |lua,
             _,
             (point_a, angle_a, shape_a, point_b, angle_b, shape_b): (
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
            )| {
                let point: Vector3 = lua.from_value(point_a)?;
                let angle: Vector3 = lua.from_value(angle_a)?;
                let shape: Vector3 = lua.from_value(shape_a)?;
                let point_a = Isometry::new(
                    vector![point.x, point.y, point.z],
                    vector![angle.x, angle.y, angle.z],
                );
                let shape_a = Cuboid::new(vector![shape.x, shape.y, shape.z]);

                let point: Vector3 = lua.from_value(point_b)?;
                let angle: Vector3 = lua.from_value(angle_b)?;
                let shape: Vector3 = lua.from_value(shape_b)?;
                let point_b = Isometry::new(
                    vector![point.x, point.y, point.z],
                    vector![angle.x, angle.y, angle.z],
                );
                let shape_b = Cuboid::new(vector![shape.x, shape.y, shape.z]);

                Ok(
                    parry::query::intersection_test(&point_a, &shape_a, &point_b, &shape_b)
                        .unwrap(),
                )
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:test_intersect_cuboid",
            "info": ""
        }
        */
        method.add_method_mut(
            "test_intersect_cuboid",
            |lua,
             this,
             (point, angle, shape, exclude_rigid, exclude_collider): (
                LuaValue,
                LuaValue,
                LuaValue,
                Option<LuaValue>,
                Option<LuaValue>,
            )| {
                let point: Vector3 = lua.from_value(point)?;
                let angle: Vector3 = lua.from_value(angle)?;
                let shape: Vector3 = lua.from_value(shape)?;
                let point = Isometry::new(
                    vector![point.x, point.y, point.z],
                    vector![angle.x, angle.y, angle.z],
                );
                let shape = Cuboid::new(vector![shape.x, shape.y, shape.z]);

                let mut filter = QueryFilter::default();

                if let Some(rigid) = exclude_rigid {
                    filter = filter.exclude_rigid_body(lua.from_value(rigid)?);
                }

                if let Some(collider) = exclude_collider {
                    filter = filter.exclude_collider(lua.from_value(collider)?);
                }

                filter = filter.exclude_sensors();

                let mut hit: Option<ColliderHandle> = None;

                this.query_pipeline.intersections_with_shape(
                    &this.rigid_body_set,
                    &this.collider_set,
                    &point,
                    &shape,
                    filter,
                    |handle| {
                        hit = Some(handle);
                        true // Return `false` instead if we want to stop searching for other colliders that contain this point.
                    },
                );

                if let Some(hit) = hit {
                    lua.to_value(&hit)
                } else {
                    Ok(mlua::Nil)
                }
            },
        );

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_collider_shape_cuboid",
            "info": "Get the shape of a collider (cuboid).",
            "member": [
                { "name": "collider", "info": "Collider handle.", "kind": "table" }
            ],
            "result": [
                { "name": "half_shape_x", "info": "Half-shape of the cuboid. (X).", "kind": "number" },
                { "name": "half_shape_y", "info": "Half-shape of the cuboid. (Y).", "kind": "number" },
                { "name": "half_shape_z", "info": "Half-shape of the cuboid. (Z).", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "get_collider_shape_cuboid",
            |lua, this, collider: LuaValue| {
                let collider: ColliderHandle = lua.from_value(collider)?;

                if let Some(collider) = this.collider_set.get(collider) {
                    if let Some(shape) = collider.shape().as_cuboid() {
                        return Ok((
                            shape.half_extents.x,
                            shape.half_extents.y,
                            shape.half_extents.z,
                        ));
                    } else {
                        return Err(mlua::Error::runtime(
                            "rapier:get_collider_shape_cuboid(): Collider is not a cuboid.",
                        ));
                    }
                }

                Err(mlua::Error::runtime(
                    "rapier:get_collider_shape_cuboid(): Invalid collider handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_collider_shape_cuboid",
            "info": "Set the shape of a collider (cuboid).",
            "member": [
                { "name": "collider",   "info": "Collider handle.",      "kind": "table"    },
                { "name": "half_shape", "info": "Half-shape of cuboid.", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_collider_shape_cuboid",
            |lua, this, (collider, half_shape): (LuaValue, LuaValue)| {
                let collider: ColliderHandle = lua.from_value(collider)?;
                let half_shape: Vector3 = lua.from_value(half_shape)?;

                if let Some(collider) = this.collider_set.get_mut(collider) {
                    if let Some(shape) = collider.shape_mut().as_cuboid_mut() {
                        shape.half_extents.x = half_shape.x;
                        shape.half_extents.y = half_shape.y;
                        shape.half_extents.z = half_shape.z;

                        return Ok(());
                    } else {
                        return Err(mlua::Error::runtime(
                            "rapier:set_collider_shape_cuboid(): Collider is not a cuboid.",
                        ));
                    }
                }

                Err(mlua::Error::runtime(
                    "rapier:set_collider_shape_cuboid(): Invalid collider handle.",
                ))
            },
        );

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_collider_parent",
            "info": "Get the parent of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.", "kind": "table" }
            ],
            "result": [
                { "name": "rigid_body",   "info": "Rigid body handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("get_collider_parent", |lua, this, collider: LuaValue| {
            let collider: ColliderHandle = lua.from_value(collider)?;

            if let Some(collider) = this.collider_set.get(collider) {
                if let Some(parent) = collider.parent() {
                    return lua.to_value(&parent);
                } else {
                    return Ok(mlua::Nil);
                }
            }

            Err(mlua::Error::runtime(
                "rapier:get_collider_parent(): Invalid collider handle.",
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_collider_position",
            "info": "Get the position of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.", "kind": "table" }
            ],
            "result": [
                { "name": "position_x", "info": "Collider position (X).", "kind": "number" },
                { "name": "position_y", "info": "Collider position (Y).", "kind": "number" },
                { "name": "position_z", "info": "Collider position (Z).", "kind": "number" }
            ]
        }
        */
        method.add_method_mut("get_collider_position", |lua, this, collider: LuaValue| {
            let collider: ColliderHandle = lua.from_value(collider)?;

            if let Some(collider) = this.collider_set.get(collider) {
                return Ok((
                    collider.translation().x,
                    collider.translation().y,
                    collider.translation().z,
                ));
            }

            Err(mlua::Error::runtime(
                "rapier:get_collider_position(): Invalid collider handle.",
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_collider_position",
            "info": "Set the position of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.",   "kind": "table"    },
                { "name": "position", "info": "Collider position.", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_collider_translation",
            |lua, this, (collider, position): (LuaValue, LuaValue)| {
                let collider: ColliderHandle = lua.from_value(collider)?;
                let position: Vector3 = lua.from_value(position)?;

                if let Some(collider) = this.collider_set.get_mut(collider) {
                    collider.set_translation(vector![position.x, position.y, position.z,]);
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_collider_position(): Invalid collider handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_collider_rotation",
            "info": "Set the rotation of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.",   "kind": "table"    },
                { "name": "rotation", "info": "Collider rotation.", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_collider_rotation",
            |lua, this, (collider, rotation): (LuaValue, LuaValue)| {
                let collider: ColliderHandle = lua.from_value(collider)?;
                let rotation: Vector3 = lua.from_value(rotation)?;

                if let Some(collider) = this.collider_set.get_mut(collider) {
                    collider
                        .set_rotation(Rotation::new(vector![rotation.x, rotation.y, rotation.z]));
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_collider_rotation(): Invalid collider handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_collider_sensor",
            "info": "Set the sensor state of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.",       "kind": "table"   },
                { "name": "sensor",   "info": "Collider sensor state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut(
            "set_collider_sensor",
            |lua, this, (collider, sensor): (LuaValue, bool)| {
                let collider: ColliderHandle = lua.from_value(collider)?;

                if let Some(collider) = this.collider_set.get_mut(collider) {
                    collider.set_sensor(sensor);
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_collider_sensor(): Invalid collider handle.",
                ))
            },
        );

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_remove",
            "info": "Remove a collider.",
            "member": [
                { "name": "collider",    "info": "Collider handle.",                                                           "kind": "table"   },
                { "name": "wake_parent", "info": "Whether or not to wake up the rigid body parent this collider is bound to.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut(
            "collider_remove",
            |lua, this, (collider, wake_parent): (LuaValue, bool)| {
                let collider: ColliderHandle = lua.from_value(collider)?;

                this.collider_set.remove(
                    collider,
                    &mut this.island_manager,
                    &mut this.rigid_body_set,
                    wake_parent,
                );

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:rigid_body_remove",
            "info": "Remove a rigid body.",
            "member": [
                { "name": "rigid_body",      "info": "Rigid body handle.",                                                   "kind": "table"   },
                { "name": "remove_collider", "info": "Whether or not to remove every collider this rigid body is bound to.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut(
            "rigid_body_remove",
            |lua, this, (rigid_body, remove_collider): (LuaValue, bool)| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;

                this.rigid_body_set.remove(
                    rigid_body,
                    &mut this.island_manager,
                    &mut this.collider_set,
                    &mut this.impulse_joint_set,
                    &mut this.multibody_joint_set,
                    remove_collider,
                );

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:character_controller",
            "info": "Create a character controller.",
            "result": [
                { "name": "character_controller", "info": "Character controller.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("character_controller", |lua, _, _: ()| {
            lua.to_value(&KinematicCharacterController::default())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_character_controller_up_vector",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_character_controller_up_vector",
            |lua, _, (character, up): (LuaValue, LuaValue)| {
                let mut character: KinematicCharacterController = lua.from_value(character)?;
                let up: Vector3 = lua.from_value(up)?;
                character.up = UnitVector::new_normalize(vector![up.x, up.y, up.z]);
                lua.to_value(&character)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_character_controller_slope",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_character_controller_slope",
            |lua, _, (character, slope_min, slope_max): (LuaValue, f32, f32)| {
                let mut character: KinematicCharacterController = lua.from_value(character)?;
                character.min_slope_slide_angle = slope_min;
                character.max_slope_climb_angle = slope_max;
                lua.to_value(&character)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_character_auto_step",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_character_auto_step",
            |lua,
             _,
             (character, kind_a, kind_b, value_a, value_b, dynamic): (
                LuaValue,
                i32,
                i32,
                f32,
                f32,
                bool,
            )| {
                let mut character: KinematicCharacterController = lua.from_value(character)?;
                if kind_a == 0 || kind_b == 0 {
                    character.autostep = None;
                } else {
                    let auto = {
                        Some(CharacterAutostep {
                            max_height: {
                                match kind_a {
                                    1 => CharacterLength::Absolute(value_a),
                                    _ => CharacterLength::Relative(value_a),
                                }
                            },
                            min_width: {
                                match kind_b {
                                    1 => CharacterLength::Absolute(value_b),
                                    _ => CharacterLength::Relative(value_b),
                                }
                            },
                            include_dynamic_bodies: dynamic,
                        })
                    };

                    character.autostep = auto;
                }
                lua.to_value(&character)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_character_snap_ground",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_character_snap_ground",
            |lua, _, (character, kind, value): (LuaValue, i32, f32)| {
                let mut character: KinematicCharacterController = lua.from_value(character)?;
                let snap = match kind {
                    1 => Some(CharacterLength::Absolute(value)),
                    2 => Some(CharacterLength::Relative(value)),
                    _ => None,
                };

                character.snap_to_ground = snap;

                lua.to_value(&character)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:character_controller_move",
            "info": "Move a character controller.",
            "member": [
                { "name": "step",        "info": "", "kind": "number"   },
                { "name": "character",   "info": "", "kind": "table"    },
                { "name": "collider",    "info": "", "kind": "table"    },
                { "name": "translation", "info": "", "kind": "vector_3" }
            ],
            "result": [
                { "name": "movement_x", "info": "Translation point (X).", "kind": "number"  },
                { "name": "movement_y", "info": "Translation point (Y).", "kind": "number"  },
                { "name": "movement_z", "info": "Translation point (Z).", "kind": "number"  },
                { "name": "floor",      "info": "Currently on floor.",    "kind": "boolean" },
                { "name": "slope",      "info": "Currently on slope.",    "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut(
            "character_controller_move",
            |lua, this, (step, character, collider, translation): (f32, LuaValue, LuaValue, LuaValue)| {
                let character: KinematicCharacterController = lua.from_value(character)?;
                let collider_h: ColliderHandle = lua.from_value(collider)?;
                let collider_r = this.collider_set.get(collider_h).unwrap();
                let translation: Vector3 = lua.from_value(translation)?;

                let movement = character.move_shape(
                    step,                 // The timestep length (can be set to SimulationSettings::dt).
                    &this.rigid_body_set, // The RigidBodySet.
                    &this.collider_set,   // The ColliderSet.
                    &this.query_pipeline, // The QueryPipeline.
                    collider_r.shape(),      // The character’s shape.
                    collider_r.position(),        // The character’s initial position.
                    vector![translation.x * step, translation.y * step, translation.z * step],
                    QueryFilter::default()
                        // Make sure the character we are trying to move isn’t considered an obstacle.
                        .exclude_collider(collider_h)
                        .exclude_sensors(),
                    |_| {}, // We don’t care about events in this example.
                );

                let collider_r = this.collider_set.get_mut(collider_h).unwrap();
                collider_r.set_translation(collider_r.translation() + movement.translation);

                Ok((
                    collider_r.translation().x,
                    collider_r.translation().y,
                    collider_r.translation().z,
                    movement.grounded,
                    movement.is_sliding_down_slope
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:rigid_body",
            "info": "Create a rigid body.",
            "member": [
                { "name": "kind", "info": "Rigid body kind.", "kind": "rigid_body_kind" }
            ],
            "result": [
                { "name": "rigid_body", "info": "Rigid body handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("rigid_body", |lua, this, kind: i32| {
            let rigid = match kind {
                _ => RigidBodyBuilder::fixed(),
                1 => RigidBodyBuilder::dynamic(),
                2 => RigidBodyBuilder::kinematic_position_based(),
                3 => RigidBodyBuilder::kinematic_velocity_based(),
            };

            lua.to_value(&this.rigid_body_set.insert(rigid))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_rigid_body_user_data",
            "info": "Get the user data of a rigid_body.",
            "member": [
                { "name": "rigid_body", "info": "Rigid body handle.",  "kind": "userdata" }
            ],
            "result": [
                { "name": "user_data",  "info": "Rigid body user data.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "get_rigid_body_user_data",
            |lua, this, rigid_body: LuaValue| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;

                if let Some(rigid_body) = this.rigid_body_set.get(rigid_body) {
                    return Ok(rigid_body.user_data);
                }

                Err(mlua::Error::runtime(
                    "rapier:get_rigid_body_user_data(): Invalid rigid body handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_rigid_body_user_data",
            "info": "Set the user data of a rigid_body.",
            "member": [
                { "name": "rigid_body", "info": "Rigid body handle.",    "kind": "userdata" },
                { "name": "user_data",  "info": "Rigid body user data.", "kind": "number"   }
            ]
        }
        */
        method.add_method_mut(
            "set_rigid_body_user_data",
            |lua, this, (rigid_body, user_data): (LuaValue, u128)| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;

                if let Some(rigid_body) = this.rigid_body_set.get_mut(rigid_body) {
                    rigid_body.user_data = user_data;
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_rigid_body_user_data(): Invalid rigid body handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_rigid_body_position",
            "info": "Set the position of a rigid_body.",
            "member": [
                { "name": "rigid_body", "info": "rigid_body handle.",   "kind": "userdata" },
                { "name": "position",   "info": "rigid_body position.", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_rigid_body_position",
            |lua, this, (rigid_body, position, wake_up): (LuaValue, LuaValue, bool)| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;
                let position: Vector3 = lua.from_value(position)?;

                if let Some(rigid_body) = this.rigid_body_set.get_mut(rigid_body) {
                    rigid_body
                        .set_translation(vector![position.x, position.y, position.z,], wake_up);
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_rigid_body_position(): Invalid rigid_body handle.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_rigid_body_rotation",
            "info": "Set the rotation of a rigid_body.",
            "member": [
                { "name": "rigid_body", "info": "rigid_body handle.",   "kind": "table"    },
                { "name": "rotation", "info": "rigid_body rotation.", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_rigid_body_rotation",
            |lua, this, (rigid_body, rotation, wake_up): (LuaValue, LuaValue, bool)| {
                let rigid_body: RigidBodyHandle = lua.from_value(rigid_body)?;
                let rotation: Vector3 = lua.from_value(rotation)?;

                if let Some(rigid_body) = this.rigid_body_set.get_mut(rigid_body) {
                    rigid_body.set_rotation(
                        Rotation::new(vector![rotation.x, rotation.y, rotation.z]),
                        wake_up,
                    );
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_rigid_body_rotation(): Invalid rigid_body handle.",
                ))
            },
        );

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:get_collider_user_data",
            "info": "Get the user data of a collider.",
            "member": [
                { "name": "collider", "info": "Collider handle.", "kind": "userdata" }
            ],
            "result": [
                { "name": "user_data", "info": "Collider user data.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut("get_collider_user_data", |lua, this, collider: LuaValue| {
            let collider: ColliderHandle = lua.from_value(collider)?;

            if let Some(collider) = this.collider_set.get(collider) {
                return Ok(collider.user_data);
            }

            Err(mlua::Error::runtime(
                "rapier:get_collider_user_data(): Invalid collider handle.",
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:set_collider_user_data",
            "info": "Set the user data of a collider.",
            "member": [
                { "name": "collider",  "info": "Collider handle.",    "kind": "userdata" },
                { "name": "user_data", "info": "Collider user data.", "kind": "number"   }
            ]
        }
        */
        method.add_method_mut(
            "set_collider_user_data",
            |lua, this, (collider, user_data): (LuaValue, u128)| {
                let collider: ColliderHandle = lua.from_value(collider)?;

                if let Some(collider) = this.collider_set.get_mut(collider) {
                    collider.user_data = user_data;
                    return Ok(());
                }

                Err(mlua::Error::runtime(
                    "rapier:set_collider_user_data(): Invalid collider handle.",
                ))
            },
        );

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_builder_cuboid",
            "info": "Create a collider builder (cuboid).",
            "member": [
                { "name": "half_shape", "info": "Half-shape of cuboid.", "kind": "vector_3" }
            ],
            "result": [
                { "name": "collider_builer", "info": "Collider builder.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "collider_builder_cuboid",
            |lua, this, (half_shape, rigid_body): (LuaValue, Option<LuaValue>)| {
                let half_shape: Vector3 = lua.from_value(half_shape)?;

                this.insert_collider(
                    lua,
                    ColliderBuilder::cuboid(half_shape.x, half_shape.y, half_shape.z),
                    rigid_body,
                )
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_builder_tri_mesh",
            "info": "Create a collider builder (tri-mesh).",
            "member": [
                { "name": "point_table", "info": "The point array table.", "kind": "table" },
                { "name": "index_table", "info": "The index array table.", "kind": "table" }
            ],
            "result": [
                { "name": "collider_builer", "info": "Collider builder.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "collider_builder_tri_mesh",
            |lua, this, (point_table, index_table, rigid_body): (LuaValue, LuaValue, Option<LuaValue>)| {
                let mut p_table: Vec<Point<f32>> = Vec::new();
                let mut i_table: Vec<[u32; 3]> = Vec::new();
                let point_table: Vec<Vector3> = lua.from_value(point_table)?;
                let index_table: Vec<u32> = lua.from_value(index_table)?;

                for x in point_table {
                    p_table.push(point![x.x, x.y, x.z]);
                }

                let mut iterator = index_table.iter();

                while let Some(a) = iterator.next() {
                    if let Some(b) = iterator.next() {
                        if let Some(c) = iterator.next() {
                            i_table.push([*a, *b, *c]);
                        }
                    }
                }

                // TO-DO this should really be a convex_mesh call, but for some reason, it doesn't work, no matter what input is sent?
                this.insert_collider(
                    lua,
                    ColliderBuilder::trimesh_with_flags(
                        p_table,
                        i_table,
                        TriMeshFlags::all(),
                    ),
                    rigid_body,
                )
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:collider_builder_convex_hull",
            "info": "Create a collider builder (convex hull).",
            "member": [
                { "name": "vector_table", "info": "A vector_3 vertex array table.", "kind": "table" }
            ],
            "result": [
                { "name": "collider_builer", "info": "Collider builder.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "collider_builder_convex_hull",
            |lua, this, (vector_table, rigid_body): (LuaValue, Option<LuaValue>)| {
                let mut point_table: Vec<Point<f32>> = Vec::new();
                let vector_table: Vec<Vector3> = lua.from_value(vector_table)?;

                for x in vector_table {
                    point_table.push(point![x.x, x.y, x.z]);
                }

                if let Some(collider) = ColliderBuilder::convex_hull(&point_table) {
                    this.insert_collider(lua, collider, rigid_body)
                } else {
                    Ok(mlua::Nil)
                }
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "rapier:step",
            "info": "Step the Rapier simulation."
        }
        */
        method.add_method_mut("step", |lua, this, _: ()| {
            {
                let mut list = this.event_handler.event_list.lock().unwrap();
                list.clear();
            }

            this.simulation_pipeline.step(
                &vector![0.0, -9.81, 0.0],
                &this.integration_parameter,
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
                &this.event_handler,
            );

            let list = this.event_handler.event_list.lock().unwrap();

            if !list.is_empty() {
                lua.to_value(&*list)
            } else {
                Ok(mlua::Nil)
            }
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

#[derive(Default)]
struct QuiverHandler {
    event_list: Arc<Mutex<Vec<QuiverEvent>>>,
}

#[derive(Serialize)]
struct QuiverEvent {
    handle_a: ColliderHandle,
    handle_b: ColliderHandle,
    flag: CollisionEventFlags,
    start: bool,
}

impl Default for QuiverEvent {
    fn default() -> Self {
        Self {
            handle_a: ColliderHandle::default(),
            handle_b: ColliderHandle::default(),
            flag: CollisionEventFlags::empty(),
            start: false,
        }
    }
}

impl QuiverHandler {
    pub fn new() -> Self {
        Self {
            event_list: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventHandler for QuiverHandler {
    fn handle_collision_event(
        &self,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        event: CollisionEvent,
        contact_pair: Option<&ContactPair>,
    ) {
        let mut lock = self.event_list.lock().unwrap();
        match event {
            CollisionEvent::Started(collider_handle, collider_handle1, collision_event_flags) => {
                lock.push(QuiverEvent {
                    handle_a: collider_handle,
                    handle_b: collider_handle1,
                    flag: collision_event_flags,
                    start: true,
                });
            }
            CollisionEvent::Stopped(collider_handle, collider_handle1, collision_event_flags) => {
                lock.push(QuiverEvent {
                    handle_a: collider_handle,
                    handle_b: collider_handle1,
                    flag: collision_event_flags,
                    start: false,
                });
            }
        }
    }

    fn handle_contact_force_event(
        &self,
        dt: f32,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        contact_pair: &ContactPair,
        total_force_magnitude: f32,
    ) {
        println!("bar");
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
