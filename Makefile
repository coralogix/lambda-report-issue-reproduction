build-ExtensionLayer:
	cd extension TARGET_CC=x86-64-unknown-linux-gnu-gcc cargo lambda build --extension --release --locked --target x86_64-unknown-linux-gnu.2.17
	cd extension/target/lambda && cp -r extensions $(ARTIFACTS_DIR)