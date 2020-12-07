use tm_derive::Component;
use tm_rs::{
    add_or_remove_entity_simulation,
    api::{
        self,
        application::ApplicationApi,
        entity::{EntityApi, EntityApiInstanceMut},
        graph_interpreter::GraphInterpreterApi,
        log::LogApi,
        the_truth::TheTruthApi,
        the_truth_assets::TheTruthAssetsApi,
    },
    component::{ComponentsIterator, Write},
    tm_plugin,
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
    let asset_root = api::get::<ApplicationApi>().application().asset_root();
    let assets = entity_api.the_truth_assets();

    for (entity, wall_spawner) in components {
        if !wall_spawner.has_run {
            let player = assets.asset_from_path(asset_root, "player.entity");

            log.info(&format!("Hi {:?}", player));

            //entity_api.

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
    api::register::<ApplicationApi>(reg);

    reg.add_or_remove_component::<WallSpawnerComponent>();

    add_or_remove_entity_simulation!(reg, register_light_engine);
});
