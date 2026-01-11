fn order_weight(s: &str) -> String 
{
    let mut weights_as_str: Vec<_> = s.split_whitespace()
                                    .collect();
                                    
    
    //println!("{:?}", weights_as_vec);
    
    weights_as_str
                  .sort_by_key( |&weight| 
                  {
                    let sum_digts_weight: u32 = weight.chars()
                                                .filter_map(|c| c.to_digit(10))
                                                .sum();
                    (sum_digts_weight, weight)
                  });
    
    weights_as_str.join(" ")
}


fn main ()
{
    println!("{:?}", order_weight("2000 10003 1234000 44444444 9999 11 11 22 123") );
}
