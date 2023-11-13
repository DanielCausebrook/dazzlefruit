#[macro_export]
macro_rules! impl_component {
    ($($rest:tt)*) => {
        $crate::__internal_impl_component!(begin $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_impl_component {
    // begin:        < generics  |  rest
    // generics:     (*/*) tt generics  |  (*/*<) < generics  |  (<*/*) > generics  |  () > rest
    // rest:


    // Starts with brackets
    (begin < $($rest:tt)+) => {
        $crate::__internal_impl_component!(generics () () $($rest)*);
    };

    // No brackets
    (begin $($rest:tt)*) => {
        $crate::__internal_impl_component!(rest () $($rest)*);
    };

    // generics (generics) (STACK) *

    // End
    (generics ($($generics:tt)*) () > $($rest:tt)*) => {
        $crate::__internal_impl_component!(rest ($($generics)*) $($rest)*);
    };

    // Token
    (generics ($($generics:tt)*) ($($stack:tt)*) $first:tt $($rest:tt)*) => {
        $crate::__internal_impl_component!(generics ($($generics)* $first) ($($stack)*) $($rest)*);
    };

    // Open bracket
    (generics ($($generics:tt)*) ($($stack:tt)*) < $($rest:tt)*) => {
        $crate::__internal_impl_component!(generics ($($generics)* $first) ($($stack)* <) $($rest)*);
    };

    // Close bracket
    (generics ($($generics:tt)*) (< $($stack:tt)*) > $($rest:tt)*) => {
        $crate::__internal_impl_component!(generics ($($generics)* $first) ($($stack)*) $($rest)*);
    };

    (rest ($($generics:tt)*) $sel:ident: $struct:ty, $config:expr, $component_type:expr) => {
        impl<$($generics)*> $crate::Component for $struct {
            fn config(&$sel) -> &dyn $crate::pattern_builder::component::ComponentConfig {
                &$config
            }
            fn config_mut(&mut $sel) -> &mut dyn $crate::pattern_builder::component::ComponentConfig {
                &mut $config
            }
            fn component_type(&$sel) -> &'static str {
                $component_type
            }
        }
    };
}

#[macro_export]
macro_rules! impl_component_config {
    ($($rest:tt)*) => {
        $crate::__internal_impl_component_config!(begin $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_impl_component_config {
    // begin:        < generics  |  rest
    // generics:     (*/*) tt generics  |  (*/*<) < generics  |  (<*/*) > generics  |  () > rest
    // rest:


    // Starts with brackets
    (begin < $($rest:tt)+) => {
        $crate::__internal_impl_component_config!(generics () () $($rest)*);
    };

    // No brackets
    (begin $($rest:tt)*) => {
        $crate::__internal_impl_component_config!(rest () $($rest)*);
    };

    // generics (generics) (STACK) *

    // End
    (generics ($($generics:tt)*) () > $($rest:tt)*) => {
        $crate::__internal_impl_component_config!(rest ($($generics)*) $($rest)*);
    };

    // Token
    (generics ($($generics:tt)*) ($($stack:tt)*) $first:tt $($rest:tt)*) => {
        $crate::__internal_impl_component_config!(generics ($($generics)* $first) ($($stack)*) $($rest)*);
    };

    // Open bracket
    (generics ($($generics:tt)*) ($($stack:tt)*) < $($rest:tt)*) => {
        $crate::__internal_impl_component_config!(generics ($($generics)* $first) ($($stack)* <) $($rest)*);
    };

    // Close bracket
    (generics ($($generics:tt)*) (< $($stack:tt)*) > $($rest:tt)*) => {
        $crate::__internal_impl_component_config!(generics ($($generics)* $first) ($($stack)*) $($rest)*);
    };

    (rest ($($generics:tt)*) $sel:ident: $struct:ty, $info:expr, [$( $prop:expr ),*$(,)?]) => {
        impl<$($generics)*> $crate::pattern_builder::component::ComponentConfig for $struct {
            fn info(&$sel) -> &$crate::pattern_builder::component::ComponentInfo {
                &$info
            }

            fn info_mut(&mut $sel) -> &mut $crate::pattern_builder::component::ComponentInfo {
                &mut $info
            }

            fn properties(&$sel) -> Vec<&dyn $crate::pattern_builder::component::property::Property> {
                vec![$( &$prop as &dyn $crate::pattern_builder::component::property::Property),*]
            }

            fn properties_mut(&mut $sel) -> Vec<&mut dyn $crate::pattern_builder::component::property::Property> {
                vec![$(&mut $prop as &mut dyn $crate::pattern_builder::component::property::Property),*]
            }
        }
    };
}