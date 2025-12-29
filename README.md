# bevy_splash_kit
> Startup intros, splash screens, and end credits for Bevy

**Note:** This crate is still under heavy development. Expect breaking changes in the api. However, it will always be up to date with the latest bevy version.

# Usage
This crate is a **time-driven screen sequence engine**. You bring your own rendering logic. We make scheduling easy. Eash splash screen is a sequence of `Fade In -> Hold -> Fade Out`.

Possible future api
```Rust
SplashSequence::new()
    .fade_in(Duration::from_secs(1))
    .hold(Duration::from_secs(2))
    .fade_out(Duration::from_secs(1))
    .then_enter(GameState::Menu);
```


Future api for chains
```Rust
SplashChainPlugin::new()
    .add_screen(Timeline { ... }, |commands| spawn_ui_1(commands))
    .add_screen(Timeline { ... }, |commands| spawn_ui_2(commands))
    .add_screen(Timeline { ... }, |commands| spawn_ui_3(commands))
    .then_enter(AppState::MainMenu)
```
