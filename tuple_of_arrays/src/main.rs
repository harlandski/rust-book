fn main() {
  let t = ([1; 2], [3; 4]);
  println!("{:?}",t);
  let (a, _) = t;
  println!("{:?}",a);
  println!("{}", a[0] + t.1[0]); 
}
