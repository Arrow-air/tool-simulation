# tool-simulation

Simulated agents to load test the Arrow Services
## :wrench: How it Works

A "Docker Compose" file is used to spin up the microservices needed for the simulation.

A local script contacts the REST layer of the services (`svc-cargo`, `svc-rideshare`, `svc-charter`, etc.) and executes a series of requests. The script follows an EEL (External Event Log) or pseudo-randomly generates events according to a configuration file.

### :rocket: Launch the Microservices

:construction:

Want to launch the microservices locally through a docker-compose.yml file.

:exclamation: A single host running all microservices may experience memory and CPU cap issues. In progress.

We would like to run simulations where the Arrow backend uses a local database instead of real-world assets.

1) `svc-storage` needs an argument to point to a local database
    - This argument should be provided in the `docker-compose.yml`
2) A local CSV file can populate a new PostgreSQL table
    - A CSV file will be checked into the GitHub, e.g. `demo_1_assets.csv`
    - During a simulation run, a PostgreSQL table will be populated for use with svc-storage.
    - The `docker-compose.yml` will launch a PostgreSQL Docker container that `svc-storage` relies on.

### :scroll: Launch a Simulation from an EEL File

EEL files are logs of events with details and timestamps.

Example EEL file:
```json
{
    "events": [
        {
            "event":{
                "CustomerEvent":{
                    "CargoRequest":{
                        "CargoCreate":{
                            "vertiport_depart_id":"vertiport-1",
                            "vertiport_arrive_id":"vertiport-2",
                            "timestamp_depart_min":"2022-01-01T12:12:12",
                            "timestamp_depart_max":"2022-01-01T12:12:12",
                            "cargo_weight_kg": 10.0
                        }
                    }
                }
            },
            "timestamp":"2022-01-01T12:12:12"
        }
    ]
}
```

The simulation tool will follow the EEL file events, issuing customer requests and government flight approvals according to the EEL's log.

This is helpful to test specific edge cases, or re-run previous days' real-world events on different routing libraries.

To run the simulation tool:
```bash
make run -p tool-simulation -- -i demo-1.json
```

Use `validate_eel` to confirm that an EEL file is properly formed.

```bash
cargo run -p validate_eel -- samples/eel.json
```

### :space_invader: Launch a Simulation from a Configuration File

A configuration has rules governing the creation of agents and events.

```bash
make run -p tool-simulation -- -i config.yaml
```

Basic configuration:
```yaml
# Assets
duration_s: 10
timestamp_start: "2022-01-03T12:00:11.002"
```

Use `validate_config` to confirm that a configuration file is properly formed.

```bash
cargo run -p validate_config -- samples/cfg.yaml
```
