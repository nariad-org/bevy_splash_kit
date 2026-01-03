# bevy_splash_kit
> Startup intros, splash screens, and end credits for Bevy

**Note:** This crate is still under heavy development. Expect breaking changes in the api. However, it will always be up to date with the latest bevy version.

# Usage
This crate is a **time-driven screen sequence engine**. You bring your own rendering logic. We make scheduling easy. Eash splash screen is a sequence of `Fade In -> Hold -> Fade Out`.

# Roadmap

### Implement a new builder api
```Rust
SplashChainPlugin::new()
    // simple closure, easy for beginners
    .add_screen(
        Timeline { fade_in: 1s, hold: 2s, fade_out: 1s },
        |commands| spawn_ui(commands),
        |t, commands| fade_in_ui(t, commands),
        |t, commands| fade_out_ui(t, commands),
    )
    // advanced closure, can access assets/audio
    .add_screen_advanced(
        Timeline { fade_in: 1s, hold: 2s, fade_out: 1s },
        |mut commands, asset_server: Res<AssetServer>, audio: Res<Audio>| {
            let logo = asset_server.load("logo.png");
            commands.spawn(SpriteBundle { texture: logo, ..default() });
            audio.play(handle.clone());
        },
        |t, mut commands, _asset_server, _audio| {
            // fade logic with access to resources
        },
        |t, mut commands, _asset_server, _audio| {
            // fade out logic with access to resources
        },
    )
    .then_enter(AppState::MainMenu);
```
