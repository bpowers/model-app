// Copyright 2020 The Model Authors. All rights reserved.
// Use of this source code is governed by the Apache License,
// Version 2.0, that can be found in the LICENSE file.

use wasm_bindgen::prelude::*;

use std::io::BufReader;

use system_dynamics_compat::{engine, open_xmile, prost};

#[wasm_bindgen]
pub fn from_xmile(xmile_xml: &str) -> Box<[u8]> {
    use prost::Message;
    let project = match open_xmile(&mut BufReader::new(xmile_xml.as_bytes())) {
        Ok(project) => project,
        Err(err) => panic!("open_xmile failed: {}", err),
    };
    let project_pb = engine::serde::serialize(&project);

    let mut buf: Vec<u8> = Vec::with_capacity(project_pb.encoded_len() + 8);
    match project_pb.encode(&mut buf) {
        Ok(_) => {}
        Err(err) => panic!("encode failed: {}", err),
    };

    buf.into_boxed_slice()
}

// #[wasm_bindgen]
// pub fn from_vensim(xmile_xml: &str) -> Box<[u8]> {
//     use system_dynamics_compat::open_xmile;
//     use prost::Message;
//     let project = open_vensim(&mut BufReader::new(xmile_xml.as_bytes())).unwrap();
//     let project_pb = engine::serde::serialize(&project);
//
//     let mut buf: Vec<u8> = Vec::with_capacity(project_pb.encoded_len() + 8);
//     project_pb
//         .encode(&mut buf)
//         .unwrap();
//
//     buf.into_boxed_slice()
// }