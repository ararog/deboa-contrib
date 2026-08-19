use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use hickory_resolver::{net::runtime::RuntimeProvider, Resolver};
use rand::seq::SliceRandom;
use std::{net::IpAddr, sync::Arc};

pub struct HickoryDnsResolver<P>
where
    P: RuntimeProvider,
{
    resolver: Arc<Resolver<P>>,
}

impl<P> HickoryDnsResolver<P>
where
    P: RuntimeProvider,
{
    pub fn new(resolver: Arc<Resolver<P>>) -> Self {
        Self { resolver }
    }
}

impl<P> Default for HickoryDnsResolver<P>
where
    P: RuntimeProvider + Default,
{
    fn default() -> Self {
        Self {
            resolver: Arc::new(
                Resolver::builder(P::default())
                    .expect("Could not create builder!")
                    .build()
                    .expect("Coult not build resolver!"),
            ),
        }
    }
}

impl<P> DnsResolver for HickoryDnsResolver<P>
where
    P: RuntimeProvider,
{
    fn resolve(&self, host: String, _port: u16) -> DnsResolverFuture {
        let resolver = self
            .resolver
            .clone();
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
