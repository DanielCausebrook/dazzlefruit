#![allow(non_camel_case_types)]

macro_rules! packet_type_enum {
    ($enumName:ident<$valueType:ty>, {$($variantName:ident => $variantValue:expr),*$(,)?}) => {
        pub enum $enumName {
            $($variantName,)*
        }
        impl Into<$valueType> for $enumName {
            fn into(self) -> $valueType {
                match self {
                    $($enumName::$variantName => $variantValue,)*
                }
            }
        }
        impl TryFrom<$valueType> for $enumName {
            type Error = ();
            fn try_from(value: $valueType) -> Result<Self, Self::Error> {
                match value {
                    $($variantValue => Result::Ok($enumName::$variantName),)*
                    _ => Result::Err(())
                }
            }
        }
    };
}

packet_type_enum!(TcpPacketType<u8>, {
    Hello => 1,
    Ok => 2,
    Err => 3,
    Ping => 5,
    SetWifi => 7,
    UnsetWifi => 8,
    Neopixel_Init => 10,
    Neopixel_Show => 11,
    Neopixel_Auto => 12,
    Neopixel_Off => 19,
    Matrix11x7_Init => 20,
    Matrix11x7_Show => 21,
    Matrix11x7_Off => 22,
    Matrix5x5_Init => 30,
    Matrix5x5_Show => 31,
    Matrix5x5_Off => 32,
    Inky_Init => 40,
    Inky_Show => 41,
    Inky_Off => 42,
    DistanceSensor_Init => 100,
    LightSensor_Init => 110,
    TempSensor_Init => 120,
});

packet_type_enum!(UdpPacketType<u8>, {
    Neopixel_Show => 11,
    Matrix11x7_Show => 21,
    Matrix5x5_Show => 31,
});