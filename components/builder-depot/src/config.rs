// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::net;

use hab_core::config::{ConfigFile, ParseInto};
use hab_net::config::{GitHubOAuth, RouteAddrs};
use redis;
use toml;

use error::{Error, Result};

/// URL to GitHub API endpoint
const GITHUB_URL: &'static str = "https://api.github.com";
// Default Client ID for providing a default value in development environments only. This is
// associated to Jamie Winsor's GitHub account and is configured to re-direct and point to a local
// builder-api.
const DEV_GITHUB_CLIENT_ID: &'static str = "0c2f738a7d0bd300de10";
// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
// additional comments.
const DEV_GITHUB_CLIENT_SECRET: &'static str = "438223113eeb6e7edf2d2f91a232b72de72b9bdf";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub path: String,
    pub listen_addr: net::SocketAddrV4,
    pub datastore_addr: net::SocketAddrV4,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<net::SocketAddrV4>,
    /// URL to GitHub API
    pub github_url: String,
    /// Client identifier used for GitHub API requests
    pub github_client_id: String,
    /// Client secret used for GitHub API requests
    pub github_client_secret: String,
    /// allows you to upload packages and public keys without auth
    pub insecure: bool,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.path", &mut cfg.path));
        try!(toml.parse_into("cfg.bind_addr", &mut cfg.listen_addr));
        try!(toml.parse_into("cfg.datastore_addr", &mut cfg.datastore_addr));
        try!(toml.parse_into("cfg.router_addrs", &mut cfg.routers));
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "/hab/svc/hab-depot/data".to_string(),
            listen_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 9632),
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
            routers: vec![net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562)],
            github_url: GITHUB_URL.to_string(),
            github_client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            github_client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
            insecure: false,
        }
    }
}

impl<'a> redis::IntoConnectionInfo for &'a Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        format!("redis://{}:{}",
                self.datastore_addr.ip(),
                self.datastore_addr.port())
            .into_connection_info()
    }
}

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4> {
        &self.routers
    }
}

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github_url
    }

    fn github_client_id(&self) -> &str {
        &self.github_client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github_client_secret
    }
}
