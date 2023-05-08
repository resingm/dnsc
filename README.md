# dnsc

DNS Lookup tool to lookup thousands of domain names quickly.

## TODO:

* ~Make DNS server configurable~
* Loop CNAME responses back to query channel
* ~Rate limiting on query sender~
* ~Timeout on receiver socket~
* Improve error handling (so far errors are simply printed to STDERR)
* Publish on Cargo.rs


More advanced features:

* Query ID/Domain name book keeping and resending if no response within a timeout



