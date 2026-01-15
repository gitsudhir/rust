#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    /* This is a comment 
    let str = "1".to_string();
    let mut stack = vec![];
    for i in str.chars(){
        stack.push(i);
    }
    dbg!(&stack);
   loop {
       if stack.last() == Some(&'1'){
           stack.pop();
       }else{
           break;
       }
   }
   dbg!(&stack);
   let result = stack.into_iter().collect::<String>();
    dbg!("{}", result);
    */
    pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
       let sorted = {
           let mut v = nums.clone();
           v.sort();
           v
       };
       dbg!(&sorted); 
       let max_sum = sorted.iter().rev().take(k as usize).sum::<i32>();
       let min_sum = sorted.iter().take(k as usize).sum::<i32>();
       dbg!(max_sum);
       dbg!(min_sum);
       max_sum - min_sum
       
    }
//    let result = abs_difference(vec![100], 1);
    // dbg!("{}", result);
 pub fn roman_to_int(s: String) -> i32 {
  let mut roman_map = std::collections::HashMap::new();
    roman_map.insert('I', 1);
    roman_map.insert('V', 5);
    roman_map.insert('X', 10);
    roman_map.insert('L', 50);
    roman_map.insert('C', 100);     
    roman_map.insert('D', 500);
    roman_map.insert('M', 1000);

    dbg!(&roman_map);
    let chars: Vec<char> = s.chars().collect();
    dbg!(&chars.last());
    let mut result = 0;
    let mut i = 0;
    while i < chars.len() {
        let current_value = roman_map.get(&chars[i]).unwrap();
        let next_value = if i + 1 < chars.len() {
            roman_map.get(&chars[i + 1]).unwrap()
        } else {
            &0
        };
        if current_value < next_value {
            result += next_value - current_value;
            i += 2;
        } else {
            result += current_value;
            i += 1;
        }
    }
    result as i32
        
    }

    let result = roman_to_int("MCMXCIV".to_string());
    dbg!("{}", result);

}