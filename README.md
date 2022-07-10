# Step Functions Adapter CLI
 CLI adapter for AWS Step Functions and Serverless framework written in Rust

# Prerequisites
- `Rust`
- `cargo`

# Run

1. Run the CLI with test `serverless.yml` file

```shell
cargo run -- parse tests/serverless.yml
```

This command will output something like this in your terminal.

![image](https://user-images.githubusercontent.com/12900528/178156999-a4f25c2e-43d8-48fb-a09d-cfe045dd51b9.png)

2. Then you create a new json file with copied json string.

3. Go to AWS console, navigate to Step Functions service and import your json file.

Hopefully you'll see something similar like this.

![image](https://user-images.githubusercontent.com/12900528/178157093-79b61799-fb48-4973-bc9a-e576d5a338c0.png)
