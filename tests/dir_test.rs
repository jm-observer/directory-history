use directory_history::ty::Dir;
use std::path::PathBuf;

#[test]
fn test() {
    let data_before = std::fs::read("c.4.json").unwrap();
    let dir_before: Dir = serde_json::from_slice(&data_before).unwrap();
    let vb = dir_before.find_dir(".VirtualBox").unwrap();

    let data = serde_json::to_vec(vb).unwrap();
    std::fs::write("vb.old.json", data).unwrap();
    println!("{:?}", vb);

    let data_before = std::fs::read("c.3.json").unwrap();
    let dir_before: Dir = serde_json::from_slice(&data_before).unwrap();
    let vb = dir_before.find_dir(".VirtualBox").unwrap();

    let data = serde_json::to_vec(vb).unwrap();
    std::fs::write("vb.new.json", data).unwrap();
    println!("{:?}", vb);
}
