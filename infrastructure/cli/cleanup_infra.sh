# Setup Name Vars

ROLE_NAME="rust-link-shortener-ec2-role"
INSTANCE_PROFILE_NAME="rust-link-shortener-ec2-role"
POLICY_ARN=$(/usr/local/bin/aws iam list-policies --query 'Policies[?PolicyName==`rust-link-shortener-dynamo-permissions`].Arn' --output text)
TABLE_NAME="url_shortener"

## Delete DynamoDB Table
echo "Deleting DynamoDB Table..."
/usr/local/bin/aws dynamodb delete-table --table-name $TABLE_NAME --no-cli-pager

## Delete Instance Profile
echo "Deleting Instance Profile..."
/usr/local/bin/aws iam remove-role-from-instance-profile --instance-profile-name $INSTANCE_PROFILE_NAME --role-name $ROLE_NAME --no-cli-pager
/usr/local/bin/aws iam delete-instance-profile --instance-profile-name $INSTANCE_PROFILE_NAME --no-cli-pager

## Delete IAM Policy
echo "Deleting IAM Policy..."
/usr/local/bin/aws iam detach-role-policy --policy-arn $POLICY_ARN --role-name $ROLE_NAME --no-cli-pager
/usr/local/bin/aws iam delete-policy --policy-arn $POLICY_ARN --no-cli-pager

## Delete IAM Role
echo "Deleting IAM Role..."
/usr/local/bin/aws iam delete-role --role-name $ROLE_NAME --no-cli-pager