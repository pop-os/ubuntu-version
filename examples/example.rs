use ubuntu_version::{Codename, Version};
use std::convert::TryFrom;

fn main() {
    let _version = "18.04 LTS".parse::<Version>().unwrap();
    let codename = "bionic".parse::<Codename>().unwrap();

    let version = Version::from(codename);

    if let Ok(codename) = Codename::try_from(version) {
        println!("{} ({})", version, codename);
        println!("{}", <&'static str>::from(codename));
    }
}
