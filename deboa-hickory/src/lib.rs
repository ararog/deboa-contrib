use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
    Result,
};
use hickory_resolver::{
    config::{ResolverConfig, GOOGLE},
    net::runtime::TokioRuntimeProvider,
    ResolverBuilder, TokioResolver,
};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use std::{net::IpAddr, sync::Arc};

static GLOBAL_RESOLVER: Lazy<Arc<TokioResolver>> = Lazy::new(|| {
    let resolver = TokioResolver::builder_with_config(
        ResolverConfig::udp_and_tcp(&GOOGLE),
        TokioRuntimeProvider::default(),
    )
    .build()
    .expect("Failed to create Hickory resolver");
    Arc::new(resolver)
});

#[derive(Default)]
pub struct HickoryDnsResolver;

impl HickoryDnsResolver {
    pub fn builder() -> Result<ResolverBuilder<TokioRuntimeProvider>> {
        TokioResolver::builder_tokio()
            .map_err(|e| Dns(DnsError::Resolver { message: e.to_string() }))
    }
}

impl DnsResolver for HickoryDnsResolver {
    fn resolve(&self, host: String, _port: u16) -> DnsResolverFuture {
        let resolver = GLOBAL_RESOLVER.clone();
        let future = async move {
            let mut ips: Vec<IpAddr> = {
                let addrs = resolver
                    .lookup_ip(&host)
                    .await;
                if let Err(e) = addrs {
                    return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
                }

                addrs
                    .unwrap()
                    .iter()
                    .collect()
            };

            ips.shuffle(&mut rand::rng());

            Ok(ips)
        };

        Box::pin(future)
    }
}
