use std::io;
use rs_llama_cpp::{gpt_params_c, run_inference, str_to_mut_i8};

pub fn generate(prompt: &str, model: &str) -> String {
    let params: gpt_params_c = {
        gpt_params_c {
            model: str_to_mut_i8(model),
            prompt: str_to_mut_i8(prompt),
            ..Default::default()
        }
    };

    run_inference(params, |token| {
        println!("Token: {}", token);

        if token.ends_with("\n") {
            return false; // stop inference
        }

        return true; // continue inference
    });
    return ":3".to_string();
}
