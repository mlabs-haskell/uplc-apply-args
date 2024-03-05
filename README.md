# uplc-apply-args

Implementation of argument application for Cardano Plutus scripts (nodejs & the browser).

This package defines a single "apply_params_to_script_no_panic" function.

## Building and publishing

Here's a description on to how publish a newer version to the NPM registry:

1. Install wasm-pack according to [wasm-pack docs](https://rustwasm.github.io/docs/wasm-pack/quickstart.html).
2. Set the `package.name` in `Cargo.toml` to the correct name (i.e. `apply-args-browser`). This will become the npm package name.
3. Bump the `package.version` in `Cargo.toml`.
4. Run `wasm-pack build --target [bundler|nodejs]`.
5. Run `wasm-pack publish --target [bundler|nodejs]`. You may need to login to the registry you want to publish to. You can login using `wasm-pack login`.
6. Re-publish the NPM meta-package (located in `./npm-package`)
