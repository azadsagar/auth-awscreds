use ini::Ini;
use std::env;

pub struct AWSCreds {
    pub profile_name: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String
}

pub fn configcreds(creds: &AWSCreds) {
    let filename = format!("{}/.aws/credentials",env::var("HOME").unwrap());

    let mut conf = Ini::load_from_file(&filename).unwrap();

    conf.with_section(Some(&creds.profile_name))
    .set("aws_access_key_id", &creds.aws_access_key_id)
    .set("aws_secret_access_key", &creds.aws_secret_access_key)
    .set("aws_session_token", &creds.aws_session_token);

    conf.write_to_file(filename).unwrap();
}

pub fn config_profile(profile_name: &String,app_domain: &String){
    let filename = format!("{}/.auth-awscreds",env::var("HOME").unwrap());

    let mut conf = Ini::new();

    conf.with_section(Some("default"))
    .set("profile_name",profile_name)
    .set("app_domain",app_domain);

    conf.write_to_file(filename).unwrap();

}

pub fn read_profile() -> (String, String) {
    let filename = format!("{}/.auth-awscreds",env::var("HOME").unwrap());
    let mut conf = Ini::load_from_file(&filename).unwrap();

    let profile_name = conf.with_section(Some("default"))
    .get("profile_name").expect("profile_name").to_string();

    let app_domain = conf.with_section(Some("default"))
    .get("app_domain").expect("app_domain").to_string();

    return (profile_name,app_domain);
}