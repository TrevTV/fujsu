#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void *hook(void *target, void *detour);

void unhook(void *target);

} // extern "C"
