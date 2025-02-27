//! Unified data access to different storage services.
//!
//! ## Supported storage services
//!
//! | Scheme        | Description                              | Feature flag          |
//! |---------------|------------------------------------------|-----------------------|
//! | `azblob`      | Azure Storage Blob services.             | `accessor`            |
//! | `azdfs`       | Azure Data Lake Storage Gen2 services.   | `accessor`            |
//! | `dashmap`     | Dashmap backend.                         | `accessor-dashmap`    |
//! | `fs`          | POSIX alike file system.                 | `accessor`            |
//! | `ftp`         | FTP and FTPS.                            | `accessor-ftp`        |
//! | `gcs`         | Google Cloud Storage services.           | `accessor`            |
//! | `ghac`        | Github Action Cache services.            | `accessor`            |
//! | `ipfs`        | InterPlanetary File System HTTP gateway. | `accessor-ipfs`       |
//! | `ipmfs`       | InterPlanetary File System MFS API.      | `accessor`            |
//! | `memcached`   | Memcached services.                      | `accessor-memcached`  |
//! | `memory`      | In memory backend.                       | `accessor`            |
//! | `minio`       | MinIO services.                          | `accessor`            |
//! | `moka`        | Moka backend.                            | `accessor-moka`       |
//! | `obs`         | Huawei Cloud Object Storage services.    | `accessor`            |
//! | `oss`         | Aliyun Object Storage Service.           | `accessor`            |
//! | `redis`       | Redis services.                          | `accessor-redis`      |
//! | `s3`          | AWS S3 alike services.                   | `accessor`            |
//! | `sled`        | Sled services.                           | `accessor-sled`       |
//! | `webdav`      | WebDAV services.                         | `accessor`            |
//! | `webhdfs`     | WebHDFS services.                        | `accessor`            |
//!

use crate::{extend::TomlTableExt, state::State};
use opendal::{
    layers::{MetricsLayer, RetryLayer, TracingLayer},
    services::{Azblob, Azdfs, Fs, Gcs, Ghac, Ipmfs, Memory, Obs, Oss, Webdav, Webhdfs, S3},
    Error,
    ErrorKind::Unsupported,
    Operator,
};
use std::sync::LazyLock;
use toml::Table;

#[cfg(feature = "accessor-dashmap")]
use opendal::services::Dashmap;
#[cfg(feature = "accessor-ftp")]
use opendal::services::Ftp;
#[cfg(feature = "accessor-ipfs")]
use opendal::services::Ipfs;
#[cfg(feature = "accessor-memcached")]
use opendal::services::Memcached;
#[cfg(feature = "accessor-moka")]
use opendal::services::Moka;
#[cfg(feature = "accessor-redis")]
use opendal::services::Redis;
#[cfg(feature = "accessor-sled")]
use opendal::services::Sled;

/// Global storage accessor built on the top of [`opendal`](https://crates.io/crates/opendal).
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalAccessor;

impl GlobalAccessor {
    /// Constructs a new operator with the configuration for the specific storage service,
    /// returning an error if it fails.
    pub fn try_new_operator(scheme: &'static str, config: &Table) -> Result<Operator, Error> {
        let operator = match scheme {
            "azblob" => {
                let mut builder = Azblob::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(container) = config.get_str("container") {
                    builder.container(container);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(account_name) = config.get_str("account-name") {
                    builder.account_name(account_name);
                }
                if let Some(account_key) = config.get_str("account-key") {
                    builder.account_key(account_key);
                }
                if let Some(sas_token) = config.get_str("sas-token") {
                    builder.sas_token(sas_token);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "azdfs" => {
                let mut builder = Azdfs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(filesystem) = config.get_str("filesystem") {
                    builder.filesystem(filesystem);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(account_name) = config.get_str("account-name") {
                    builder.account_name(account_name);
                }
                if let Some(account_key) = config.get_str("account-key") {
                    builder.account_key(account_key);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-dashmap")]
            "dashmap" => {
                let builder = Dashmap::default();
                Ok(Operator::new(builder)?.finish())
            }
            "fs" => {
                let mut builder = Fs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(atomic_write_dir) = config.get_str("atomic-write-dir") {
                    builder.atomic_write_dir(atomic_write_dir);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-ftp")]
            "ftp" => {
                let mut builder = Ftp::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(user) = config.get_str("user") {
                    builder.user(user);
                }
                if let Some(password) = State::decrypt_password(config) {
                    builder.password(password.as_ref());
                }
                Ok(Operator::new(builder)?.finish())
            }
            "gcs" => {
                let mut builder = Gcs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(bucket) = config.get_str("bucket") {
                    builder.bucket(bucket);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(service_account) = config.get_str("service-account") {
                    builder.service_account(service_account);
                }
                if let Some(credential) = config.get_str("credential") {
                    builder.credential(credential);
                }
                if let Some(credential_path) = config.get_str("credential-path") {
                    builder.credential_path(credential_path);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "ghac" => {
                let mut builder = Ghac::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(version) = config.get_str("version") {
                    builder.version(version);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-ipfs")]
            "ipfs" => {
                let mut builder = Ipfs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "ipmfs" => {
                let mut builder = Ipmfs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-memcached")]
            "memcached" => {
                let mut builder = Memcached::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(default_ttl) = config.get_duration("default-ttl") {
                    builder.default_ttl(default_ttl);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "memory" => {
                let builder = Memory::default();
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-moka")]
            "moka" => {
                let mut builder = Moka::default();
                if let Some(name) = config.get_str("name") {
                    builder.name(name);
                }
                if let Some(max_capacity) = config.get_u64("max-capacity") {
                    builder.max_capacity(max_capacity);
                }
                if let Some(time_to_live) = config.get_duration("time-to-live") {
                    builder.time_to_live(time_to_live);
                }
                if let Some(time_to_idle) = config.get_duration("time-to-idle") {
                    builder.time_to_idle(time_to_idle);
                }
                if let Some(segments) = config.get_usize("segments") {
                    builder.segments(segments);
                }
                if let Some(thread_pool_enabled) = config.get_bool("thread-pool-enabled") {
                    builder.thread_pool_enabled(thread_pool_enabled);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "obs" => {
                let mut builder = Obs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(bucket) = config.get_str("bucket") {
                    builder.bucket(bucket);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(access_key_id) = config.get_str("access-key-id") {
                    builder.access_key_id(access_key_id);
                }
                if let Some(secret_access_key) = config.get_str("secret_access_key") {
                    builder.secret_access_key(secret_access_key);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "oss" => {
                let mut builder = Oss::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(bucket) = config.get_str("bucket") {
                    builder.bucket(bucket);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(presign_endpoint) = config.get_str("presign-endpoint") {
                    builder.presign_endpoint(presign_endpoint);
                }
                if let Some(access_key_id) = config.get_str("access-key-id") {
                    builder.access_key_id(access_key_id);
                }
                if let Some(access_key_secret) = config.get_str("access-key-secret") {
                    builder.access_key_secret(access_key_secret);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-redis")]
            "redis" => {
                let mut builder = Redis::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(username) = config.get_str("username") {
                    builder.username(username);
                }
                if let Some(password) = State::decrypt_password(config) {
                    builder.password(password.as_ref());
                }
                if let Some(db) = config.get_i64("db") {
                    builder.db(db);
                }
                if let Some(default_ttl) = config.get_duration("default-ttl") {
                    builder.default_ttl(default_ttl);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "s3" | "minio" => {
                let mut builder = S3::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(bucket) = config.get_str("bucket") {
                    builder.bucket(bucket);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(region) = config.get_str("region") {
                    builder.region(region);
                }
                if let Some(access_key_id) = config.get_str("access-key-id") {
                    builder.access_key_id(access_key_id);
                }
                if let Some(secret_access_key) = config.get_str("secret-access-key") {
                    builder.secret_access_key(secret_access_key);
                }
                if let Some(role_arn) = config.get_str("role-arn") {
                    builder.role_arn(role_arn);
                }
                if let Some(external_id) = config.get_str("external-id") {
                    builder.external_id(external_id);
                }
                Ok(Operator::new(builder)?.finish())
            }
            #[cfg(feature = "accessor-sled")]
            "sled" => {
                let mut builder = Sled::default();
                if let Some(dir) = config.get_str("data-dir") {
                    builder.datadir(dir);
                }
                Ok(Operator::new(builder)?.finish())
            }
            "webdav" => {
                let mut builder = Webdav::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(username) = config.get_str("username") {
                    builder.username(username);
                }
                if let Some(password) = State::decrypt_password(config) {
                    builder.password(password.as_ref());
                }
                Ok(Operator::new(builder)?.finish())
            }
            "webhdfs" => {
                let mut builder = Webhdfs::default();
                if let Some(root) = config.get_str("root") {
                    builder.root(root);
                }
                if let Some(endpoint) = config.get_str("endpoint") {
                    builder.endpoint(endpoint);
                }
                if let Some(delegation) = config.get_str("delegation") {
                    builder.delegation(delegation);
                }
                Ok(Operator::new(builder)?.finish())
            }
            _ => Err(Error::new(Unsupported, "scheme is unsupported")),
        };
        operator.map(|op| {
            op.layer(TracingLayer)
                .layer(MetricsLayer)
                .layer(RetryLayer::new())
        })
    }

    /// Gets the operator for the specific storage service.
    #[inline]
    pub fn get(name: &'static str) -> Option<&'static Operator> {
        GLOBAL_ACCESSOR
            .iter()
            .find_map(|(key, operator)| (key == &name).then_some(operator))
    }
}

/// Global storage accessor.
static GLOBAL_ACCESSOR: LazyLock<Vec<(&'static str, Operator)>> = LazyLock::new(|| {
    let mut operators = Vec::new();
    let memory_operator = Operator::new(Memory::default())
        .expect("fail to create an operator for the memory accessor")
        .layer(TracingLayer)
        .layer(MetricsLayer)
        .layer(RetryLayer::new())
        .finish();
    operators.push(("memory", memory_operator));

    if let Some(accessors) = State::shared().config().get_array("accessor") {
        for accessor in accessors.iter().filter_map(|v| v.as_table()) {
            let scheme = accessor.get_str("scheme").unwrap_or("unkown");
            let name = accessor.get_str("name").unwrap_or(scheme);
            let operator = GlobalAccessor::try_new_operator(scheme, accessor)
                .unwrap_or_else(|err| panic!("fail to build `{scheme}` operator: {err}"));
            operators.push((name, operator));
        }
    }
    operators
});
