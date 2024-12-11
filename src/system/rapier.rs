use crate::system::*;

use mlua::prelude::*;
use rapier3d::{
    control::{EffectiveCharacterMovement, KinematicCharacterController},
    prelude::*,
};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

/* class
{ "name": "quiver.rapier", "info": "The Rapier API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let rapier = lua.create_table()?;

    rapier.set("new", lua.create_function(self::Rapier::new)?)?;

    table.set("rapier", rapier)?;

    Ok(())
}

#[derive(Default)]
struct DebugRender;

/* class
{ "name": "rapier", "info": "An unique handle for a Rapier simulation in memory." }
*/
struct Rapier {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameter: IntegrationParameters,
    simulation_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    debug_pipeline: DebugRenderPipeline,
}

impl rapier3d::pipeline::DebugRenderBackend for DebugRender {
    fn draw_line(
        &mut self,
        _object: DebugRenderObject,
        a: Point<f32>,
        b: Point<f32>,
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

impl Rapier {
    /* entry
    {
        "name": "quiver.rapier.new",
        "info": "Create a new Rapier simulation.",
        "result": [
            { "name": "rapier", "info": "Rapier resource.", "kind": "rapier" }
        ]
    }
    */
    fn new(_: &Lua, _: ()) -> mlua::Result<Self> {
        Ok(Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameter: IntegrationParameters::default(),
            simulation_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            debug_pipeline: DebugRenderPipeline::new(
                DebugRenderStyle::default(),
                DebugRenderMode::all(),
            ),
        })
    }
}

impl mlua::UserData for Rapier {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "name": "rapier:debug_render", "info": "Render every object in the simulation." }
        */
        method.add_method_mut("debug_render", |_, this, ()| {
            this.debug_pipeline.render(
                &mut DebugRender,
                &this.rigid_body_set,
                &this.collider_set,
                &this.impulse_joint_set,
                &this.multibody_joint_set,
                &this.narrow_phase,
            );
            Ok(())
        });

        /* entry
        { "name": "rapier:step", "info": "" }
        */
        method.add_method_mut("step", |_, this, ()| {
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
                &(),
            );
            Ok(())
        });

        //================================================================

        /* entry
        {
            "name": "rapier:create_character_controller",
            "info": "Create a character controller.",
            "result": [
                { "name": "character_controller", "info": "Character controller handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_character_controller", |lua, _, _: ()| {
            lua.to_value(&KinematicCharacterController::default())
        });

        /* entry
        {
            "name": "rapier:move_character_controller",
            "info": "Move the character controller.",
            "member": [
                { "name": "character_controller", "info": "Character controller handle.", "kind": "table"    },
                { "name": "collider",             "info": "Collider handle.",             "kind": "table"    },
                { "name": "value",                "info": "Translation.",                 "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "move_character_controller",
            |lua, this, (character_controller, collider, value): (LuaValue, LuaValue, LuaValue)| {
                let character_controller: KinematicCharacterController =
                    lua.from_value(character_controller)?;
                let collider: ColliderHandle = lua.from_value(collider)?;
                let value: general::Vector3 = lua.from_value(value)?;

                let colli = this.collider_set.get(collider).unwrap();

                let mut collisions = vec![];

                let movement = character_controller.move_shape(
                    1.0 / 60.0,
                    &this.rigid_body_set,
                    &this.collider_set,
                    &this.query_pipeline,
                    colli.shape(),
                    colli.position(),
                    vector![value.x, value.y, value.z],
                    QueryFilter::default().exclude_collider(collider),
                    |collision| collisions.push(collision),
                );

                character_controller.solve_character_collision_impulses(
                    1.0 / 60.0,
                    &mut this.rigid_body_set,
                    &this.collider_set,
                    &this.query_pipeline,
                    colli.shape(),
                    1.0,
                    &collisions,
                    QueryFilter::default().exclude_collider(collider),
                );

                let trans = colli.translation().clone();

                let colli = this.collider_set.get_mut(collider).unwrap();

                colli.set_translation(vector![
                    trans.x + movement.translation.x,
                    trans.y + movement.translation.y,
                    trans.z + movement.translation.z
                ]);

                lua.to_value(&CharacterMove::from(movement))
            },
        );

        #[derive(Serialize)]
        pub struct CharacterMove {
            translation: general::Vector3,
            ground: bool,
            slide: bool,
        }

        impl From<EffectiveCharacterMovement> for CharacterMove {
            fn from(value: EffectiveCharacterMovement) -> Self {
                Self {
                    translation: general::Vector3::new(
                        value.translation.x,
                        value.translation.y,
                        value.translation.z,
                    ),
                    ground: value.grounded,
                    slide: value.is_sliding_down_slope,
                }
            }
        }

        //================================================================

        #[derive(Deserialize)]
        pub enum RigidBodyKind {
            Fixed = 0,
            Dynamic = 1,
            Velocity = 2,
            Position = 3,
        }

        /* class
        {
            "name": "rigid_body_info",
            "info": "Rigid body info.",
            "member": [
                { "name": "kind",      "info": "Kind.",      "kind": "number"   },
                { "name": "position",  "info": "Position.",  "kind": "vector_3" },
                { "name": "rotation",  "info": "Rotation.",  "kind": "vector_3" },
                { "name": "lin_vel",   "info": "Lin. vel.",  "kind": "vector_3" },
                { "name": "ang_vel",   "info": "Ang. vel.",  "kind": "vector_3" },
                { "name": "gravity",   "info": "Gravity.",   "kind": "number"   },
                { "name": "can_sleep", "info": "Can sleep.", "kind": "boolean"  },
                { "name": "continous", "info": "Continous.", "kind": "boolean"  }
            ]
        }
        */
        #[derive(Deserialize)]
        pub struct RigidBodyInfo {
            kind: i32,
            position: general::Vector3,
            //rotation: general::Vector3,
            //lin_vel: general::Vector3,
            //ang_vel: general::Vector3,
            //gravity: f32,
            //can_sleep: bool,
            //continous: bool,
        }

        /* entry
        {
            "name": "rapier:create_rigid_body",
            "info": "Create a rigid body.",
            "member": [
                { "name": "kind", "info": "Rigid body kind (fixed, dynamic, velocity-based, position-based).", "kind": "rigid_body_info" }
            ],
            "result": [
                { "name": "body", "info": "Rigid body handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_rigid_body", |lua, this, info: LuaValue| {
            let info: RigidBodyInfo = lua.from_value(info)?;

            let rigid_body = {
                match info.kind {
                    0 => RigidBodyBuilder::fixed(),
                    1 => RigidBodyBuilder::dynamic(),
                    2 => RigidBodyBuilder::kinematic_velocity_based(),
                    3 => RigidBodyBuilder::kinematic_position_based(),
                    _ => RigidBodyBuilder::fixed(),
                }
            }
            .translation(vector![info.position.x, info.position.y, info.position.z])
            //.rotation(vector![info.position.x, info.position.y, info.position.z])
            //.linvel(vector![info.position.x, info.position.y, info.position.z])
            //.angvel(vector![info.position.x, info.position.y, info.position.z])
            //.gravity_scale(info.gravity)
            //.can_sleep(info.can_sleep)
            //.ccd_enabled(info.continous)
            .build();

            lua.to_value(&this.rigid_body_set.insert(rigid_body))
        });

        /* entry
        {
            "name": "rapier:rigid_body_lin_vel",
            "info": "Set a rigid body's linear velocity.",
            "member": [
                { "name": "handle", "info": "Rigid body handle.", "kind": "table"    },
                { "name": "value",  "info": "Linear velocity.",   "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "rigid_body_lin_vel",
            |lua, this, (handle, value): (LuaValue, LuaValue)| {
                let handle: RigidBodyHandle = lua.from_value(handle)?;
                let handle = this.rigid_body_set.get_mut(handle).unwrap();
                let value: general::Vector3 = lua.from_value(value)?;

                handle.set_linvel(vector![value.x, value.y, value.z], true);

                Ok(())
            },
        );

        #[derive(Deserialize)]
        pub enum ColliderKind {
            Ball(f32),
            Cuboid(f32, f32, f32),
            ConvexMesh(Vec<general::Vector3>, Vec<f32>),
        }

        /* class
        {
            "name": "collider_info",
            "info": "Collider info.",
            "member": [
                { "name": "kind",      "info": "Kind.",     "kind": "table"    },
                { "name": "position",  "info": "Position.", "kind": "vector_3" },
                { "name": "rotation",  "info": "Rotation.", "kind": "vector_3" },
                { "name": "density",   "info": "Density.",  "kind": "number"   },
                { "name": "friction",  "info": "Friction.", "kind": "number"   },
                { "name": "trigger",   "info": "Trigger.",  "kind": "boolean"  }
            ]
        }
        */
        #[derive(Deserialize)]
        pub struct ColliderInfo {
            kind: ColliderKind,
            position: general::Vector3,
            //rotation: general::Vector3,
            //density: f32,
            //friction: f32,
            //sensor: bool,
        }

        /* entry
        {
            "name": "rapier:get_collider",
            "info": "Get a collider.",
            "member": [
                { "name": "handle", "info": "Collider handle.", "kind": "table" }
            ],
            "result": [
                { "name": "collider", "info": "Collider.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("get_collider", |lua, this, handle: LuaValue| {
            let handle: ColliderHandle = lua.from_value(handle)?;

            lua.to_value(&this.collider_set.get(handle))
        });

        /* entry
        {
            "name": "rapier:create_collider",
            "info": "Create a collider.",
            "member": [
                { "name": "info", "info": "Collider info.", "kind": "collider_info" }
            ],
            "result": [
                { "name": "collider", "info": "Collider handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_collider", |lua, this, info: LuaValue| {
            let info: ColliderInfo = lua.from_value(info)?;

            let collider = {
                match info.kind {
                    ColliderKind::Ball(x) => ColliderBuilder::ball(x),
                    ColliderKind::Cuboid(x, y, z) => ColliderBuilder::cuboid(x, y, z),
                    ColliderKind::ConvexMesh(vertex, index) => {
                        todo!()
                    }
                }
            }
            .restitution(0.75)
            .translation(vector![info.position.x, info.position.y, info.position.z])
            //.rotation(info.rotation)
            //.density(info.density)
            //.friction(info.friction)
            //.sensor(info.sensor)
            .build();

            lua.to_value(&this.collider_set.insert(collider))
        });

        /* entry
        {
            "name": "rapier:create_collider_parent",
            "info": "Create a collider with a parent.",
            "member": [
                { "name": "info",   "info": "Collider info.",              "kind": "table" },
                { "name": "parent", "info": "Collider rigid body parent.", "kind": "table" }
            ],
            "result": [
                { "name": "collider", "info": "Collider handle.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "create_collider_parent",
            |lua, this, (info, parent): (LuaValue, LuaValue)| {
                let info: ColliderInfo = lua.from_value(info)?;
                let parent: RigidBodyHandle = lua.from_value(parent)?;

                let collider = {
                    match info.kind {
                        ColliderKind::Ball(x) => ColliderBuilder::ball(x),
                        ColliderKind::Cuboid(x, y, z) => ColliderBuilder::cuboid(x, y, z),
                    }
                }
                .restitution(0.75)
                //.translation(info.position)
                //.rotation(info.rotation)
                //.density(info.density)
                //.friction(info.friction)
                //.sensor(info.sensor)
                .build();

                lua.to_value(&this.collider_set.insert_with_parent(
                    collider,
                    parent,
                    &mut this.rigid_body_set,
                ))
            },
        );
    }
}
