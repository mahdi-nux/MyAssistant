use candle_core::{D, DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2;
use serde_json::from_str;
use std::fs::read_to_string;
use std::io::{Write, stdin, stdout};
use tokenizers::Tokenizer;

fn read_input() -> String {
    let mut user_input = String::new();
    print!("\n>>> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut user_input).unwrap();
    user_input
}

fn main() {
    let device = match Device::new_cuda(0) {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };

    let tokenizer = match Tokenizer::from_file(
        "/home/mahdi/.cache/huggingface/hub/models--deepseek-ai--DeepSeek-R1-Distill-Qwen-1.5B/snapshots/ad9f0ae0864d7fbcd1cd905e3c6c5b069cc8b562/tokenizer.json",
    ) {
        Ok(tokenizer) => tokenizer,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };

    let model_path = "/home/mahdi/.cache/huggingface/hub/models--deepseek-ai--DeepSeek-R1-Distill-Qwen-1.5B/snapshots/ad9f0ae0864d7fbcd1cd905e3c6c5b069cc8b562/model.safetensors";

    let var_builder =
        match unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::BF16, &device) } {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

    let config_path = "/home/mahdi/.cache/huggingface/hub/models--deepseek-ai--DeepSeek-R1-Distill-Qwen-1.5B/snapshots/ad9f0ae0864d7fbcd1cd905e3c6c5b069cc8b562/config.json";

    let tmp = match read_to_string(config_path) {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };

    let config: qwen2::Config = match from_str(&tmp) {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };

    let mut model = match qwen2::ModelForCausalLM::new(&config, var_builder) {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };

    loop {
        let input = read_input();

        if input.trim().is_empty() {
            continue;
        }
        if input.trim() == "exit" {
            break;
        }

        let prompt = format!(
            "<｜begin▁of▁sentence｜><｜User｜>{}<｜Assistant｜><think>\n",
            input.trim()
        );

        let mut generated_tokens = 0;

        let encoding = match tokenizer.encode(prompt, false) {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        let mut tokens = encoding.get_ids().to_vec();

        let tensor = match Tensor::new(tokens.as_slice(), &device) {
            Ok(value) => match value.reshape((1, tokens.as_slice().len())) {
                Ok(reshaped) => reshaped,
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            },
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        println!("encoding: {:?}", encoding.get_tokens());
        println!("tensor: {}", tensor);

        model.clear_kv_cache();

        let logits = match model.forward(&tensor, 0) {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        generated_tokens += 1;

        println!("logits: {}", logits);

        let next_token_tensor = match logits
            .squeeze(0)
            .unwrap()
            .squeeze(0)
            .unwrap()
            .argmax(D::Minus1)
        {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        let mut next_token_id = match next_token_tensor.to_scalar::<u32>() {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        println!("next token id: {}", next_token_id);

        tokens.push(next_token_id);

        let decoded = match tokenizer.decode(
            &[next_token_id], 
            false
        ) {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };

        print!("{}", decoded);
        stdout().flush().unwrap();

        while next_token_id != 151643 && generated_tokens < 200 {
            let second_tensor = match Tensor::new(&[next_token_id], &device) {
                Ok(value) => match value.reshape((1, 1)) {
                    Ok(reshaped) => reshaped,
                    Err(error) => {
                        println!("{}", error);
                        panic!()
                    }
                },
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            };

            let new_logits = match model.forward(&second_tensor, tokens.len() - 1) {
                Ok(value) => value,
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            };

            generated_tokens += 1;

            let new_next_token_tensor = match new_logits
                .squeeze(0)
                .unwrap()
                .squeeze(0)
                .unwrap()
                .argmax(D::Minus1)
            {
                Ok(value) => value,
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            };

            next_token_id = match new_next_token_tensor.to_scalar::<u32>() {
                Ok(value) => value,
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            };

            tokens.push(next_token_id);

            let new_decoded = match tokenizer.decode(
                &[next_token_id], 
                false
            ) {
                Ok(value) => value,
                Err(error) => {
                    println!("{}", error);
                    panic!()
                }
            };

            print!("{}", new_decoded);
            stdout().flush().unwrap();
        }
    }
}

