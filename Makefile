.PHONY: benchmark clean

# Run benchmarks and copy report
benchmark:
	cargo bench
	rm -rf ./reports
	cp -r target/criterion/* ./reports

# Clean up benchmark results and report directory
clean:
	rm -rf target/criterion
	rm -rf reports

# Run benchmarks and open the report in default browser
view: benchmark
	python3 -m webbrowser "file://$(PWD)/reports/report/index.html" || \
	python -m webbrowser "file://$(PWD)/reports/report/index.html" || \
	open "./reports/report/index.html" || \
	xdg-open "./reports/report/index.html"