// Copyright 2017 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Implementation of the `/ipfs/id/1.0.0` protocol. Allows a node A to query another node B which
//! information B knows about A. Also includes the addresses B is listening on.
//!
//! When two nodes connect to each other, the listening half sends a message to the dialing half,
//! indicating the information, and then the protocol stops.
//!
//! # Usage
//!
//! Both low-level and high-level usages are available.
//!
//! ## High-level usage through the `PeerIdTransport` struct
//!
//! This crate provides the `PeerIdTransport` struct, which wraps around a `Transport` and an
//! address resolver. `PeerIdTransport` is itself a transport that accepts multiaddresses of the
//! form `/p2p/...` (or of the deprecated `/ipfs/...` form).
//!
//! If you dial a multiaddr of the form `/p2p/...`, then the `PeerIdTransport` will query the
//! address resolver for any known multiaddress for this peer and try to dial them using the
//! underlying transport. If you dial any other multiaddr, then it will dial this multiaddr using
//! the underlying transport. In both situations, it will then negotiate the *identify* protocol
//! with the remote in order to obtain its ID and information about it.
//!
//! Listening doesn't support multiaddresses of the form `/p2p/...` (because that wouldn't make
//! sense). Any address passed to `listen_on` will be passed directly to the underlying transport
//! and incoming connections will be identified.
//!
//! Whenever a remote connects to us, either through listening or through `next_incoming`, the
//! `PeerIdTransport` dials back the remote, upgrades the connection to the *identify* protocol
//! in order to obtain its information, and then only returns the incoming connection. From the
//! exterior, the multiaddress of the remote is of the form `/p2p/...`. If the remote doesn't
//! support the *identify* protocol, then the socket is closed.
//!
//! Because of the behaviour of `PeerIdTransport`, it is recommended to build it on top of a
//! `ConnectionReuse`.
//!
//! ## Low-level usage through the `IdentifyProtocolConfig` struct
//!
//! The `IdentifyProtocolConfig` struct implements the `ConnectionUpgrade` trait. Using it will
//! negotiate the *identify* protocol.
//!
//! The output of the upgrade is a `IdentifyOutput`. If we are the dialer, then `IdentifyOutput`
//! will contain the information sent by the remote. If we are the listener, then it will contain
//! a `IdentifySender` struct that can be used to transmit back to the remote the information about
//! it.

extern crate bytes;
extern crate fnv;
extern crate futures;
extern crate futures_mutex;
extern crate libp2p_peerstore;
extern crate libp2p_core;
#[macro_use]
extern crate log;
extern crate multiaddr;
extern crate parking_lot;
extern crate protobuf;
extern crate tokio_io;
extern crate varint;

pub use self::peer_id_transport::{PeerIdTransport, PeerIdTransportOutput};
pub use self::protocol::{IdentifyInfo, IdentifyOutput, IdentifyProtocolConfig, IdentifySender};

mod identify_transport;
mod peer_id_transport;
mod protocol;
mod structs_proto;
