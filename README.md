# Step Functions Adapter CLI
 CLI adapter for AWS Step Functions and Serverless framework

 Note: Use only for visual representation

# Prerequisites
- `Rust`
- `cargo`

# Run

1. Run the CLI with test `serverless.yml` file

```shell
cargo run -- parse tests/serverless.yml > test/serverless.json
```

This command will output something like this in `test/serverless.json` file.

![image](https://user-images.githubusercontent.com/12900528/178156999-a4f25c2e-43d8-48fb-a09d-cfe045dd51b9.png)

2. Go to AWS console, navigate to Step Functions service and import generated `serverless.json` file.

Hopefully you'll see something similar like this.

![image](https://user-images.githubusercontent.com/12900528/178157093-79b61799-fb48-4973-bc9a-e576d5a338c0.png)
