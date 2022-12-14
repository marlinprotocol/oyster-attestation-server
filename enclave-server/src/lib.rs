use nsm_io::{Request, Response};
use serde_bytes::ByteBuf;
use sysinfo::{System, SystemExt};
use serde::Serialize;

#[derive(Serialize)]
struct EnclaveConfig {
    total_memory: u64,
    total_cpus : usize
}

fn get_enclave_config() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();
    
    let config = EnclaveConfig {
        total_memory : sys.total_memory(),
        total_cpus : sys.cpus().len()
    };

    serde_json::to_string(&config).unwrap()
}

pub fn get_attestation_doc() -> Vec<u8> {
    let nsm_fd = nsm_driver::nsm_init();
    let enclave_config = get_enclave_config();

    let public_key = ByteBuf::from("my super secret key");
    let enclave_config = ByteBuf::from(enclave_config);

    let request = Request::Attestation {
        public_key: Some(public_key),
        user_data: Some(enclave_config),
        nonce: None,
    };

    let response = nsm_driver::nsm_process_request(nsm_fd, request);

    nsm_driver::nsm_exit(nsm_fd);

    match response {
        Response::Attestation { document } => document,
        _ => panic!("nsm driver returned invalid response: {:?}", response),
    }
}
