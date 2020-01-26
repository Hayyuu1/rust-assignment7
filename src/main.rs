mod student{
  pub mod information{
      pub fn data(){
          println!("Student Name is abc");
          println!("Student Roll number is 123");
      }
  }


}

fn main() {
    crate::student::information::data();
}
