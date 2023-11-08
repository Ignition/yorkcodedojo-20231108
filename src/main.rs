use std::collections::{BTreeMap};

trait Bencode /*: Clone*/ {
    fn bencode(&self) -> String;
}

impl Bencode for &str {
    fn bencode(&self) -> String {
        let size = self.len();
        format!("{}:{}", size, self)
    }
}

impl Bencode for i64 {
    fn bencode(&self) -> String {
        format!("i{}e", self)
    }
}

impl<T: Bencode> Bencode for Vec<T> {
    fn bencode(&self) -> String {
        let main_string = self.iter()
            .fold(String::new(), |x, y| format!("{}{}", x, y.bencode()));
        format!("l{}e", main_string)
    }
}

impl<T: Bencode> Bencode for [T] {
    fn bencode(&self) -> String {
        let main_string = self.iter()
            .fold(String::new(), |x, y| format!("{}{}", x, y.bencode()));
        format!("l{}e", main_string)
    }
}


// impl Clone for BTreeMap<String, Box<dyn Bencode>> {
//     fn clone(&self) -> Self {
//         BTreeMap
//     }
// }

impl Bencode for BTreeMap<String, Box<dyn Bencode>> {
    fn bencode(&self) -> String {
        let main_string = self.iter().fold(String::new(), |acc, (k, v)|
            format!("{}{}{}", acc, k.as_str().bencode(), v.bencode()));
        format!("d{}e", main_string)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap};
    use std::vec;
    use crate::Bencode;

    #[test]
    fn ser_string() {
        assert_eq!("".bencode(), "0:");
        assert_eq!("spam".bencode(), "4:spam");
    }

    #[test]
    fn ser_int() {
        assert_eq!(0.bencode(), "i0e");
        assert_eq!((-0).bencode(), "i0e");
        assert_eq!(3.bencode(), "i3e");
        assert_eq!((-3).bencode(), "i-3e");
        assert_eq!(i64::MAX.bencode(), "i9223372036854775807e");
        assert_eq!(i64::MIN.bencode(), "i-9223372036854775808e");
    }

    #[test]
    fn ser_list() {
        assert_eq!(Vec::<i64>::new().bencode(), "le");
        assert_eq!(vec!(1, 2).bencode(), "li1ei2ee");
        assert_eq!([1, 2, 3].bencode(), "li1ei2ei3ee");
        assert_eq!(vec!("hi", "world").bencode(), "l2:hi5:worlde");
    }

    #[test]
    fn ser_dict() {
        type Bmap = BTreeMap<String, Box<dyn Bencode>>;

        assert_eq!(Bmap::new().bencode(), "de");
        let mut sut1 = Bmap::new();
        sut1.insert(String::from("name1"), Box::new("value"));
        sut1.insert(String::from("name2"), Box::new(50));
        assert_eq!(sut1.bencode(), "d5:name15:value5:name2i50ee");

        // let mut sut2 = Bmap::new();
        // sut2.insert(String::from("k1"), Box::new( sut1.clone()));
        // sut2.insert(String::from("k2"), Box::new( sut1));
        // assert_eq!(sut2.bencode(), "d2:k2d5:name15:value5:name2i50eee");
    }
}

fn main() {}