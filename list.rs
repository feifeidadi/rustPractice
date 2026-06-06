fn main()
{
  let mut v1 = vec![1, 10 , 55, 3, 7];
  let mut v2 = vec![8, 30 , 51, 13, 99];

  println!("{:?}", v1);
  println!("{:?}", v2);

  v1.sort();
  v2.sort();

  println!("After sort:");
  println!("{:?}", v1);
  println!("{:?}", v2);

  v1.append(&mut v2);

  println!("After append v2 into v1:");
  println!("{:?}", v1);
  println!("{:?}", v2);

  println!("After sort v1:");
  v1.sort();
  println!("{:?}", v1);
}
