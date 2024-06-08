/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod connection;
mod context;
mod response;
mod stats;
mod task;

pub(crate) use connection::{
    send_req_header_to_origin, send_req_header_via_proxy, BoxHttpForwardConnection,
    BoxHttpForwardReader, BoxHttpForwardWriter, HttpConnectionEofPoller, HttpForwardRead,
    HttpForwardWrite, HttpForwardWriterForAdaptation,
};
pub(crate) use context::{
    BoxHttpForwardContext, DirectHttpForwardContext, HttpForwardContext, ProxyHttpForwardContext,
    RouteHttpForwardContext,
};
pub(crate) use response::HttpProxyClientResponse;
pub(crate) use stats::{
    ArcHttpForwardTaskRemoteStats, HttpForwardRemoteStatsWrapper, HttpForwardTaskRemoteStats,
};
pub(crate) use task::HttpForwardTaskNotes;
