#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    extern crate vcx;

    use super::*;
    use self::vcx::settings;
    use self::vcx::connection;
    use self::vcx::credential;
    use self::vcx::issuer_credential;
    use self::vcx::disclosed_proof;
    use self::vcx::proof;
    use self::vcx::utils::libindy::wallet;
    use self::vcx::api::VcxStateType;
    use self::vcx::api::ProofStateType;
    use serde_json::Value;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_real_proof() {
        self::vcx::utils::logger::LoggerUtils::init();
        settings::set_to_defaults();
        //BE INSTITUTION AND GENERATE INVITE FOR CONSUMER
        self::vcx::utils::devsetup::setup_dev_env("test_real_proof");
        self::vcx::utils::libindy::anoncreds::libindy_prover_create_master_secret(wallet::get_wallet_handle(), settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        let alice = connection::build_connection("alice").unwrap();
        connection::connect(alice, Some("{}".to_string())).unwrap();
        let details = connection::get_invite_details(alice, true).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        self::vcx::utils::devsetup::be_consumer();
        let faber = connection::build_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, connection::get_state(faber));
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, connection::get_state(alice));
        connection::connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        self::vcx::utils::devsetup::be_institution();
        thread::sleep(Duration::from_millis(2000));
        connection::update_state(alice).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, connection::get_state(alice));
        // AS INSTITUTION SEND CREDENTIAL OFFER
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let schema_seq_no = 22;
        let credential_offer = issuer_credential::issuer_credential_create(schema_seq_no,
                                                            "1".to_string(),
                                                            settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                            "credential_name".to_string(),
                                                            credential_data.to_owned()).unwrap();
        issuer_credential::send_credential_offer(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND CREDENTIAL REQUEST
        self::vcx::utils::devsetup::be_consumer();
        let credential_offers = credential::get_credential_offer_messages(faber, None).unwrap();
        let offers: Value = serde_json::from_str(&credential_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        let credential = credential::credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, credential::get_state(credential).unwrap());
        credential::send_credential_request(credential, faber).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS INSTITUTION SEND CREDENTIAL
        self::vcx::utils::devsetup::be_institution();
        issuer_credential::update_state(credential_offer);
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_credential::get_state(credential_offer));
        issuer_credential::send_credential(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CREDENTIAL
        self::vcx::utils::devsetup::be_consumer();
        credential::update_state(credential).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, credential::get_state(credential).unwrap());
        // AS INSTITUTION SEND PROOF REQUEST
        self::vcx::utils::devsetup::be_institution();
        let requested_attrs = json!([
           {
              "schema_seq_no":schema_seq_no,
              "name":"address1",
              "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"address2",
              "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"city",
              "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"state",
              "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"zip",
              "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID
           }
        ]).to_string();

        let proof_req_handle = proof::create_proof("1".to_string(), requested_attrs, "[]".to_string(), "name".to_string()).unwrap();
        proof::send_proof_request(proof_req_handle, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND PROOF
        self::vcx::utils::devsetup::be_consumer();
        let requests = disclosed_proof::get_proof_request_messages(faber, None).unwrap();
        let requests: Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();
        let proof_handle = disclosed_proof::create_proof(self::vcx::utils::constants::DEFAULT_PROOF_NAME.to_string(), requests).unwrap();
        disclosed_proof::send_proof(proof_handle, faber).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, disclosed_proof::get_state(proof_handle).unwrap());
        thread::sleep(Duration::from_millis(5000));
        // AS INSTITUTION VALIDATE PROOF
        self::vcx::utils::devsetup::be_institution();
        proof::update_state(proof_req_handle);
        assert_eq!(proof::get_proof_state(proof_req_handle), ProofStateType::ProofValidated as u32);
        println!("proof validated!");
        self::vcx::utils::devsetup::cleanup_dev_env("test_real_proof");
    }
}