#!/usr/bin/env python3

import aws_cdk as cdk
from aws_cdk import Stack
from aws_cdk import aws_route53 as route53
from constructs import Construct


def with_trailing_dot(value: str) -> str:
    return value if value.endswith(".") else f"{value}."


class PvRootCauseDnsStack(Stack):
    def __init__(
        self,
        scope: Construct,
        construct_id: str,
        *,
        apex_domain: str,
        record_name: str,
        pages_target: str,
        hosted_zone_id: str | None = None,
        **kwargs,
    ) -> None:
        super().__init__(scope, construct_id, **kwargs)

        fqdn = f"{record_name}.{apex_domain}"
        zone_selector = (
            {"hosted_zone_id": hosted_zone_id}
            if hosted_zone_id
            else {"hosted_zone_name": with_trailing_dot(apex_domain)}
        )

        route53.CfnRecordSet(
            self,
            "PvRootCausePagesCname",
            name=with_trailing_dot(fqdn),
            type="CNAME",
            ttl="300",
            resource_records=[with_trailing_dot(pages_target)],
            **zone_selector,
        )


app = cdk.App()

PvRootCauseDnsStack(
    app,
    "PvRootCauseDnsStack",
    apex_domain=app.node.try_get_context("apexDomain") or "proximal.energy",
    record_name=app.node.try_get_context("recordName") or "pv-root-cause",
    pages_target=app.node.try_get_context("pagesTarget") or "proximalenergy.github.io.",
    hosted_zone_id=app.node.try_get_context("hostedZoneId"),
)

app.synth()
