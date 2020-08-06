#[macro_export]
macro_rules! gen_setter {
    ($target:ty, $field:ident : into $t:ty) => {
        impl $target {
            /// Sets the field to the provided value and returns updated config object.
            pub fn $field<T: Into<$t>>(mut self, value: T) -> $target {
                self.$field = value.into();
                self
            }
        }
    };
    ($target:ty, $field:ident : val $t:ty) => {
        impl $target {
            /// Sets the field to the provided value and returns updated config object.
            pub fn $field(mut self, value: $t) -> $target {
                self.$field = value;
                self
            }
        }
    };
    ($target:ty, $map:ident, $it:path, $key:ident : val $t:ty) => {
        impl $target {
            /// Sets the field to the provided value and returns updated config object.
            pub fn $key(mut self, value: $t) -> $target {
                let elm = $it(value);
                let keyS = stringify!($key);
                self.$map.insert(String::from(keyS), elm);
                self
            }
        }
    }

}

#[macro_export]
macro_rules! gen_setters {
    ($target:ty, $($field:ident : $k:tt $tpe:ty),+) => {
        $(
            crate::gen_setter! { $target, $field : $k $tpe }
        )+
    };
    ($target:ty, $map:ident, $($it:path, $key:ident : $k:tt $tpe:ty),+) => {
        $(
            crate::gen_setter! { $target, $map, $it, $key: $k $tpe }
        )+
    }
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);