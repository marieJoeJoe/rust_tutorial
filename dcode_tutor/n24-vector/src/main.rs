fn main() {

    let mut my_v = vec![1,2,3,4];


    //println!{"{}",my_v[2]}

    my_v.push(49);
    
    my_v.push(77);

    println!("{}",my_v.len());


    for i in 0..my_v.len() {
        println!("{}:{}",i,my_v[i]);
    }

    for num in my_v.iter() {
        println!("{}",num);
    }

}
