use bevy::prelude::*;

use crate::{game::game_res::GameTime, gamepad::gamepad_rcs::MyGamepad, AppState};

use super::game_over_cmps::*;
use super::*;

pub fn spawn_menu(mut cmds: Commands, assets: Res<AssetServer>, game_time: Res<GameTime>) {
    cmds.spawn((Camera3dBundle::default(), GameOverMenu));

    let container = (
        NodeBundle {
            background_color: GAME_OVER_MENU_COLOR.into(),
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        },
        GameOverMenu,
        Name::new("Game Over Menu"),
    );

    let time = game_time.0.elapsed().as_secs_f32();
    let time_survived_txt = (
        TextBundle::from_section(
            format!("Time Survived: {:.2}", time),
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
        ),
        Name::new("Time Survived Text"),
    );

    let play_again_btn = (
        ButtonBundle {
            background_color: PLAY_AGAIN_BTN_COLOR.into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(230.0), Val::Px(75.0)),
                margin: UiRect::top(Val::Px(50.0)),
                ..default()
            },
            ..default()
        },
        PlayAgainBtn,
        Name::new("Play Again Button"),
    );

    let play_again_txt = (
        TextBundle::from_section(
            "Play Again - Y",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
        ),
        Name::new("Play Again Text"),
    );

    // game over menu
    cmds.spawn(container).with_children(|parent| {
        // time survived txt
        parent.spawn(time_survived_txt);

        // play again btn
        parent.spawn(play_again_btn).with_children(|parent| {
            // play again txt
            parent.spawn(play_again_txt);
        });
    });
}

pub fn select_play_again_gamepad(
    btns: Res<Input<GamepadButton>>,
    cur_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::North)))
        .unwrap_or(false);

    if gamepad_input {
        if cur_app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn select_play_again_mouse(
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayAgainBtn>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Clicked => next_app_state.set(AppState::Game),
            Interaction::Hovered => *background_clr = PLAY_AGAIN_BTN_COLOR_HOVER.into(),
            Interaction::None => *background_clr = PLAY_AGAIN_BTN_COLOR.into(),
        }
    }
}

pub fn despawn_game_over_menu(mut cmds: Commands, game_over_q: Query<Entity, With<GameOverMenu>>) {
    for ent in game_over_q.iter() {
        cmds.entity(ent).despawn_recursive();
    }
}