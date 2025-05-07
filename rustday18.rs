
pub mod dog{
    pub fn walk(){
        println!("Thi");
    }
}

pub use dog::walk as e;



fn main() {


    e(); 

}


