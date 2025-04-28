// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// } // error will need to learn in chapter 10


struct Rectangle {
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle{
    fn sqaure(size:u32)-> Rectangle{
        Rectangle { width: size, height: size }
    }
}
fn main() {
    // let user1 =  User{
    //     active:true,
    //     username:"Bob",
    //     email:"Bobby",
    //     sign_in_count:0,
    // };
    // let width1: u32 = 30;
    // let height1: u32 = 50;
    // println!("The area of the rectangele is {} sqaure pixels",area(width1,height1) );
    // // let rect1: (u32, u32) = (10,15);
    // println!("the area of the triangle calculated with better syntax is {}",area_betther_syntax((width1,height1)));
    // //  Here can i  also use the rect one as the argument inside the area_betther_syntax like pas it as an argument

    // fn area(width: u32,height:u32) -> u32{
    //     width * height
    // }
    // fn area_betther_syntax(dimensons:(u32,u32)) ->u32{
    //     dimensons.0 * dimensons.1
    // }
    
    // A Betther way to do is using structres

    let react1: Rectangle = Rectangle{
    width:50,height:30,
    };
    let react2: Rectangle = Rectangle{
        width:10,height:10,
        };

        let react3: Rectangle = Rectangle{
            width:53,height:35,
            };
    println!("The calculated area of the rectangle {}",react1.area());
            println!("can fit {}",react3.can_hold(&react3));
            println!("can fit {}",react3.can_hold(&react2));
 let  _rectme = Rectangle::sqaure(25);



}
