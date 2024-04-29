
pub  fn precentage(p: f32){
    println!("    〚{}〛 {}%    ", {
        let p = p/10.0;
        let mut n = "".to_string();
        for j in 1..11{
            if j as f32 <= p.floor(){
                n.push_str("▋▋▋");
            }
            else {n.push_str("░░░");}}
            n
    
    }, p as i32);
}

#[test]
fn precentages(){
    for i in 1..11{
        println!("    〚{}〛 {}%    ", {
            let mut n = "".to_string();
            for j in 1..11{
                if j <= i{
                    n.push_str("▋▋▋");
                }
                else {n.push_str("░░░");}}
                n
        
        }, i*10);
        println!();
    }
}