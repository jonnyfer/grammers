// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Contains definitions for the [`Client`] and [`ClientHandle`] structures, along with required
//! structures to initialize them such as [`Config`].

pub use super::updates::UpdateIter;
use grammers_mtproto::{mtp, transport};
use grammers_mtsender::{InvocationError, Sender};
use grammers_session::Session;
use tokio::sync::{mpsc, oneshot};

/// When no locale is found, use this one instead.
const DEFAULT_LOCALE: &str = "en";

/// Configuration required to create a [`Client`] instance.
///
/// [`Client`]: struct.Client.html
pub struct Config {
    /// Session storage where data should persist, such as authorization key, server address,
    /// and other required information by the client.
    pub session: Session,

    /// Developer's API ID, required to interact with the Telegram's API.
    ///
    /// You may obtain your own in <https://my.telegram.org/auth>.
    pub api_id: i32,

    /// Developer's API hash, required to interact with Telegram's API.
    ///
    /// You may obtain your own in <https://my.telegram.org/auth>.
    pub api_hash: String,

    /// Additional initialization parameters that can have sane defaults.
    pub params: InitParams,
}

/// Optional initialization parameters, required when initializing a connection to Telegram's
/// API.
pub struct InitParams {
    pub device_model: String,
    pub system_version: String,
    pub app_version: String,
    pub system_lang_code: String,
    pub lang_code: String,
}

pub(crate) struct Request {
    pub(crate) request: Vec<u8>,
    pub(crate) response: oneshot::Sender<oneshot::Receiver<Result<Vec<u8>, InvocationError>>>,
}

/// A client capable of connecting to Telegram and invoking requests.
pub struct Client {
    pub(crate) sender: Sender<transport::Full, mtp::Encrypted>,
    pub(crate) config: Config,
    pub(crate) handle_tx: mpsc::UnboundedSender<Request>,
    pub(crate) handle_rx: mpsc::UnboundedReceiver<Request>,
}

#[derive(Clone)]
pub struct ClientHandle {
    pub(crate) tx: mpsc::UnboundedSender<Request>,
}

impl Default for InitParams {
    fn default() -> Self {
        let info = os_info::get();

        let mut system_lang_code = locate_locale::system();
        if system_lang_code.is_empty() {
            system_lang_code.push_str(DEFAULT_LOCALE);
        }

        let mut lang_code = locate_locale::user();
        if lang_code.is_empty() {
            lang_code.push_str(DEFAULT_LOCALE);
        }

        Self {
            device_model: format!("{} {}", info.os_type(), info.bitness()),
            system_version: info.version().to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            system_lang_code,
            lang_code,
        }
    }
}
