# uplc-apply-args

Implementation of argument application for Cardano Plutus scripts (nodejs & the browser).

This package defines a single "apply_params_to_script_no_panic" function.

## Building and publishing

Here's a description on to how publish a newer version to the NPM registry:

1. Install wasm-pack according to [wasm-pack docs](https://rustwasm.github.io/docs/wasm-pack/quickstart.html).
2. Set the `package.name` in `Cargo.toml` to the correct name (i.e. `apply-args-browser` or `apply-args-nodejs`. We can't use `@mlabs-haskell/` prefix here, because the name must be a valid crate name, but we can update it later in the generated package (step 5)).
3. Bump the `package.version` in `Cargo.toml`. Set it to be equal to the version of the uplc crate.
4. Run `wasm-pack build --target [bundler|nodejs]`.
5. Modify the generated npm package located in `./pkg`: Change its name to `@mlabs-haskell/`
6. Publish the generated package.
7. Repeat steps 1-6 for the `-nodejs` version too.
8. Update the isomorphic (supporting both nodejs and the browser) package that is located in `./npm-package` and publish it
