use crate::system::*;

use mlua::prelude::*;
use rapier3d::{control::KinematicCharacterController, prelude::*};
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

        /* class
        {
            "name": "rigid_body_info",
            "info": "Rigid body information.",
            "field": [
                { "name": "kind",          "info": "Rigid body type.",                          "kind": "rigid_body_type" },
                { "name": "position",      "info": "Rigid body position.",                      "kind": "vector_3" },
                { "name": "rotation",      "info": "Rigid body rotation.",                      "kind": "vector_3" },
                { "name": "lin_velocity",  "info": "Rigid body lin. velocity.",                 "kind": "vector_3" },
                { "name": "ang_velocity",  "info": "Rigid body ang. velocity.",                 "kind": "vector_3" },
                { "name": "gravity_scale", "info": "Rigid body gravity scale.",                 "kind": "number"   },
                { "name": "can_sleep",     "info": "Rigid body can sleep.",                     "kind": "boolean"  },
                { "name": "continous",     "info": "Rigid body continous collision detection.", "kind": "boolean"  }
            ]
        }
        */
        #[derive(Deserialize, Serialize)]
        struct RigidBodyInfo {
            kind: i32,
            position: general::Vector3,
            rotation: general::Vector3,
            lin_velocity: general::Vector3,
            ang_velocity: general::Vector3,
            gravity_scale: f32,
            can_sleep: bool,
            continous: bool,
        }

        /* entry
        {
            "name": "rapier:create_rigid_body",
            "info": "Create a rigid body.",
            "parameter": [
                { "name": "rigid_body", "info": "Rigid body information.", "kind": "rigid_body_info" }
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
                    _ => todo!(),
                }
            };

            rigid_body.translation(info.position);

            Ok(())
        });

        /* entry
        { "name": "rapier:create_ground", "info": "" }
        */
        method.add_method_mut("create_ground", |_, this, ()| {
            let collider = ColliderBuilder::cuboid(10.0, 0.1, 10.0).build();
            this.collider_set.insert(collider);

            Ok(())
        });

        /* entry
        { "name": "rapier:create_sphere", "info": "" }
        */
        method.add_method_mut("create_sphere", |_, this, ()| {
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![0.0, 10.0, 0.0])
                .build();
            let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
            let ball_body_handle = this.rigid_body_set.insert(rigid_body);
            this.collider_set.insert_with_parent(
                collider,
                ball_body_handle,
                &mut this.rigid_body_set,
            );

            Ok(())
        });

        /* entry
        {
            "name": "rapier:create_convex_hull",
            "info": "",
            "result": [
                { "name": "vertex", "info": "The vertex list to use for the convex hull.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "create_convex_hull_dynamic",
            |lua, this, (path, dynamic): (String, bool)| unsafe {
                let rigid_body = RigidBodyBuilder::dynamic()
                    .translation(vector![0.0, 10.0, 0.0])
                    .rotation(vector![45.0, 0.0, 0.0])
                    .build();
                let handle = this.rigid_body_set.insert(rigid_body);

                let name = std::ffi::CString::new(path.clone())
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;

                let data = ffi::LoadModel(name.as_ptr());

                let model = Model::from_raw(data);

                for mesh in model.meshes() {
                    let mut work: Vec<Point<f32>> = Vec::new();

                    for vertex in mesh.vertices() {
                        work.push(Point::new(vertex.x, vertex.y, vertex.z));
                    }

                    if let Some(collider) = ColliderBuilder::convex_hull(&work) {
                        this.collider_set.insert_with_parent(
                            collider,
                            handle,
                            &mut this.rigid_body_set,
                        );
                    } else {
                        return Err(mlua::Error::RuntimeError(
                            "Failed to generate a convex hull".to_string(),
                        ));
                    }
                }

                Ok(())
            },
        );

        /* entry
        {
            "name": "rapier:create_convex_hull",
            "info": "",
            "result": [
                { "name": "vertex", "info": "The vertex list to use for the convex hull.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut(
            "create_convex_hull",
            |lua, this, (path, dynamic): (String, bool)| unsafe {
                let name = std::ffi::CString::new(path.clone())
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;

                let data = ffi::LoadModel(name.as_ptr());

                let model = Model::from_raw(data);

                for mesh in model.meshes() {
                    let mut work: Vec<Point<f32>> = Vec::new();

                    for vertex in mesh.vertices() {
                        work.push(Point::new(vertex.x, vertex.y, vertex.z));
                    }

                    if let Some(collider) = ColliderBuilder::convex_hull(&work) {
                        this.collider_set.insert(collider);
                    } else {
                        return Err(mlua::Error::RuntimeError(
                            "Failed to generate a convex hull".to_string(),
                        ));
                    }
                }

                Ok(())
            },
        );

        /* entry
        {
            "name": "rapier:create_controller",
            "info": "Create a kinematic character controller.",
            "result": [
                { "name": "index",      "info": "The handle to the collider.",   "kind": "table" },
                { "name": "controller", "info": "The handle to the controller.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("create_controller", |lua, this, ()| {
            let collider = ColliderBuilder::cuboid(1.0, 2.0, 1.0)
                .translation(vector![0.0, 10.0, 0.0])
                .build();
            let character_controller = KinematicCharacterController::default();

            let r_1 = lua.to_value(&this.collider_set.insert(collider))?;
            let r_2 = lua.to_value(&character_controller)?;

            Ok((r_1, r_2))
        });

        /* entry
        {
            "name": "rapier:move_controller",
            "info": "Move a kinematic character controller.",
            "member": [
                { "name": "index",      "info": "The handle to the collider.",   "kind": "table"    },
                { "name": "controller", "info": "The handle to the controller.", "kind": "table"    },
                { "name": "point",      "info": "The point to move to.",         "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "move_controller",
            |lua, this, (index, character, point): (LuaValue, LuaValue, LuaValue)| {
                let index: ColliderHandle = lua.from_value(index)?;
                let character: KinematicCharacterController = lua.from_value(character)?;
                let point: general::Vector3 = lua.from_value(point)?;
                let object = this.collider_set.get(index).unwrap();

                // Calculate the possible movement.
                let corrected_movement = character.move_shape(
                    1.0 / 60.0,
                    &this.rigid_body_set,
                    &this.collider_set,
                    &this.query_pipeline,
                    object.shape(),
                    object.position(),
                    vector![point.x, point.y, point.z],
                    QueryFilter::default().exclude_collider(index),
                    |_| {},
                );

                let mut object = this.collider_set.get_mut(index).unwrap();

                object.set_position(
                    vector![
                        object.position().translation.x + corrected_movement.translation.x,
                        object.position().translation.y + corrected_movement.translation.y,
                        object.position().translation.z + corrected_movement.translation.z,
                    ]
                    .into(),
                );

                lua.to_value(&general::Vector3::new(
                    corrected_movement.translation.x,
                    corrected_movement.translation.y,
                    corrected_movement.translation.z,
                ))
            },
        );
    }
}
