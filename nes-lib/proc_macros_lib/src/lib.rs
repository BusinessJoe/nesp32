extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
// Sinful code - forgive me
pub fn generate_nes_6502_tests(_: TokenStream) -> TokenStream {
    let mut output_token_stream_str = String::new();
    
    for i in 0 ..= 0xFF {
        output_token_stream_str.push_str(&format!(r#"
            #[datatest::data("tests/nes6502/v1/{:02X}.json")]
            fn nes_6502_test_{:02X}(case: Nes6502TestCase) {{
                let bus = MockBus::new();
                let mut nes = nes_lib::Nes::new(bus);
                assert_eq!(0, 1)
            }}
        "#, i, i));
    }

    let output = output_token_stream_str.parse().unwrap();
    output
}
