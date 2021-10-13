fn main(){
	

    println!("{} ",stringConcat("Hello"," thank you for teeching us."));
    
	println!("{}",subArrayAverage(&[3.2,2.5,1.5],(0,2)));
	
    println!("{:?}",arraySignum(&mut[-3,0,5]));


}

fn stringConcat(first: &'static str , second: &'static str ) -> String
{
    let f1 = first.to_string();
    let f2 = second.to_string();
    //f1 + &f2
    let f5 = format!("{}, {}", f1, f2);
    f5
    // first.push_str(&second);
    // first
	//z = first.push_str(second)
}



fn subArrayAverage(arr: &[f32], tup: (usize, usize) ) -> f32
{
    let first = tup.0;
    let second = tup.1;
    let new_arr = &arr[first..second];
    let mut total = 0.0;
    //println!("{:?}",newArr);  //to print the array
    for i in new_arr {
        total += i;
        //println!("{}",total);
    };
    //println!("{}",total);
    let length = new_arr.len() as f32;
    total/length
}

fn arraySignum(arr: &mut[i32]) -> &[i32]
{
    let length = arr.len();
    for i in (0..length) {
        if arr[i] > 0 { arr[i] = 1}
        else if arr[i] < 0 {arr[i] = -1}
        else {arr[i] = 0}
    };
    arr
}