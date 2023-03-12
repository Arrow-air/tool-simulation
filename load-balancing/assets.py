"""This module spawns a locust test interface for svc-assets."""

import uuid

from locust import HttpUser, between, task


class PlayBoy(HttpUser):
    """A user who only wants to play around with aircraft."""

    wait_time = between(0.5, 5)

    @task
    def register_aircraft(self):
        self.client.post(
            "/assets/aircraft",
            json={
                "manufacturer": "Boeing",
                "model": "747",
                "registration_number": "N12345",
                "max_payload_kg": 100000,
                "max_range_km": 10000,
                "owner": str(uuid.uuid4()),
                "serial_number": "12345",
                "status": "Available",
                "whitelist": [],
            },
        )


class Landlord(HttpUser):
    """A user who only wants to deal with vertiports and vertipads."""

    wait_time = between(0.5, 5)

    @task
    def query_vertiports(self):
        self.client.post(
            "/cargo/vertiports", json={"latitude": 0, "longitude": 0})
