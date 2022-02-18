# Needle

Needle is an index for string data, comparable to ElasticSearch in its purpose, but with a belief in using modern industry research to achieve a level of optimality competitive with paid search providers. For instance, established techniques of learning from click data ([see here from Google](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/36252.pdf)), not yet incorporated into any major OSS offerings, are a basic feature in Needle. Structured data is first-class, and can be powerfully exploited with custom 'query plans'.

This is still a work in progress, so I discourage using this in production, but encourage experimenting with it yourself.

It’s designed on a few assumptions:

- **Disk access is fast, and memory faster.** For this, I recommend provisioning high-grade NVMe SSDs. Modern HDDs can outperform SSDs in sequential reads, but the access pattern for a string index is inherently random. Memory should be high-capacity DDR5 RAM, if not NVRAM, and ideally enough to hold a large portion of your dataset.
- **Data is likely to be structured, like an HTML page or a JSON document, and a proper search engine should exploit that.** All the core learning algorithms are written with structured data in mind - though they also work with plain text - and it's key to the query plans operators can configure.
- **Latency is deeply important.** All hot paths are hyper-optimised to avoid allocations and use optimal algorithms even for small tasks, for which Rust, without any GC or runtime and with deterministic binaries, is an excellent foundation. Latency is one of the two key metrics, along with relevance, and it's measured in cycles rather than milliseconds.

## Installation

You can install this with [Cargo](https://cargo.rs) after cloning it:

```
$ RUSTFLAGS="-C target-cpu=native" cargo build
```

## Usage

For a large dataset and heavy traffic, you should run Needle on a machine with >16GB of RAM and a multicore (>2-3) CPU with SIMD support. For now, there's no native support for distributed infrastructure, though it lends itself very naturally to user-implemented sharding.

It's as simple as setting up a `systemd` task to start Needle with `needle --conf={MY_CONF_FILE}`. Needle is architected to safely handle 'pull-the-plug' SIGKILL termination, without having corrupted data or program state, though it will of course lose any unacknowledged writes, and no in-flight queries will be resumed.