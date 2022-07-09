# Step Functions Adapter CLI
 CLI adapter for AWS Step Functions and Serverless framework written in Rust

# Run
```shell
cargo run -- parse tests/serverless.yml
```

# Release
In order to release a new version you need to add "release" keyword in your commit message.
At the moment only patch versions are possible. In the future based on the commit message
you will be able to release (major, feature or bug) version.
```shell
git commit -m "chore: release"
```