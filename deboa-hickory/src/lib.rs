use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{
        DeboaError::{self, Dns},
        DnsError,
    },
};
use hickory_resolver::{
    config::ResolverConfig, net::runtime::TokioRuntimeProvider, Hosts, Resolver,
};
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct HickoryDnsResolver;

impl DnsResolver for HickoryDnsResolver {
    fn resolve(&self, host: String, _port: u16) -> DnsResolverFuture {
        let future = async move {
            let mut resolver = Resolver::builder_with_config(
                ResolverConfig::default(),
                TokioRuntimeProvider::default(),
            )
            .build()
            .map_err(|e| {
                DeboaError::Dns(DnsError::Resolve { host: host.clone(), message: e.to_string() })
            })?;

            if let Ok(hosts) = Hosts::from_system() {
                resolver.set_hosts(hosts.into());
            }

            let addrs = resolver
                .lookup_ip(&host)
                .await;
            if let Err(e) = addrs {
                return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
            };

            let mut ips = addrs
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            ips.shuffle(&mut rand::rng());
            Ok(ips)
        };

        Box::pin(future)
    }
}
