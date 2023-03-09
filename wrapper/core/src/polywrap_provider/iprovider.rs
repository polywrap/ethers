use crate::wrap::{IProvider, IProviderModule};

pub fn get_iprovider() -> IProviderModule {
    let impls = IProvider::get_implementations();
    if impls.len() < 1 {
        panic!("The Ethereum wrapper requires a registered implementation of the EthereumProvider interface. \
            You can register an interface implementation in your Polywrap Client configuration.");
    }
    let iprovider = IProviderModule::new(impls[0].clone());
    iprovider
}