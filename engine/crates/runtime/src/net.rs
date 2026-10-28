//! The network the game starts on its way to its main menu: Winsock and
//! XNet (`port/include/xdk/xdk_winsock.h`). The app has no system link
//! (the plan has no network task), so the platform answers as an Xbox
//! with its cable out: the network starts, the title has no address
//! (`XNET_GET_XNADDR_NONE`), the link is down and no socket can be made.
//! The transport (`source/bungie_net/network/transport_endpoint_set_winsock.c`)
//! then gives up at once as not initialized, where a pending address would
//! keep it polling for ten seconds. Keys and random bytes are made as the
//! Linux port makes them (`port/linux/src/xnet.c`).

use std::hash::{BuildHasher, RandomState};

use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::{with_space, Record};

/// XNET_GET_XNADDR_NONE: the title has no address
const NO_ADDRESS: u32 = 1;
/// the size of XNADDR (`port/include/xdk/xdk_pdb.h`): its size, flags,
/// abEnet[6] and ina
const XNADDR: usize = 12;
/// and where its ina is
const XNADDR_INA: u32 = 8;
/// the size of WSADATA: two versions, szDescription[257],
/// szSystemStatus[129], two limits and lpVendorInfo, 4-aligned
const WSADATA: usize = 400;
/// where WSADATA's iMaxSockets is
const WSADATA_LIMITS: usize = 390;
/// the Linux port's limits: 64 sockets and 1264-byte datagrams
const MAX_SOCKETS: u16 = 64;
const MAX_DATAGRAM: u16 = 1264;
/// MAKEWORD(2, 2), the highest version
const HIGH_VERSION: u16 = 0x0202;
/// INVALID_SOCKET
const INVALID_SOCKET: u32 = u32::MAX;
/// WSAENETDOWN: every socket call's error, the network being down
const NETWORK_DOWN: u32 = 10050;
/// the sizes of XNKID and XNKEY
const KEY_ID: usize = 8;
const KEY: usize = 16;

/// The handler for one of the network's imports, if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "WSAStartup" => startup,
        "WSAGetLastError" => last_error,
        "WSACleanup" | "XNetStartup" | "XNetCleanup" | "XNetRegisterKey" | "XNetUnregisterKey" => {
            ok
        }
        "XNetGetTitleXnAddr" => title_address,
        "XNetGetEthernetLinkStatus" => link_status,
        "XNetRandom" => random,
        "XNetCreateKey" => create_key,
        "XNetXnAddrToInAddr" => to_in_address,
        "halo_ws_socket" => socket,
        _ => return None,
    };
    Some(handler)
}

/// the calls that succeed with nothing to do
fn ok(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `WSAStartup(version, data)`: the version asked for, 2.2 at most, and
/// the limits; a null `data` is not written
fn startup(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (version, data) = (arg(params, 0), arg(params, 1));
    if data != 0 {
        let mut written = [0; WSADATA];
        let version = u16::try_from(version & 0xFFFF).unwrap_or(HIGH_VERSION);
        written[0..2].copy_from_slice(&version.to_le_bytes());
        written[2..4].copy_from_slice(&HIGH_VERSION.to_le_bytes());
        written[WSADATA_LIMITS..WSADATA_LIMITS + 2].copy_from_slice(&MAX_SOCKETS.to_le_bytes());
        written[WSADATA_LIMITS + 2..WSADATA_LIMITS + 4]
            .copy_from_slice(&MAX_DATAGRAM.to_le_bytes());
        with_space(caller, |space, _| space.write(data, &written))?;
    }
    give(results, 0);
    Ok(())
}

/// `WSAGetLastError()`: the network is down
fn last_error(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, NETWORK_DOWN);
    Ok(())
}

/// `XNetGetTitleXnAddr(address)`: cleared but for its size, and none
fn title_address(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    if address != 0 {
        let mut cleared = [0; XNADDR];
        cleared[0] = u8::try_from(XNADDR).unwrap_or(u8::MAX);
        with_space(caller, |space, _| space.write(address, &cleared))?;
    }
    give(results, NO_ADDRESS);
    Ok(())
}

/// `XNetGetEthernetLinkStatus()`: no link
fn link_status(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `size` random bytes, from the standard library's randomly keyed hasher
fn random_bytes(size: usize) -> Vec<u8> {
    let state = RandomState::new();
    (0..size.div_ceil(8))
        .flat_map(|block| state.hash_one(block).to_le_bytes())
        .take(size)
        .collect()
}

/// `XNetRandom(buffer, size)`
fn random(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (buffer, size) = (arg(params, 0), arg(params, 1));
    let bytes = random_bytes(usize::try_from(size).unwrap_or(0));
    if buffer != 0 {
        with_space(caller, |space, _| space.write(buffer, &bytes))?;
    }
    give(results, 0);
    Ok(())
}

/// `XNetCreateKey(identifier, key)`: both random
fn create_key(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (identifier, key) = (arg(params, 0), arg(params, 1));
    let bytes = random_bytes(KEY_ID + KEY);
    with_space(caller, |space, _| {
        if identifier != 0 {
            space.write(identifier, bytes.get(..KEY_ID).unwrap_or_default());
        }
        if key != 0 {
            space.write(key, bytes.get(KEY_ID..).unwrap_or_default());
        }
    })?;
    give(results, 0);
    Ok(())
}

/// `XNetXnAddrToInAddr(address, identifier, result)`: the address's own
/// ina, as the Linux port gives it for a system link peer
fn to_in_address(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, result) = (arg(params, 0), arg(params, 2));
    if address != 0 && result != 0 {
        with_space(caller, |space, _| {
            let ina = space
                .read(address.wrapping_add(XNADDR_INA), 4)
                .unwrap_or_default();
            space.write(result, &ina);
        })?;
    }
    give(results, 0);
    Ok(())
}

/// `socket(family, type, protocol)`: none, the network being down
fn socket(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, INVALID_SOCKET);
    Ok(())
}
