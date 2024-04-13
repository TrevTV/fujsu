# *fujsu*
A minimal APK embedded native mod loader for Unity IL2CPP on Android.

## Why?
QuestLoader and/or scotland2 (I don't know the difference) was too fancy for my needs, so I figured I'd make my own very basic loader.

## How?
Fujsu does very little by itself. The project has two sections, `main` and `fujsu`.

`main` is a replacement for the built-in Unity `libmain.so`. It allows the original game data to load, while also loading fujsu.

`fujsu` is the actual mod loader. Simply put, it reads a configuration embedded into the APK which defines which libraries (mods) to load. From there it loads the mods, calls the `load()` function, hooks `il2cpp_init`, then once that is called, it calls every mod's `il2cpp_ready()` function.

## Installation
Installation is meant to be mostly painless. Once compiled, put `libfujsu.so` and `libmain.so` inside the APK's ARM64 library folder (replacing the original `libmain.so`). Place any mods inside that directory as well. Then, create a `fujsu.toml` inside the `assets` folder. Inside of the config, you only need to define a list of mods.

```toml
mods = [
    "libmod1.so",
    "libmod2.so"
]
```

Then, recompile the APK and fujsu, and your mod, should automatically be loaded into the game.

## Mod Development
Fujsu is intended to be used with [cordl](https://github.com/QuestPackageManager/cordl/) and [my fork of beatsaber-hook](https://github.com/TrevTV/beatsaber-hook), though neither are required. If you don't want to use either, but want the other functions from `fujsu.h`, add `add_compile_definitions(NO_CORDL)` to `CMakeLists.txt`. 

`cordl` is the tool that generates C++ headers based on IL2CPP data and my fork of `beatsaber-hook` is a heavily stripped down version that only provides IL2CPP functions/utilities.

To create a mod, you can use the base `mod_template` project which defines the basics for a project to compile and run. I use the project with CLion with the following CMake options `-DCMAKE_TOOLCHAIN_FILE={NDK}/build/cmake/android.toolchain.cmake -DANDROID_ABI=arm64-v8a -DANDROID_NATIVE_API_LEVEL=23`.

Here is another example using cordl and the macros in `fujsu.h` to hook a function.
```cpp
#include "core.h"
#include "fujsu.h"

CREATE_HOOK(UIIncrementHook, &GlobalNamespace::UIController::IncrementPressed, void, GlobalNamespace::UIController* self) {
    self->count += 9; // counter adds 1 by default
    fujsu::info("Hello, world. - from UIIncrementHook macro; UIController Instance ID: %d", self->GetInstanceID());
    UIIncrementHook(self);
}

extern "C" void load() {
    fujsu::info("load()");
}

extern "C" void il2cpp_ready() {
    fujsu::info("il2cpp_ready()");

    il2cpp_functions::Init(); // required for beatsaber-hook

    INSTALL_HOOK(UIIncrementHook);
}
```

## Console
It can be viewed with `adb logcat`. If you want a filtered version, this following works.

`adb logcat -v time main:D fujsu:D Dobby:D Zygote:D DEBUG:D Unity:D Binder:D AndroidRuntime:D *:S`

## Thanks and Licenses
- [Millzy](https://github.com/MillzyDev/)
- [Someone Somewhere](https://github.com/s1sw)
- [zCubed](https://github.com/zCubed3/)
- [RinLovesYou](https://github.com/RinLovesYou/)
- A lot of the Beat Saber for Quest modders.
- Some code was based on (or directly taken from) [beatsaber-hook](https://github.com/QuestPackageManager/beatsaber-hook) under the MIT License. The full license is available [here](https://github.com/QuestPackageManager/beatsaber-hook/blob/master/LICENSE).