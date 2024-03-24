use aws_nitro_enclaves_nsm_api::api::{Request, Response};
use aws_nitro_enclaves_nsm_api::driver as nsm_driver;
use serde::Serialize;
use serde_bytes::ByteBuf;

#[derive(Serialize)]
struct EnclaveConfig {
    total_memory: u64,
    total_cpus: usize,
}

pub fn get_attestation_doc(pub_key: &[u8]) -> Vec<u8> {
    let nsm_fd = nsm_driver::nsm_init();

    let public_key = ByteBuf::from(pub_key);

    let request = Request::Attestation {
        public_key: Some(public_key),
        user_data: None,
        nonce: None,
    };

    let response = nsm_driver::nsm_process_request(nsm_fd, request);

    nsm_driver::nsm_exit(nsm_fd);

    match response {
        Response::Attestation { document } => document,
        _ => panic!("nsm driver returned invalid response: {:?}", response),
    }
}
