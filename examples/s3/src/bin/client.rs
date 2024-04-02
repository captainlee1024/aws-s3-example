// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::result_large_err)]

use std::env;
// snippet-start:[s3.rust.client-use]
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
// snippet-end:[s3.rust.client-use]

/// Lists your buckets.
#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    // snippet-start:[s3.rust.client-client]
    const LOCALSTACK_ENDPOINT: &str = "http://localhost:4566/";
    env::set_var("AWS_ACCESS_KEY_ID", "test");
    env::set_var("AWS_SECRET_ACCESS_KEY", "test");
    env::set_var("AWS_DEFAULT_REGION", "us-east-1");
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).endpoint_url(LOCALSTACK_ENDPOINT).load().await;
    let client =  s3_client(&config);
    // snippet-end:[s3.rust.client-client]

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    println!("Found {} buckets.", num_buckets);

    Ok(())
}
fn s3_client(conf: &aws_config::SdkConfig) -> aws_sdk_s3::Client {
    // Copy config from aws_config::SdkConfig to aws_sdk_s3::Config
    let s3_config_builder = aws_sdk_s3::config::Builder::from(conf);
    aws_sdk_s3::Client::from_conf(s3_config_builder.build())
}
