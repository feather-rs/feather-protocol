macro_rules! user_type {
    (VarInt) => {
        i32
    };
    ($typ:ty) => {
        $typ
    };
}

macro_rules! packets {
    (
        $(
            $packet:ident {
                $(
                    $field:ident $typ:ident
                );* $(;)?
            }
        ),* $(,)?
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet {
                $(
                    pub $field: user_type!($typ),
                )*
            }

            impl crate::Readable for $packet {
                fn read(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
                where
                    Self: Sized
                {
                    use anyhow::Context as _;
                    $(
                        let $field = <$typ>::read(buffer, version)
                            .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                            .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            impl crate::Writeable for $packet {
                fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {
                    $(
                        self.$field.write(buffer, version);
                    )*
                }
            }
        )*
    };
}

macro_rules! def_enum {
    (
        $ident:ident ($discriminant_type:ty) {
            $(
                $discriminant:literal = $variant:ident
                $(
                    {
                        $(
                            $field:ident: $typ:ident
                        ),* $(,)?
                    }
                )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $variant
                $(
                    {
                        $(
                            $field: user_type!($typ),
                        )*
                    }
                )?,
            )*
        }

        impl crate::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
                where
                    Self: Sized
            {
                use anyhow::Context as _;
                let discriminant = <$discriminant_type>::read(buffer, version)
                    .context(concat!("failed to read discriminant for enum type ", stringify!($ident)))?;

                match discriminant.into() {
                    $(
                        $discriminant => {
                            $(
                                $(
                                    let $field = <$typ>::read(buffer, version)
                                        .context(concat!("failed to read field `", stringify!($field),
                                            "` of enum `", stringify!($ident), "::", stringify!($variant), "`"))?;
                                )*
                            )?

                            Ok($ident::$variant $(
                                {
                                    $(
                                        $field,
                                    )*
                                }
                            )?)
                        },
                    )*
                    _ => Err(anyhow::anyhow!(
                        concat!(
                            "no discriminant for enum `", stringify!($ident), "` matched value {:?}"
                        ), discriminant
                    ))
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {
                match self {
                    $(
                        $ident::$variant $(
                            {
                                $($field,)*
                            }
                        )? => {
                            let discriminant = <$discriminant_type>::from($discriminant);
                            discriminant.write(buffer, version);

                            $(
                                $(
                                    $field.write(buffer, version);
                                )*
                            )?
                        }
                    )*
                }
            }
        }
    };
}

use crate::io::VarInt;

pub mod handshake;
