# poc-oci-on-demand

This *"Proof-of-concept"* evaluates how it is possible to

1. Create an OCI compliant image with arbitrary content
2. Pull and read out an OCI compliant image programmatically

The image creation is done based on `buildah`,
see __scripts/build_n_push_image.sh__.

Reading it out is done in code.