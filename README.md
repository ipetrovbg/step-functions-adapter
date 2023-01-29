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

```json
{
  "Comment": "Hello State Machine comment",
  "StartAt": "FirstStep",
  "States": {
    "ChoiceStep": {
        "Type": "Choice",
        "Choices": [
            {
                "Variable": "$.userId",
                "IsPresent": true,
                "Next": "GetUser"
            },
            {
                "Variable": "$.userId",
                "IsPresent": false,
                "Next": "Third Step"
            }
        ]
    },
    "FirstStep": {
        "Type": "Task",
        "Resource": "arn:aws:states:::lambda:invoke",
        "Parameters": {
            "FunctionName": "arn:aws:lambda:eu-central-1:00000000000:function:hello"
        },
        "ResultPath": "$.someTestPath",
        "Next": "SecondStep"
    },
    "GetUser": {
        "Type": "Task",
        "Resource": "arn:aws:states:::lambda:invoke",
        "Parameters": {
            "FunctionName": "arn:aws:lambda:eu-central-1:00000000000:function:getUser"
        },
        "Next": "Third Step"
    },
    "SecondStep": {
        "Type": "Task",
        "Resource": "arn:aws:states:::lambda:invoke",
        "Parameters": {
            "FunctionName": "arn:aws:lambda:eu-central-1:00000000000:function:nextFunction"
        },
        "Next": "ChoiceStep"
    },
    "Third Step": {
        "Type": "Pass",
        "End": true
    }
  }
}
```

2. Go to AWS console, navigate to Step Functions service and import the generated `serverless.json` file.

Hopefully you'll see something similar like this.

<img width="1263" alt="image" src="https://user-images.githubusercontent.com/12900528/215326497-087cfa0c-affa-4e05-985d-051a2ab6f526.png">

