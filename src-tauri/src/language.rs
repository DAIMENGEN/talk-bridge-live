macro_rules! define_languages {
    ($($variant:ident => $code:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Language {
            $($variant),*
        }

        impl Language {

            #[allow(dead_code)]
            pub fn from_code(code: &str) -> Option<Self> {
                match code {
                    $( $code => Some(Language::$variant), )*
                    _ => None,
                }
            }

            #[allow(dead_code)]
            pub fn code(&self) -> &'static str {
                match self {
                    $( Language::$variant => $code, )*
                }
            }
        }
    };
}

// Define the language codes.
define_languages!(
    Chinese => "zh",
    English => "en",
    Japanese => "ja",
    Russian => "ru",
    German => "de",
    Korean => "ko",
    French => "fr",
    Spanish => "es",
);