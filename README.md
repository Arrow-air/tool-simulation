# tool-simulation

Simulated agents to load test the Arrow Services
## :wrench: How it Works

A "Docker Compose" file is used to spin up the microservices needed for the simulation.

A local script contacts the REST layer of the services (`svc-cargo`, `svc-rideshare`, `svc-charter`, etc.) and executes a series of requests. The script follows an EEL (External Event Log) or pseudo-randomly generates events according to a configuration file.

### :rocket: Launch the Microservices

See the [tools/local-dev/README.md](https://github.com/Arrow-air/tools/blob/main/local-dev/README.md) file for instructions on using a `docker-compose.yml` file to launch the microservices.

If testing local branches of a microservice (e.g. `svc-cargo`), build the docker container for that service (`make docker-build`) and make the following edits to the `.env` file used in tandem with `docker compose`:

```dotenv
# -----------------------------------------------------------
# svc-cargo
# -----------------------------------------------------------
CARGO_IMAGE=svc-cargo # instead of ghcr.io/etc...
CARGO_TAG=latest # instead of develop
CARGO_PORT_GRPC=50000
CARGO_PORT_REST=8000
```

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
                        "Create":{
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
make -f sim.mk run-sim FILE=samples/eel.json
```

Use `validate_eel` to confirm that an EEL file is properly formed.

```bash
make -f sim.mk validate-eel FILE=samples/eel.json
```

### :space_invader: Launch a Simulation from a Configuration File

A configuration has rules governing the creation of agents and events.

```bash
make -f sim.mk run-sim FILE=samples/cfg.yaml
```

Basic configuration:
```yaml
# Assets
duration_s: 10
timestamp_start: "2022-01-03T12:00:11.002"
n_customers: 1
customer_types:
- greedy # Will not cancel, will pick up first available flight
- mistake # Will book a flight and then attempt to cancel
- indecisive # Will query flights but never pick anything
```

Use `validate_config` to confirm that a configuration file is properly formed.

```bash
make -f sim.mk validate-cfg FILE=samples/cfg.yaml 
```

## Load Balancing
*Note: It is highly recommended that you set up a [virtual
environment](https://docs.python.org/3/library/venv.html) under
the `load-balancing` folder so that you don't install unnecessary
packages on your local machine.*

To test load balancing, run the following commands:

```bash
make prepare
make start svc-<service-name>
```

Then go to the website printed on the console and start the simulation.

For example, to test the load balancing of the `svc-cargo` service, run the following commands:

```bash
make prepare
make start svc-cargo
```
