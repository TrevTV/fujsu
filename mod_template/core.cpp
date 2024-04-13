#include "core.h"
#include <android/log.h>

extern "C" void load() {
    __android_log_print(ANDROID_LOG_INFO, "fujsu", "Hello, world. - from load()");
}

extern "C" void il2cpp_ready() {
    __android_log_print(ANDROID_LOG_INFO, "fujsu", "Hello, world. - from il2cpp_ready()");
}