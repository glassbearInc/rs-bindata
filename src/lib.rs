#[allow(unused_imports)]
#[macro_use]
extern crate bindata_impl;

#[allow(unused)]
macro_rules! bindata {
    ($($path:expr),*) => {
        #[allow(unused)]
        #[derive(BinDataImpl)]
        $(
        #[BinDataImplContent(path=$path)]
        )*
        struct BinData;

        #[allow(unused)]
        pub fn get(key: &str) -> Option<Vec<u8>> {
            BinData.get(&key)
        }
    };
}


#[cfg(test)]
mod tests {
    use std::str;

    mod assets {
        bindata!("tests/data/a/");
    }

    mod assets2 {
        bindata!("tests/data/a/", "tests/data/b/");
    }

    #[test]
    fn embedded_data() {
        if let Some(georgie) = assets::get("tests/data/a/georgie-porgie") {
            assert_eq!(Ok("pudding and pie\n"), str::from_utf8(&georgie));
        }
    }
}
