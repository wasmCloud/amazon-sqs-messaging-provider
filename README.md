# Amazon SQS Messaging Provider

This library is a [waSCC](https://wascc.dev/) _native capability provider_ that interacts with [Amazon SQS](https://aws.amazon.com/sqs/) to implement the [`wascap:messaging`](https://github.com/wascc/wascc-codec) protocol.

The following configuration values can be passed to the waSCC host runtime to configure the SQS client on a per-actor basis:

* `SQS_QUEUE_URL` - The SQS [queue URL](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-general-identifiers.html).
