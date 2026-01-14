use blpapi::version_info::VersionInfo;

#[test]
pub fn test_version_info() {
    let version = VersionInfo::default();
    assert!(version.major_version != 0);
    assert!(version.minor_version != 0);
}
