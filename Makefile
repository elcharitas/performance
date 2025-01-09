.PHONY: benchmark clean

# Run benchmarks and copy report
benchmark:
	cargo bench
	rm -rf ./report
	cp -r target/criterion/report ./report

# Clean up benchmark results and report directory
clean:
	rm -rf target/criterion
	rm -rf report

# Run benchmarks and open the report in default browser
view: benchmark
	python3 -m webbrowser "file://$(PWD)/report/index.html" || \
	python -m webbrowser "file://$(PWD)/report/index.html" || \
	open "./report/index.html" || \
	xdg-open "./report/index.html"