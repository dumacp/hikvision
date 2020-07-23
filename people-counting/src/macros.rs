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
    }
}

#[macro_export]
macro_rules! gen_setters {
    ($target:ty, $($field:ident : $k:tt $tpe:ty),+) => ($(
        crate::gen_setter! { $target, $field : $k $tpe }
    )+)
}