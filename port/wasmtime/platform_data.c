/*
PLATFORM_DATA.C

The variables of the Xbox's libraries that the game reads and writes
directly, not through a call: Direct3D's render and texture states, which
the SDK's inline functions keep (port/include/xdk/xdk_d3d8.h), and the
input library's device types, whose addresses name a kind of device. A
variable can't be imported, so these live in game.wasm; the runtime finds
the states by their exported addresses. The Linux port defines the same
ones (port/linux/src/d3d8_gl.c, xinput_sdl.c), and platform.h includes the
SDK's headers as its platform layer does.
*/

#include "platform.h"

/* ---------- Direct3D */

DWORD D3D__RenderState[D3DRS_MAX];
DWORD D3D__TextureState[D3DTSS_MAXSTAGES][D3DTSS_MAX];
WORD *D3D__IndexData;

/* ---------- input */

/* only their addresses are used: XGetDevices(XDEVICE_TYPE_GAMEPAD) and the
rest */
XPP_DEVICE_TYPE XDEVICE_TYPE_GAMEPAD_TABLE;
XPP_DEVICE_TYPE XDEVICE_TYPE_MEMORY_UNIT_TABLE;
XPP_DEVICE_TYPE XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE;
