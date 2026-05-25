# Route 53 DNS

This CDK app manages the GitHub Pages DNS record for:

```txt
pv-root-cause.proximal.energy
```

It creates a Route 53 `CNAME` record pointing to:

```txt
proximalenergy.github.io.
```

## Deploy

Install the Python CDK dependencies:

```sh
uv sync --directory infra
```

From the repository root, preview the CloudFormation template:

```sh
cdk synth
```

Deploy the record:

```sh
cdk deploy
```

If the AWS account has multiple `proximal.energy` hosted zones, pass the exact zone ID:

```sh
cdk deploy -c hostedZoneId=Z1234567890ABC
```

The root `mise deploy` task performs the same dependency setup, validates and builds
the static site, exports prerendered pages, and deploys this Route 53 stack.
