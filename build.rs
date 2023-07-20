use const_gen::*;
use masp_proofs::bellman::groth16::Parameters;
use masp_proofs::bls12_381::Bls12;
use namada_masp_params::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let spend_params = Parameters::<Bls12>::read(NAMADA_MASP_SPEND_PARAMS.as_slice(), false)
        .expect("expected valid spend params");
    let convert_params = Parameters::<Bls12>::read(NAMADA_MASP_CONVERT_PARAMS.as_slice(), false)
        .expect("expected valid convert params");
    let output_params = Parameters::<Bls12>::read(NAMADA_MASP_OUTPUT_PARAMS.as_slice(), false)
        .expect("expected valid output params");

    let spend_vk = spend_params.vk;
    let convert_vk = convert_params.vk;
    let output_vk = output_params.vk;

    let mut spend_vk_bytes = vec![];
    spend_vk
        .write(&mut spend_vk_bytes)
        .expect("expected to serialize spend vk");
    let mut convert_vk_bytes = vec![];
    convert_vk
        .write(&mut convert_vk_bytes)
        .expect("expected to serialize convert vk");
    let mut output_vk_bytes = vec![];
    output_vk
        .write(&mut output_vk_bytes)
        .expect("expected to serialize output vk");

    let mut constants_rs =
        File::create("src/constants.rs").expect("expected to create constants file");
    let declarations = vec![
        const_declaration!(pub NAMADA_MASP_SPEND_VK_BYTES = spend_vk_bytes),
        const_declaration!(pub NAMADA_MASP_CONVERT_VK_BYTES = convert_vk_bytes),
        const_declaration!(pub NAMADA_MASP_OUTPUT_VK_BYTES = output_vk_bytes),
    ]
    .join("\n");
    constants_rs
        .write_all(declarations.as_ref())
        .expect("expected to write to constants file");
}
