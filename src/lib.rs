use tm_derive::Component;
use tm_rs::{
    add_or_remove_entity_simulation,
    api::{
        self, application::ApplicationApi, entity::EntityApiInstanceMut,
        graph_interpreter::GraphInterpreterApi, the_truth_assets::TheTruthAssetsApi,
    },
    component::{ComponentsIterator, Write},
    log::info,
    tm_plugin,
};

#[derive(Copy, Clone, Default, Component)]
struct WallSpawnerComponent {
    has_run: bool,

    #[property(default = 0.0)]
    distance: f32,
}

fn engine_update(
    entity_api: &mut EntityApiInstanceMut,
    components: ComponentsIterator<(Write<WallSpawnerComponent>,)>,
) {
    let asset_root = api::get::<ApplicationApi>().application().asset_root();
    let the_truth = entity_api.the_truth();
    let assets = entity_api.the_truth_assets();

    for (entity, wall_spawner) in components {
        if !wall_spawner.has_run {
            let player_asset_id = assets.asset_from_path(asset_root, "player.entity");

            info!("Hi {:?}", player_asset_id);

            if let Some(type_name) = the_truth.type_name(player_asset_id) {
                info!("{}", &type_name);
            }

            let player_id = the_truth.read(player_asset_id).unwrap().get_subobject(3);

            if let Some(type_name) = the_truth.type_name(player_id) {
                info!("{}", &type_name);
            }
            let wall_entity = entity_api.create_entity_from_asset(player_id);

            wall_spawner.has_run = true;
        }
    }
}

fn register_light_engine(entity_api: &mut EntityApiInstanceMut) {
    entity_api.register_engine("Wall Spawner Engine", engine_update);
}

tm_plugin!(|reg: &mut RegistryApi| {
    api::register::<TheTruthAssetsApi>(reg);
    api::register::<GraphInterpreterApi>(reg);
    api::register::<ApplicationApi>(reg);

    reg.add_or_remove_component::<WallSpawnerComponent>();

    add_or_remove_entity_simulation!(reg, register_light_engine);
});
