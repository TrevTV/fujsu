#pragma once
#include <android/log.h>
#include <dlfcn.h>
#include <cstdint>

namespace fujsu {
    extern "C"
    {

    void *hook(void *target, void *detour);

    void unhook(void *target);

    }

    void info(const char *fmt, ...) {va_list args;
        va_start(args, fmt);
        __android_log_vprint(ANDROID_LOG_INFO, "fujsu", fmt, args);
        va_end(args);
    }

    void warning(const char *fmt, ...) {va_list args;
        va_start(args, fmt);
        __android_log_vprint(ANDROID_LOG_WARN, "fujsu", fmt, args);
        va_end(args);
    }

    void error(const char *fmt, ...) {va_list args;
        va_start(args, fmt);
        __android_log_vprint(ANDROID_LOG_ERROR, "fujsu", fmt, args);
        va_end(args);
    }
}

#ifndef NO_CORDL
#define CREATE_HOOK(name_, mPtr, retval, ...)                                                                              \
    struct Hook_##name_                                                                                                    \
    {                                                                                                                      \
        using funcType = retval (*)(__VA_ARGS__);                                                                          \
        constexpr static const char *name() { return #name_; }                                                             \
        static const MethodInfo *getInfo() { return il2cpp_utils::il2cpp_type_check::MetadataGetter<mPtr>::methodInfo(); } \
        static funcType *trampoline() { return &name_; }                                                                   \
        static inline retval (*name_)(__VA_ARGS__) = nullptr;                                                              \
        static funcType hook() { return &hook_##name_; }                                                                   \
        static retval hook_##name_(__VA_ARGS__);                                                                           \
    };                                                                                                                     \
    retval Hook_##name_::hook_##name_(__VA_ARGS__)

#define INSTALL_HOOK(name) InstallHook<Hook_##name>();

template<typename T>
void InstallHook() {
    auto info = T::getInfo();
    auto addr = (void *) info->methodPointer;
    void *trampoline = fujsu::hook(addr, (void *) T::hook());
    (*(void **) T::trampoline()) = trampoline;
}
#else
#define CREATE_HOOK(name_, retval, ...)                                                                              \
    struct Hook_##name_                                                                                                    \
    {                                                                                                                      \
        using funcType = retval (*)(__VA_ARGS__);                                                                          \
        constexpr static const char *name() { return #name_; }                                                             \
        static funcType *trampoline() { return &name_; }                                                                   \
        static inline retval (*name_)(__VA_ARGS__) = nullptr;                                                              \
        static funcType hook() { return &hook_##name_; }                                                                   \
        static retval hook_##name_(__VA_ARGS__);                                                                           \
    };                                                                                                                     \
    retval Hook_##name_::hook_##name_(__VA_ARGS__)

#define INSTALL_HOOK_WITH_ADDR(name, addr) InstallHookWithAddr<Hook_##name>((void*)addr);

template<typename T>
void InstallHookWithAddr(void* addr) {
    // TODO: this is awful
    void* mod = dlopen("libil2cpp.so", RTLD_LAZY);
    Dl_info info;
    void* initPtr = dlsym(mod, "il2cpp_init");
    dladdr(initPtr, &info);
    char* addrChar = reinterpret_cast<char*>(info.dli_fbase);
    dlclose(mod);

    void *trampoline = fujsu::hook((void*)(addrChar + reinterpret_cast<std::uintptr_t>(addr)), (void *) T::hook());
    (*(void **) T::trampoline()) = trampoline;
}
#endif
