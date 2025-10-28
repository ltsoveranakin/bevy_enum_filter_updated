#![doc = include_str!("../README.md")]

mod extensions;
mod filter_trait;
mod systems;

pub use bevy_enum_filter_derive::{Enum, EnumFilter};
pub use filter_trait::EnumFilter;
pub use systems::watch_for_enum;

pub mod prelude {
    pub use super::extensions::AddEnumFilter;
    pub use super::filter_trait::EnumFilter;
    pub use bevy_enum_filter_derive::{Enum, EnumFilter};
}

// #[cfg(test)]
// mod tests {
//     use super::prelude::*;
//     use bevy::prelude::*;
//
//     #[derive(Component, EnumFilter)]
//     enum TestEnum {
//         Unit,
//         Tuple(usize),
//         Struct { _foo: usize },
//     }
//
//     #[test]
//     fn should_use_enum_filter() {
//         #[derive(Message)]
//         struct FoundFilter(bool);
//
//         let mut app = App::new();
//         app.add_enum_filter::<TestEnum>();
//         app.add_message::<FoundFilter>();
//         app.add_systems(Update, remove_entity_with_enum);
//
//         let entity = app.world_mut().spawn_empty().id();
//
//         app.update();
//
//         assert!(app.world_mut().get_entity(entity).is_ok());
//
//         app.world_mut()
//             .entity_mut(entity)
//             .insert(TestEnum::Struct { _foo: 123 });
//
//         app.update();
//         app.update();
//
//         assert!(app.world_mut().get_entity(entity).is_err());
//
//         fn remove_entity_with_enum(
//             query: Query<Entity, With<Enum!(TestEnum::Struct)>>,
//             mut commands: Commands,
//         ) {
//             for entity in &query {
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
//
//     #[test]
//     fn should_automatically_handle_markers() {
//         let mut app = App::new();
//         app.add_enum_filter::<TestEnum>();
//         let entity = app.world_mut().spawn_empty().id();
//
//         app.update();
//
//         let total_components = app.world().entity(entity).archetype().components().len();
//         assert_eq!(0, total_components);
//         assert!(!app
//             .world()
//             .entity(entity)
//             .contains::<test_enum_filters::Tuple>());
//
//         app.world_mut().entity_mut(entity).insert(TestEnum::Tuple(123));
//
//         app.update();
//
//         let total_components = app.world().entity(entity).archetype().components().len();
//         assert_eq!(2, total_components);
//         assert!(app
//             .world()
//             .entity(entity)
//             .contains::<test_enum_filters::Tuple>());
//
//         app.world_mut().entity_mut(entity).insert(TestEnum::Unit);
//
//         app.update();
//
//         let total_components = app.world().entity(entity).archetype().components().len();
//         assert_eq!(2, total_components);
//         assert!(!app
//             .world()
//             .entity(entity)
//             .contains::<test_enum_filters::Tuple>());
//         assert!(app
//             .world()
//             .entity(entity)
//             .contains::<test_enum_filters::Unit>());
//     }
// }
