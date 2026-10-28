/*
DSOUND.C

DirectSound's objects as the game holds them. The game calls a stream's
media object methods through its vtable (IDirectSoundStream_Process and the
rest, port/include/xdk/xdk_dsound.h), and a vtable's entries are functions
of the module, so the streams are made here: each is the vtable's pointer
alone, and each entry is an import the runtime answers with the stream's
address (opence_runtime's dsound module keeps the stream's state, as
port/linux/src/dsound_sdl.c keeps it beside the object). The runtime
completes a packet by calling the game's callback through
halo_dsound_complete, since a function pointer is an index into the
module's table.
*/

#include "platform.h"

/* ---------- the runtime's */

extern long halo_dsound_stream_create(LPDIRECTSOUNDSTREAM stream, const struct _DSSTREAMDESC *description);
extern unsigned long halo_dsound_stream_add_ref(LPDIRECTSOUNDSTREAM stream);
extern unsigned long halo_dsound_stream_release(LPDIRECTSOUNDSTREAM stream);
extern long halo_dsound_stream_get_info(LPDIRECTSOUNDSTREAM stream, struct _XMEDIAINFO *information);
extern long halo_dsound_stream_get_status(LPDIRECTSOUNDSTREAM stream, unsigned long *status);
extern long halo_dsound_stream_process(LPDIRECTSOUNDSTREAM stream, const struct _XMEDIAPACKET *input,
	const struct _XMEDIAPACKET *output);
extern long halo_dsound_stream_discontinuity(LPDIRECTSOUNDSTREAM stream);
extern long halo_dsound_stream_flush(LPDIRECTSOUNDSTREAM stream);

/* ---------- streams */

static unsigned long __stdcall stream_release(LPDIRECTSOUNDSTREAM stream)
{
	unsigned long count = halo_dsound_stream_release(stream);

	if (!count)
		free(stream);
	return count;
}

static IDirectSoundStreamVtbl stream_vtable = {
	halo_dsound_stream_add_ref,
	stream_release,
	halo_dsound_stream_get_info,
	halo_dsound_stream_get_status,
	halo_dsound_stream_process,
	halo_dsound_stream_discontinuity,
	halo_dsound_stream_flush,
};

HRESULT WINAPI IDirectSound_CreateSoundStream(LPDIRECTSOUND sound, LPCDSSTREAMDESC description,
	LPDIRECTSOUNDSTREAM *result, LPUNKNOWN outer)
{
	LPDIRECTSOUNDSTREAM stream = calloc(1, sizeof(*stream));
	HRESULT created;

	(void)sound;
	(void)outer;
	if (!stream)
		return E_OUTOFMEMORY;
	stream->lpVtbl = &stream_vtable;
	created = halo_dsound_stream_create(stream, description);
	if (created < 0)
	{
		free(stream);
		return created;
	}
	*result = stream;
	return DS_OK;
}

/* ---------- the callback */

/* a packet's completion: the stream's callback with its context, the
packet's and the status */
void halo_dsound_complete(LPFNXMEDIAOBJECTCALLBACK callback, void *context, void *packet_context,
	unsigned long status)
{
	callback(context, packet_context, status);
}
