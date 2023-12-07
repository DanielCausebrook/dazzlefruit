#[macro_export]
macro_rules! fork_properties {
    ($($prop:expr),*$(,)?) => {
        $($prop = $prop.fork();)*
    };
}

#[macro_export]
macro_rules! view_properties {
    ($($prop:expr),*$(,)?) => {
        vec![
            $($prop.view(),)*
        ]
    };
}

#[macro_export]
macro_rules! expose_prop {
    ($sel:ident, $prop:expr, $propType:ty, $getterName:ident, $setterName:ident) => {
        pub fn $getterName (&$sel) -> &$crate::pattern_builder::component::property::Prop<$propType> {
            &$prop
        }
        pub fn $setterName (&mut $sel, prop: impl $crate::pattern_builder::component::property::PropCore<Value=$propType>) {
            $prop = $crate::pattern_builder::component::property::PropCore::into_dyn(prop);
        }
    };
}