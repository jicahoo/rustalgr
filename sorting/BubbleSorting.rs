fn main(){

let mut array: [i32;3] = [1,3,4];
array[0]=5;
 for i in array.iter() {
        println!("{}", i);
 }

}

fn swap(arr: &mut [usize], i: usize, j: usize) {
	let mut temp = arr[i];
	arr[i] = arr[j];
	arr[j] = temp;
}