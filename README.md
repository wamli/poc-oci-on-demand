# poc-oci-on-demand

This *"Proof-of-concept"*

1. Creates an OCI compliant image based on arbitrary content in a given folder (__*scripts/build_n_push_from_script.sh*__)
2. Pushes the new OCI compliant image to a local registry (__*scripts/tag_n_push.sh*__)
3. Pulls and reads out an OCI compliant image programmatically (__*src*__)

Call it like

```bash
# create image
scripts/build_n_push_from_script.sh binaries/squeezenet wamli-squeezenetv117

# push to local registry
scripts/tag_n_push.sh wamli-squeezenetv117 wamli-squeezenetv117:latest
```

Reading it out is supposed to be done from code.
