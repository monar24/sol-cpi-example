# sol-cpi-example

* Cross Program Invocation to System Program to transfer coins
* Cross Program Invocation from Program A to B to call additional functionality 
* Make sure to update programB as a dependency in program A cargo.toml, with features = ["cpi"]
* CpiContext::new_with_signer is the equivalent of the System Program's Invoke_signed