use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
 pub struct Usb {
    name: String,
    hash: String,
}

impl Usb {
    pub fn all_from_str(tmp: &str) -> Vec<Self>{
        let mut result: Vec<Usb> = vec![];
        let mut coursor = 0;

        while match Usb::one_from_str(&tmp[coursor..]){
            (None, None) => false,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(usb_hash), Some(new_coursor)) => {

                coursor += new_coursor;
                result.push(usb_hash);
                true
            },
        } && coursor <= tmp.len(){};
        result
    }




    fn one_from_str(tmp: &str) -> (Option<Self>, Option<usize>) {
        let mut coursor = 0;
        let hash = match tmp.find(';') {
            Some(n) => {
                let result = Some(tmp[coursor..coursor + n].to_string());
                coursor += n + 1;
                result
            }
            None => None,
        };
        let name = match tmp[coursor..].find(';') {
            Some(n) => {
                let result = Some(tmp[coursor..coursor + n].to_string());
                coursor += n + 1;
                result
            }
            None => None,
        };

        match (name, hash) {
            (Some(name), Some(hash)) => (Some(Usb{name,hash}),Some(coursor)),
            (None, None) => (None, None),
            (None, Some(_)) => (None, None),
            (Some(_), None) => (None, None)
        }
    }

    pub fn show(&self){
        println!("nazwa: {} | hash: {}",self.name, self.hash);
    }
}


pub fn get_usbs_string() -> String {
    // sciezka do skryptu powershell
    let script_path = r"D:\DTU\get_pnp.ps1";
    // Uruchomienie skryptu PowerShell
    let output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(script_path)
        .output()
        .expect("Failed to execute script");

    // Wyświetlenie wyjścia standardowego
    let zmienna = String::from_utf8_lossy(&output.stdout);
    let zmienna_as_ref = zmienna.as_ref();

    // Wyświetlenie błędów, jeśli wystąpiły
    if !output.stderr.is_empty() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    zmienna_as_ref.to_string()
}