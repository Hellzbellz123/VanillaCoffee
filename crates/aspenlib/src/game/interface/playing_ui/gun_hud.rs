use bevy::prelude::*;

use crate::game::{
    characters::{components::WeaponSlot, player::PlayerSelectedHero},
    items::weapons::components::{CurrentlyDrawnWeapon, WeaponAmmoCount, WeaponCarrier},
};

/// creates player weapon information display
pub fn create_gun_hud(playing_ui_parts: &mut ChildBuilder) {
    playing_ui_parts
        .spawn((
            Name::new("GunHud"),
            GunHudContainer,
            Outline {
                width: Val::Px(3.0),
                offset: Val::default(),
                color: super::colors::OUTLINE,
            },
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexStart,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(300.0),
                height: Val::Px(120.0),
                ..default()
            },
            BackgroundColor(super::colors::BACKDARK),
        ))
        .with_children(|gun_hud_parts| {
            create_ammo_bar(gun_hud_parts);
            create_gun_slots(gun_hud_parts);
        });
}

/// spawns gun slots widget
fn create_gun_slots(gun_hud_parts: &mut ChildBuilder) {
    gun_hud_parts
        .spawn((
            Name::new("GunSlotsContainer"),
            BackgroundColor(super::colors::BACKLIGHT),
            Node {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                height: Val::Percent(70.0),
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|gun_slot_parts| {
            create_gun_slot(gun_slot_parts, WeaponSlot::Slot1, 50.0);
            create_gun_slot(gun_slot_parts, WeaponSlot::Slot2, 50.0);
            create_gun_slot(gun_slot_parts, WeaponSlot::Slot3, 50.0);
            create_gun_slot(gun_slot_parts, WeaponSlot::Slot4, 50.0);
        });
}

/// spawns ammo bar widget
fn create_ammo_bar(gun_hud_parts: &mut ChildBuilder) {
    // TODO: make this unique widget with splits per ammo count
    gun_hud_parts
        .spawn((
            Name::new("AmmoBarContainer"),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                ..default()
            },
        ))
        .with_children(|ammo_count_parts| {
            ammo_count_parts.spawn((
                Name::new("AmmoCountText"),
                Text::new("Ammo Count"),
                TextFont::from_font_size(12.0),
            ));
            ammo_count_parts
                .spawn((
                    Name::new("AmmoBarBackGround"),
                    Node {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            top: Val::Auto,
                            bottom: Val::Auto,
                            ..default()
                        },
                        width: Val::Percent(95.0),
                        height: Val::Percent(40.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    BackgroundColor(super::colors::BACKLIGHT),
                ))
                .with_children(|bar_parts| {
                    bar_parts.spawn((
                        Name::new("AmmoBar"),
                        PlayerAmmoBar {
                            current: 0.0,
                            max: 0.0,
                        },
                        Node {
                            height: Val::Percent(75.0),
                            width: Val::Percent(0.0),
                            ..default()
                        },
                        BackgroundColor(super::colors::UTILITYEMPTY),
                    ));
                });
        });
}

/// create gun slot widget
fn create_gun_slot(gun_slot_parts: &mut ChildBuilder, slot: WeaponSlot, size: f32) {
    gun_slot_parts.spawn((
        Name::new("GunSlot"),
        UiWeaponSlot(slot),
        Outline {
            width: Val::Px(2.0),
            offset: Val::Px(5.0),
            color: super::colors::ACCENT,
        },
        ImageNode::default(),
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct GunHudContainer;

/// ui ammo bar resource data
#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerAmmoBar {
    /// current value for bar
    current: f32,
    /// max value for bar
    max: f32,
}

/// ui widget tag for weapon slots
#[derive(Debug, Component)]
pub struct UiWeaponSlot(WeaponSlot);

pub fn gunhud_visibility_system(
    player_query: Query<&WeaponCarrier, With<PlayerSelectedHero>>,
    mut gunhud_query: Query<&mut Node, With<GunHudContainer>>,
) {
    let Ok(player_slots) = player_query.get_single() else {
        return;
    };
    let Ok(mut gunhud_style) = gunhud_query.get_single_mut() else {
        return;
    };

    let all_slots_empty = player_slots
        .weapon_slots
        .values()
        .all(std::option::Option::is_none);

    if all_slots_empty || player_slots.drawn_slot.is_none() {
        gunhud_style.display = Display::None;
    } else {
        gunhud_style.display = Display::Flex;
    }
}

/// update ui ammo slot with equipped weapon
pub fn update_ui_ammo_slots(
    player_query: Query<&WeaponCarrier, With<PlayerSelectedHero>>,
    weapon_query: Query<&Sprite, With<CurrentlyDrawnWeapon>>,
    mut ui_weapon_slot: Query<
        (&UiWeaponSlot, &mut ImageNode, &mut Outline),
        Without<CurrentlyDrawnWeapon>,
    >,
) {
    let Ok(player_slots) = player_query.get_single() else {
        return;
    };

    for (slot_id, mut slot_image, mut outline) in &mut ui_weapon_slot {
        if player_slots.drawn_slot.is_some_and(|f| f == slot_id.0) {
            outline.color = super::colors::HIGHLIGHT;
        } else {
            outline.color = super::colors::ACCENT;
        }

        let Some(weapon_in_slot) = player_slots.weapon_slots.get(&slot_id.0).unwrap() else {
            continue;
        };

        if let Ok(weapon_image) = weapon_query.get(*weapon_in_slot) {
            if slot_image.image != weapon_image.image {
                slot_image.image = weapon_image.image.clone();
                slot_image.texture_atlas = weapon_image.texture_atlas.clone();
            }
        }
    }
}

/// updates ui ammo counter value with current ammo amount
pub fn update_ui_ammo_counter(
    mut ammo_bar_query: Query<(&mut Node, &mut PlayerAmmoBar)>,
    player_query: Query<&WeaponCarrier, With<PlayerSelectedHero>>,
    weapon_query: Query<&WeaponAmmoCount, With<CurrentlyDrawnWeapon>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    let ammo_counts: Option<&WeaponAmmoCount> = {
        let Some(slot) = player.drawn_slot else {
            return;
        };
        let Some(drawn_weapon) = player.weapon_slots.get(&slot) else {
            warn!("could not get drawn weapon entity");
            return;
        };

        if let Some(weapon) = drawn_weapon {
            let Ok(ammo_count) = weapon_query.get(*weapon) else {
                return;
            };
            Some(ammo_count)
        } else {
            None
        }
    };

    let (mut style, mut bar_data) = ammo_bar_query.single_mut();
    if ammo_counts.is_some_and(|weapon_count| weapon_count.current == bar_data.current as u32) {
        return;
    };

    let percentage = if let Some(ammo_count) = ammo_counts {
        bar_data.current = ammo_count.current as f32;
        bar_data.max = ammo_count.max as f32;
        (ammo_count.current as f32 / ammo_count.max as f32) * 100.0
    } else {
        100.0
    };

    if style.width != Val::Percent(percentage) {
        // info!("setting bar width too {}%", percentage);
        style.width = Val::Percent(percentage);
    }
}
