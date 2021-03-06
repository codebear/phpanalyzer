autonodes:
	cd tree-sitter-php && tree-sitter generate
	node gennodes.mjs
	rustfmt src/autonodes/*.rs

native:
	cd src/native && php generate.php
	rustfmt src/native/**/*.rs

autofix:
	cargo fix --allow-dirty	
	rustfmt src/**/*.rs
