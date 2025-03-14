/// The ssh-agent must already be running on the machine.
/// This will work in windows git-bash and in Linux.
#[test]
fn test_win_git_bash_agent() {
    let var_ssh_auth_sock =
        std::env::var("SSH_AUTH_SOCK").expect("Env var SSH_AUTH_SOCK does not exist.");
    let path_ssh_auth_sock = std::path::PathBuf::from(&var_ssh_auth_sock);
    use ssh_agent_client_rs_git_bash::GitBash;
    let mut client =
        ssh_agent_client_rs_git_bash::Client::connect_to_git_bash_ssh_agent(&path_ssh_auth_sock)
            .expect("Cannot connect to ssh-agent.");

    client
        .remove_all_identities()
        .expect("remove_all_identities panicked");

    // list 0 identities
    let identities = client
        .list_identities()
        .expect("Failed to list identities.");
    assert_eq!(identities.len(), 0);

    // add 2 identities
    let key_1 = ssh_key::PrivateKey::from_openssh(
        std::fs::read(std::path::Path::new("tests/data/id_rsa"))
            .expect("read tests/data/id_rsa failed"),
    )
    .expect("from_openssh failed");
    client.add_identity(&key_1).expect("add_identity failed");

    let key_2 = ssh_key::PrivateKey::from_openssh(
        std::fs::read(std::path::Path::new("tests/data/id_ed25519"))
            .expect("read tests/data/id_ed25519 failed"),
    )
    .expect("from_openssh failed");
    client.add_identity(&key_2).expect("add_identity failed");

    // list 2 identities
    let identities = client.list_identities().expect("failed to list identities");
    assert_eq!(identities.len(), 2);

    let fingerprint_1 = identities[0].fingerprint(Default::default());
    assert_eq!(
        fingerprint_1.to_string(),
        "SHA256:GLEtSPhrq/14A520/k5cq9Yl2Ts7Pk9O5O2XQTW5Y+I"
    );

    let fingerprint_2 = identities[1].fingerprint(Default::default());
    assert_eq!(
        fingerprint_2.to_string(),
        "SHA256:5RBDOSvATUadn+TDt8S320/ozyfQqY525Cv70tGwCOE"
    );

    // signature
    let signature_1 = client
        .sign(key_1.public_key(), "123".as_bytes())
        .expect("sign failed");
    // first 8 bytes
    assert_eq!(
        &signature_1.as_bytes()[0..8],
        [154, 215, 145, 72, 144, 108, 134, 149]
    );

    let signature_2 = client
        .sign(key_2.public_key(), "123".as_bytes())
        .expect("sign failed");
    // first 8 bytes
    assert_eq!(
        &signature_2.as_bytes()[0..8],
        [174, 125, 54, 107, 53, 245, 135, 129]
    );

    // remove single identity
    client
        .remove_identity(&key_1)
        .expect("remove_identity failed");
    client
        .remove_identity(&key_2)
        .expect("remove_identity failed");

    // list 0 identities
    let identities = client.list_identities().expect("failed to list identities");
    assert_eq!(identities.len(), 0);
}
