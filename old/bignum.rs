use std::fmt;

pub struct BigNum{
    data  : Vec<u64>,
    comma : i128
}

impl BigNum{

    pub fn from(&value: &u64) -> BigNum{
        BigNum{data:vec![value], comma:0}
    }

    pub fn add(&mut self, other: &BigNum){
        let max: usize = std::cmp::max(self.data.len(), other.data.len());
        let mut carry : u8 = 0;
        for i in 0..max{

            let mut tmp: u128;

            if i >= self.data.len() {
                tmp = other.data[i] as u128;
            }else if i >= other.data.len() {
                tmp = self.data[i] as u128;
            }else{
                tmp = self.data[i] as u128 + other.data[i] as u128;
                if tmp > u64::MAX as u128 {
                    carry = 1;
                    tmp -= u64::MAX as u128;
                }
            }

            self.data[i] = tmp as u64;

            if carry == 1 {
                if i == self.data.len(){
                    self.data.push(1);
                }else{
                    self.data[i+1] += 1;
                }
                carry = 0;
            }
        }
    }
    
    pub fn mult(mut self, other: BigNum){
        let max: usize = std::cmp::max(self.data.len(), other.data.len());
        let mut carry : u128 = 0;
        for i in 0..max{

            let mut tmp: u128;

            if i >= self.data.len(){
                tmp = other.data[i] as u128;
            }else if i >= other.data.len(){
                tmp = self.data[i] as u128;
            }else{
                tmp = self.data[i] as u128 * other.data[i] as u128;
                if tmp > u64::MAX as u128{
                    carry = tmp;
                    tmp -= (u64::MAX as u64) as u128;
                    carry -= tmp;
                }
            }

            self.data[i] += tmp as u64;

            if carry != 0{
                if i == self.data.len(){
                    self.data.push(1);
                }else{
                    self.data[i+1] += (carry >> 64) as u64;
                }
                carry = 0;
            }
        }
    }

    /*pub fn div(&self, &other: &BigNum) -> BigNum{
        
    }

    fn trimm(&self) -> BigNum{

    }*/

}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in self.data.iter() {
            write!(f, "{}", elem);
        }
        write!(f, "e{}", self.comma);
        Ok(())
    }
}



pub struct BigNumComplex{
    r : BigNum,
    i : BigNum
}

impl BigNumComplex{

    pub fn new(r: BigNum, i: BigNum) -> BigNumComplex{
        BigNumComplex{r, i}
    }

    /*pub fn add(&self, &other: &BigNumComplex) -> BigNumComplex{

    }

    pub fn mult(&self, &other: &BigNumComplex) -> BigNumComplex{

    }

    pub fn div(&self, &other: &BigNumComplex) -> BigNumComplex{

    }*/

}