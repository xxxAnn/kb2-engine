pub fn parse_item(t: impl Into<String>) -> (usize, u64) {
    let t_str: String = t.into();

    let mut pair = t_str.split(':');
    let id: usize = pair.next().unwrap().parse().unwrap();
    let quantity: u64 = pair.next().unwrap().parse().unwrap();

    (id, quantity)
} 

pub fn parse_item_list(t: impl Into<String>) -> Vec<(usize, u64)> {
    let mut res = Vec::new();
    let inv_str: String = t.into();
    let x = inv_str.split(',');

    for indv in x {
        let (id, quantity) = parse_item(indv);

        res.push((id, quantity));
    }

    res
}