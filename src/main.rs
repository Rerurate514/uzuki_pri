use rand::Rng;

const NISSU: u32 = 30;

fn main() {
    let mut girl = Kuri::new();

    girl.do_sequence();
    println!("\n\n-------------------------------------------");
}

struct Kuri{
    jutikuti: u32,
    kuri_size: f32,//単位 mm
    uzuki: u32,
    dankai: u32,
    iki: u32
}

impl Kuri{
    fn new() -> Self {
        Kuri {
            jutikuti: 0,
            kuri_size: 5.0,
            uzuki: 0,
            dankai: 1,
            iki: 0
        }
    }

    fn do_sequence(&mut self){
        for day in 0..NISSU {
            self.day_circle();
            self.print_day_parameter(day);
        }
    }    

    fn day_circle(&mut self){
        for hour in 1..16 { //8時間睡眠と仮定
            self.check_akuming();

            self.print_hour_parameter(hour);
        }
    }

    fn check_akuming(&mut self){
        if self.uzuki >= 200 {
            self.is_iki( 100);
        }
        else if self.uzuki >= 100 {
            self.is_iki(90);
        }
        else if self.uzuki >= 50 {
            self.is_iki(80);
        }
        else if self.uzuki >= 20 {
            self.is_iki(50);
        }
        else if self.uzuki >= 10 {
            self.is_iki(30);
        }
        else if self.uzuki >= 5 {
            self.is_iki(15);
        }
        else if self.uzuki >= 1 {
            self.is_iki(10);
        }
        else {
            self.is_iki(1);
        }
    }

    fn is_iki(&mut self, chance: i32){
        let mut rng = rand::thread_rng();
        let value: i32 = rng.gen_range(0..100);

        if value >= chance {
            self.inc_jutikuti();
        }
        else {
            if self.hukaiki() {
                return;
            }
            self.iki(1);
        }
    }
    
    fn inc_jutikuti(&mut self){
        self.jutikuti = self.jutikuti + 1;
        self.kuri_size = self.kuri_size + 0.1;
        
        self.inc_uzuki();

        self.inc_dankai();
    }

    fn inc_uzuki(&mut self){
        if self.dankai == 1 {
            self.uzuki = self.jutikuti;
        }
        else if self.dankai == 2 {
            self.uzuki = self.jutikuti * 2;
        }
        else if self.dankai == 3 {
            self.uzuki = self.jutikuti * 5;
        }
        else if self.dankai == 4 {
            self.uzuki = self.jutikuti * 10;
        }
        else if self.dankai == 5 {
            self.uzuki = self.jutikuti * 20;
        }
    }

    fn inc_dankai(&mut self){
        if self.jutikuti >= 840 && self.dankai == 4 {
            self.dankai = 5
        }
        else if self.jutikuti >= 540  && self.dankai == 3{
            self.dankai = 4
        }
        else if self.jutikuti >= 300 && self.dankai == 2 {
            self.dankai = 3
        }
        else if self.jutikuti >= 120 && self.dankai == 1 {
            self.dankai = 2
        }
    }

    fn iki(&mut self, fac: u32){
        self.iki += 1;

        if self.dankai == 1 {
            self.uzuki = 0;
        }
        else if self.dankai == 2 {
            self.uzuki -= self.uzuki / (2 * fac);
        }
        else if self.dankai == 3 {
            self.uzuki -= self.uzuki / (4 * fac);
        }
        else if self.dankai == 4 {
            self.uzuki -= self.uzuki / (8 * fac);
        }
        else if self.dankai == 5 {
            self.uzuki -= self.uzuki / (16 * fac);
        }
    }

    fn hukaiki(&mut self) -> bool{
        let mut rng = rand::thread_rng();
        let value: i32 = rng.gen_range(0..100);

        if value >= 90 {
            self.iki(2);
            self.iki += 9;
            return true;
        }

        return  false;
    }

    fn print_day_parameter(&mut self, day: u32){
        println!("日数:{}\t\t呪蓄値:{}\tsize:{:.1} [mm]   \t疼き:{}  \t段階:{}\t\tいき:{}", day, self.jutikuti, self.kuri_size, self.uzuki, self.dankai, self.iki);
    }

    fn print_hour_parameter(&mut self, hour: u32){
        //println!("時間:{}\t\t呪蓄値:{}\tsize:{:.1} [mm]   \t疼き:{}  \t段階:{}\t\tいき:{}", hour, self.jutikuti, self.kuri_size, self.uzuki, self.dankai, self.iki);
    }
}

