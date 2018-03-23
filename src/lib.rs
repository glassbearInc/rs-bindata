// Copyright 2018 Glassbear, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


#[allow(unused_imports)]
#[macro_use]
extern crate bindata_impl;

#[allow(unused)]
#[macro_export]
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

    mod multiassets {
        bindata!("tests/data/a/", "tests/data/b/");
    }

    mod absolute_assets {
        bindata!("/tmp");
    }

    #[test]
    fn embedded_data() {
        if let Some(georgie) = assets::get("tests/data/a/georgie-porgie") {
            assert_eq!(Ok("pudding and pie\n"), str::from_utf8(&georgie));
        }
    }
}
