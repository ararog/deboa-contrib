use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use hickory_resolver::{
    config::ResolverConfig, name_server::TokioConnectionProvider, Hosts, Resolver,
};
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct HickoryDnsResolver;

impl DnsResolver for HickoryDnsResolver {
    fn resolve(&self, host: String, _port: u16) -> DnsResolverFuture {
        let mut resolver = Resolver::builder_with_config(
            ResolverConfig::default(),
            TokioConnectionProvider::default(),
        )
        .build();

        let future = async move {
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
                .into_iter()
                .collect::<Vec<_>>();
            ips.shuffle(&mut rand::rng());
            Ok(ips)
        };

        Box::pin(future)
    }
}
