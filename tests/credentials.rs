// tests/credentials.rs
// Export hardcoded example credentials and read them back in

extern crate libtwitch_rs;

use libtwitch_rs::Credentials;

#[test]
fn test_credential_export() {
    let cred = Credentials {
        client_id: "13211542".to_string(),
        //channel_id: "31244131".to_string(),
        token: Some("OAuth:1839213891u389u1389183139".to_string()),
    };

    Credentials::write_to_file(&cred, "tests/credentials.toml".to_string());
}

#[test]
fn test_credential_import() {
    let cred = Credentials::set_from_file("tests/credentials.toml".to_string());
    assert_eq!(cred.client_id, "13211542".to_string());
    assert_eq!(
        cred.token,
        Some("OAuth:1839213891u389u1389183139".to_string())
    );
}
