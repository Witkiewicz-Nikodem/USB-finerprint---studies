use crate::usb::get_usbs_string;
use crate::Usb;
use crate::my_io;
use std::io::{self, Write};
use std::num::ParseIntError;

pub enum List{
    W,
    B,
}


pub struct UserOperations{
    white_list: Vec<Usb>,
    black_list: Vec<Usb>,
    new_list: Vec<Usb>
}

impl UserOperations{
    pub fn new() -> Self{
        let white_list = match my_io::read_usbs(&"white_list.txt".to_string()){
            Ok(list) => list,
            Err(error) => {
                match error.to_string().as_str(){
                    "Nie można odnaleźć określonego pliku. (os error 2)" => vec![],
                    _ => {println!("{}",error.to_string().as_str());panic!("error in read white list: {}", error)}
                }
            },
        };

        let black_list = match my_io::read_usbs(&"black_list.txt".to_string()){
            Ok(list) => list,
            Err(error) => {
                match error.to_string().as_str(){
                    "Nie można odnaleźć określonego pliku. (os error 2)" => vec![],
                    _ => panic!("error in read black list: {}", error)
                }
            },
        };

        let new_list = Usb::all_from_str(&get_usbs_string());
        UserOperations{white_list,black_list,new_list}
    }

    pub fn decide_new_list(&mut self){
        println!("----------------------------------------------------------------------------------------------------------------");
        println!("znaleziono podlaczone urządzenia znajdujące się w white list: ");
        for i in 0..self.new_list.len(){
            if self.white_list.contains(&self.new_list[i]){
                self.new_list[i].show();
            }
        }
        println!("----------------------------------------------------------------------------------------------------------------");
        println!("znaleziono podlaczone urządzenia znajdujące się w black list: ");
        for i in 0..self.new_list.len(){
            if self.black_list.contains(&self.new_list[i]){
                self.new_list[i].show();
            }
        }

        for i in 0..self.new_list.len(){
            if !(self.black_list.contains(&self.new_list[i]) || self.white_list.contains(&self.new_list[i])){
                println!("do jakiej listy dodać to urządzenie:");
                self.new_list[i].show();

                let decision = get_option_from_user();
                match decision{
                    List::W => self.white_list.push(self.new_list[i].clone()),
                    List::B => self.black_list.push(self.new_list[i].clone()),
                }
            }
        }
        self.new_list.clear();
    }


    pub fn write_lists(&self) -> io::Result<()>{
        my_io::write_usbs(&self.white_list, &"white_list.txt".to_string())?;
        my_io::write_usbs(&self.black_list, &"black_list.txt".to_string())?;
        Ok(())
    }

    pub fn move_to_other_list(&mut self){
        self.show();
        println!("z jakiej listy chcesz przenieść ");
        let list = get_option_from_user();
        
        let index;


        match list {
            List::W => {
                if self.white_list.is_empty() {println!("nie mozesz przeniesc elementu z pustej listy"); return ()}
                index = self.get_index_from_user(&list);
                self.black_list.push(self.white_list.remove(index))
            },
            List::B => {
                if self.black_list.is_empty() {println!("nie mozesz przeniesc elementu z pustej listy"); return ()}
                index = self.get_index_from_user(&list);
                self.white_list.push(self.black_list.remove(index))
            },
        }
    }

    pub fn get_index_from_user(&self, list: &List) -> usize{
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let mut index: usize;

        loop{
            index = get_number_from_user("podaj index usb, który chcesz przenieść:");
            match list {
                List::W => {
                    if index < self.white_list.len() {return index;}
                    else {input.clear(); println!("podałeś zbyt dużą liczbę");}
                },
                List::B => {
                    if index < self.black_list.len() { return index;}
                    else {input.clear(); println!("podałeś zbyt dużą liczbę");}
                },
            }
        }
    }
    

    pub fn show(&self){
        println!("aktualny wykaz urządzeń: ");
        println!("----------------------------------------------------------------------------------------------------------------");
        print!("white list: ");
        if self.white_list.is_empty(){
            println!("pusta");
        }else{
            for usb in &self.white_list{
                usb.show();
            }
        }


        println!("----------------------------------------------------------------------------------------------------------------");
        print!("black list: ");
        if self.black_list.is_empty(){
            println!("pusta");
        }else{
            for usb in &self.black_list{
                usb.show();
            }
        }
        println!("----------------------------------------------------------------------------------------------------------------");
    }

    pub fn run(&mut self){
        self.decide_new_list();
        UserOperations::show_manual();
        let mut option;
        loop{
            option = get_number_from_user("co chcesz zrobić: ");
            
            match option {
                0 => {
                        self.new_list = Usb::all_from_str(&get_usbs_string());
                        self.decide_new_list()
                    },
                1 => self.show(),
                2 => self.move_to_other_list(),
                3 => UserOperations::show_manual(),
                4 => break,
                _ => println!("podales nieznana opcję spróbój jeszcze raz: "),
            }
        }
        self.write_lists();
    }


    fn show_manual(){
        println!(r#"Instrukcja obsługi programu:
        podaj 0 aby ponownie wczytać podpięte urządzenia
        podaj 1 aby pokazać aktualne listy
        podaj 2 aby przeniesc element miedzy listami
        podaj 3 aby pokazać instrukcję
        podaj 4 aby wyjść z programu :)"#);
    }


}


fn get_option_from_user() -> List{
    io::stdout().flush().unwrap();
    let mut input = String::new();

    println!("podaj [w] jeśli white list, [b] jeśli black list");
    loop {
        io::stdin().read_line(&mut input).expect("błąd w odczycie danych od użytkownika");
        let input_trimed = input.trim();

        match input_trimed {
            "w" => return List::W,
            "b" => return List::B,
            _ => {
                input.clear();
                println!("podano nieznana opcję, spróboj jeszcze raz! ")
            },
        }
    }
}

fn get_number_from_user(com: &str) -> usize{
    io::stdout().flush().unwrap();
    let mut input = String::new();

    println!("{}",com);
    loop {
        io::stdin().read_line(&mut input).expect("błąd w odczycie danych od użytkownika");
        let input_trimed = input.trim();
        let index_result: Result<usize,ParseIntError> = input_trimed.parse();

        match index_result {
            Ok(index) => return index,
            Err(_) => {input.clear(); println!("podałeś liczbę nie rzutowalną do typu usize, spróbój jeszcze raz. usize in <0,2^32>")},
        }
    }
}

