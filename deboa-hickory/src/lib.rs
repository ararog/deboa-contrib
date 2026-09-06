use deboa::{
    dns::DnsResolver,
    errors::{
        DeboaError::{self},
        DnsError,
    },
    Result,
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
    async fn resolve(&self, host: String, _port: u16) -> Result<Vec<IpAddr>> {
        let resolver = self
            .resolver
            .clone();
        let Ok(addrs) = resolver
            .lookup_ip(&host)
            .await
        else {
            return Err(DeboaError::Dns(DnsError::Resolve {
                host,
                message: "Failed to resolve host".to_string(),
            }));
        };

        let mut ips: Vec<IpAddr> = addrs
            .iter()
            .collect();

        ips.shuffle(&mut rand::rng());

        Ok(ips)
    }
}
