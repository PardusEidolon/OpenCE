/*
CRT.C

The Microsoft C runtime and D3DX helpers the game calls that are the C
library, not the platform: the printf family with MSVC's length modifiers,
the case-blind string compares, the floating-point control word, the x87
rounding conversion, the compiler barrier, stdio's fopen, winsock's fd_set
test and the D3DX maths. They are compiled into game.wasm
(tools/engine_wasm.py), so a call to one never leaves the module. Each follows the Linux port's
(port/linux/src/msvc_crt.c, d3d8_resources.c, xnet.c), with the structures
the public Win32 and D3DX documentation gives.

Compiled without the port's shims, so that vsnprintf here is wasi-libc's.
*/

#include <ctype.h>
#include <math.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ---------- printf with MSVC length modifiers */

/* rewrite %I64 as %ll and drop %I32; everything else is shared with musl */
static const char *translate_format(const char *format, char *buffer, size_t size)
{
	const char *cursor;
	size_t length = 0;

	if (!strstr(format, "I64") && !strstr(format, "I32"))
		return format;
	for (cursor = format; *cursor && length + 3 < size; cursor++)
	{
		buffer[length++] = *cursor;
		if (*cursor != '%')
			continue;
		if (cursor[1] == '%')
		{
			buffer[length++] = *++cursor;
			continue;
		}
		/* copy flags, width and precision */
		while (cursor[1] && strchr("-+ #0123456789.*", cursor[1]) && length + 3 < size)
			buffer[length++] = *++cursor;
		if (cursor[1] == 'I' && cursor[2] == '6' && cursor[3] == '4')
		{
			buffer[length++] = 'l';
			buffer[length++] = 'l';
			cursor += 3;
		}
		else if (cursor[1] == 'I' && cursor[2] == '3' && cursor[3] == '2')
		{
			cursor += 3;
		}
	}
	buffer[length] = '\0';
	return buffer;
}

int halo_linux_vsnprintf(char *buffer, size_t count, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vsnprintf(buffer, count, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_snprintf(char *buffer, size_t count, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vsnprintf(buffer, count, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vsprintf(char *buffer, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vsprintf(buffer, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_sprintf(char *buffer, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vsprintf(buffer, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vfprintf(FILE *stream, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vfprintf(stream, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_fprintf(FILE *stream, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vfprintf(stream, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vprintf(const char *format, va_list arguments)
{
	return halo_linux_vfprintf(stdout, format, arguments);
}

int halo_linux_printf(const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vfprintf(stdout, format, arguments);
	va_end(arguments);
	return result;
}

/* ---------- stdio files */

int _stricmp(const char *string1, const char *string2);

extern unsigned long halo_init_text(char *buffer, unsigned long size);

/* the game opens stdio files only for its debug logs (gamestate.txt, the
rasterizer's profile, the lruv caches' dumps) and optional overrides (the
automation files, bypass_security.txt, a map pack's loading picture), and
every caller goes on without the file; its own files go through the
platform's file imports. So there are none (the Linux port translates the
path and opens it, port/linux/src/msvc_crt.c), but for two: the error log,
d:\\debug.txt, which goes to the module's standard error with the other
log lines (errors.c keeps it open for the whole run and never closes it),
and the console's init script, d:\\init.txt, which is the text the
platform was given (halo_init_text), read from memory: console_startup
runs it as the game starts, and a map_name line starts it on that map */
FILE *halo_linux_fopen(const char *path, const char *mode)
{
	unsigned long length;
	FILE *file;

	(void)mode;
	if (!path)
		return NULL;
	if (_stricmp(path, "d:\\debug.txt") == 0)
		return stderr;
	if (_stricmp(path, "d:\\init.txt") != 0)
		return NULL;
	length = halo_init_text(NULL, 0);
	if (!length)
		return NULL;
	/* a buffer of fmemopen's own, freed when the file is closed */
	file = fmemopen(NULL, length, "w+");
	if (!file)
		return NULL;
	{
		char *text = malloc(length);

		if (!text)
		{
			fclose(file);
			return NULL;
		}
		halo_init_text(text, length);
		fwrite(text, 1, length, file);
		free(text);
	}
	rewind(file);
	return file;
}

/* ---------- strings */

int _stricmp(const char *string1, const char *string2)
{
	for (;; string1++, string2++)
	{
		int c1 = tolower((unsigned char)*string1);
		int c2 = tolower((unsigned char)*string2);

		if (c1 != c2 || !c1)
			return c1 - c2;
	}
}

int _strnicmp(const char *string1, const char *string2, size_t count)
{
	for (; count; count--, string1++, string2++)
	{
		int c1 = tolower((unsigned char)*string1);
		int c2 = tolower((unsigned char)*string2);

		if (c1 != c2 || !c1)
			return c1 - c2;
	}
	return 0;
}

/* ---------- floating point */

/* MSVC's default control word (float.h's _CW_DEFAULT): rounding to nearest,
53-bit precision, every exception masked */
#define CW_DEFAULT 0x0009001F
#define EM_DENORMAL 0x00080000

/* WebAssembly rounds to nearest and raises no exceptions, always. The MSVC
control word is only remembered, so that the game reads back what it set. */
static unsigned int msvc_control_word = CW_DEFAULT;

unsigned int _control87(unsigned int new_value, unsigned int mask)
{
	if (mask)
		msvc_control_word = (msvc_control_word & ~mask) | (new_value & mask);
	return msvc_control_word;
}

unsigned int _controlfp(unsigned int new_value, unsigned int mask)
{
	/* _controlfp ignores the denormal mask */
	return _control87(new_value, mask & ~EM_DENORMAL);
}

/* round to an integer as the x87 does in its default mode, like the inline
fast_ftol in cseries.h (fld/fistp) */
long fast_ftol_C(float value)
{
	return lrintf(value);
}

/* MSVC's compiler barrier. The game's threads share the module's memory, so
it is a full fence here: a compiler-only barrier would not order the
accesses another thread sees. */
void _ReadWriteBarrier(void)
{
	__atomic_thread_fence(__ATOMIC_SEQ_CST);
}

/* ---------- winsock */

/* winsock's fd_set (winsock.h): a count and an array of sockets, which are
32-bit handles on the Xbox */
#define WINSOCK_FD_SETSIZE 64

struct winsock_fd_set
{
	unsigned int fd_count;
	unsigned int fd_array[WINSOCK_FD_SETSIZE];
};

int __WSAFDIsSet(unsigned int socket, struct winsock_fd_set *set)
{
	unsigned int index;

	for (index = 0; index < set->fd_count && index < WINSOCK_FD_SETSIZE; index++)
	{
		if (set->fd_array[index] == socket)
			return 1;
	}
	return 0;
}

/* ---------- D3DX */

/* D3DXMATRIX: sixteen floats, row by row (_11 _12 _13 _14, _21 ...) */
struct d3dx_matrix
{
	float m[4][4];
};

/* D3DXVECTOR4 */
struct d3dx_vector4
{
	float x, y, z, w;
};

struct d3dx_matrix *D3DXMatrixPerspectiveLH(struct d3dx_matrix *out, float width, float height, float near_plane, float far_plane)
{
	memset(out, 0, sizeof(*out));
	out->m[0][0] = 2.0f * near_plane / width;
	out->m[1][1] = 2.0f * near_plane / height;
	out->m[2][2] = far_plane / (far_plane - near_plane);
	out->m[2][3] = 1.0f;
	out->m[3][2] = near_plane * far_plane / (near_plane - far_plane);
	return out;
}

struct d3dx_matrix *D3DXMatrixOrthoLH(struct d3dx_matrix *out, float width, float height, float near_plane, float far_plane)
{
	memset(out, 0, sizeof(*out));
	out->m[0][0] = 2.0f / width;
	out->m[1][1] = 2.0f / height;
	out->m[2][2] = 1.0f / (far_plane - near_plane);
	out->m[3][2] = near_plane / (near_plane - far_plane);
	out->m[3][3] = 1.0f;
	return out;
}

struct d3dx_vector4 *D3DXVec4Transform(struct d3dx_vector4 *out, const struct d3dx_vector4 *vector, const struct d3dx_matrix *matrix)
{
	const float (*m)[4] = matrix->m;
	struct d3dx_vector4 result;

	result.x = vector->x * m[0][0] + vector->y * m[1][0] + vector->z * m[2][0] + vector->w * m[3][0];
	result.y = vector->x * m[0][1] + vector->y * m[1][1] + vector->z * m[2][1] + vector->w * m[3][1];
	result.z = vector->x * m[0][2] + vector->y * m[1][2] + vector->z * m[2][2] + vector->w * m[3][2];
	result.w = vector->x * m[0][3] + vector->y * m[1][3] + vector->z * m[2][3] + vector->w * m[3][3];
	*out = result;
	return out;
}

/* the HRESULTs the Win32 documentation names */
#define HRESULT_S_OK 0x00000000L
#define HRESULT_E_OUTOFMEMORY ((long)0x8007000EL)
#define HRESULT_E_FAIL ((long)0x80004005L)

long D3DXGetErrorStringA(long error, char *buffer, unsigned int buffer_length)
{
	if (buffer && buffer_length)
	{
		const char *text = error == HRESULT_S_OK ? "S_OK" :
			error == HRESULT_E_OUTOFMEMORY ? "E_OUTOFMEMORY" :
			error == HRESULT_E_FAIL ? "E_FAIL" : "Direct3D error";

		strncpy(buffer, text, buffer_length - 1);
		buffer[buffer_length - 1] = '\0';
	}
	return HRESULT_S_OK;
}
