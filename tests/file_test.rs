use std::path::PathBuf;

#[test]
fn test() {
    // let path: PathBuf = "c:\\Users\\36225\\AppData\\Local\\Microsoft\\WindowsApps\\Microsoft.XboxGamingOverlay_8wekyb3d8bbwe\\GameBarElevatedFT_Alias.exe".into();
    let path: PathBuf =
        "C:\\Users\\36225\\AppData\\Local\\Microsoft\\VisualStudio\\Packages\\_Instances\\08954255\\state.json"
            .into();
    let metadata = std::fs::File::open(path).unwrap().metadata().unwrap();
    println!("{:?} {}", metadata, metadata.permissions().readonly());

    let path: PathBuf = "c:\\Users\\36225\\AppData\\Local\\Microsoft\\WindowsApps\\Microsoft.XboxGamingOverlay_8wekyb3d8bbwe\\GameBarElevatedFT_Alias.exe".into();

    println!("{:?}", path.metadata());
}
