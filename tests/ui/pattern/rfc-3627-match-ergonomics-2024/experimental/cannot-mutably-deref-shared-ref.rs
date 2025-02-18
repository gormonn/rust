//@ edition: 2024
//@ revisions: classic structural
//! Test that `&mut` patterns don't match shared reference types under new typing rules in Rust 2024
#![allow(incomplete_features)]
#![cfg_attr(classic, feature(ref_pat_eat_one_layer_2024))]
#![cfg_attr(structural, feature(ref_pat_eat_one_layer_2024_structural))]

pub fn main() {
    let &mut _ = &&0;
    //~^ ERROR: mismatched types

    let &mut _ = &&&&&&&&&&&&&&&&&&&&&&&&&&&&0;
    //~^ ERROR: mismatched types

    let &mut _ = &&mut 0;
    //~^ ERROR: mismatched types

    let &mut _ = &&&&&&&&&&&&&&&&&&&&&&&&&&&&mut 0;
    //~^ ERROR: mismatched types

    let &mut &mut &mut &mut _ = &mut &&&&mut &&&mut &mut 0;
    //~^ ERROR: mismatched types
}
