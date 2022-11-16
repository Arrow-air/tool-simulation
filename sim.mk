include .make/rust.mk

help:
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Simulation Tool$(SGR0)"
	@echo "  $(BOLD)run-sim FILE=<filename>$(SGR0)      -- Runs the simulation from the provided file."
	@echo "  $(BOLD)validate-eel FILE=<filename>$(SGR0) -- Validates an EEL file"
	@echo "  $(BOLD)validate-cfg FILE=<filename>$(SGR0) -- Validates a simulation config file"

sim_base = docker run \
		--name=$(1) \
		--rm \
		--user `id -u`:`id -g` \
		--workdir=/usr/src/app \
		--network host \
		-v "$(PWD)/:/usr/src/app" \
		-v "$(PWD)/.cargo/registry:/usr/local/cargo/registry" \
		-e CARGO_INCREMENTAL=$(CARGO_INCREMENTAL) \
		-e RUSTC_BOOTSTRAP=$(RUSTC_BOOTSTRAP) \
		-t $(RUST_IMAGE_NAME):$(RUST_IMAGE_TAG) \
		cargo run -p $(1) -- -i $(2)

run-sim:
	@echo "$(CYAN)Running the simulation from $(FILE)...$(SGR0)"
	@$(call sim_base,tool-simulation,$(FILE))

validate-eel:
	@echo "$(CYAN)Validating EEL file: $(FILE)...$(SGR0)"
	@$(call sim_base,validate_eel,$(FILE))

validate-cfg:
	@echo "$(CYAN)Validating Configuration: $(FILE)...$(SGR0)"
	@$(call sim_base,validate_config,$(FILE))
