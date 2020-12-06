use tm_derive::Component;
use tm_rs::{
    add_or_remove_entity_simulation, api,
    component::{ComponentsIterator, Read, Write},
    components::{graph::GraphComponent, light::LightComponent},
    entity::{EntityApi, EntityApiInstanceMut},
    ffi::{tm_tt_id_t, tm_tt_id_t__bindgen_ty_1},
    graph_interpreter::GraphInterpreterApi,
    log::LogApi,
    the_truth::TheTruthApi,
    the_truth_assets::TheTruthAssetsApi,
    tm_plugin, Vec3,
};

#[derive(Copy, Clone, Default, Component)]
struct WallSpawnerComponent {
    has_run: bool,
}

fn engine_update(
    entity_api: &mut EntityApiInstanceMut,
    components: ComponentsIterator<(Write<WallSpawnerComponent>,)>,
) {
    let log = api::get::<LogApi>();
    let mut assets = entity_api.the_truth_assets();

    for (entity, wall_spawner) in components {
        if !wall_spawner.has_run {
            let mut root = entity;

            loop {
                log.info(&format!("Hi {:#?}", unsafe { root.u64_ }));

                let new_root = entity_api.parent(root);
                if unsafe { new_root.u64_ } == 0 {
                    break;
                } else {
                    root = new_root;
                }
            }

            let asset_root = entity_api.asset(entity);

            log.info(&format!("Hi {:#?}", unsafe {
                asset_root.__bindgen_anon_1.u64_
            }));

            let player = assets.asset_from_path(asset_root, "player.entity");

            log.info(&format!("Hi {:#?}", unsafe {
                player.__bindgen_anon_1.u64_
            }));
            wall_spawner.has_run = true;
        }
    }
}

fn register_light_engine(entity_api: &mut EntityApiInstanceMut) {
    entity_api.register_engine("Wall Spawner Engine", engine_update);
}

tm_plugin!(|reg: &mut RegistryApi| {
    api::register::<LogApi>(reg);
    api::register::<TheTruthAssetsApi>(reg);
    api::register::<GraphInterpreterApi>(reg);

    reg.add_or_remove_component::<WallSpawnerComponent>();

    add_or_remove_entity_simulation!(reg, register_light_engine);
});
