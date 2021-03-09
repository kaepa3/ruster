#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn sum(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data { 
        ret += x;
    }
    ret
}
pub fn sum_wrappiing(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        ret = x.wrapping_add(ret)
    }
    ret
}
