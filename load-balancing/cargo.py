"""This module spawns a locust test interface for svc-cargo."""


from locust import HttpUser, between, task


class Basic(HttpUser):
    wait_time = between(0.5, 5)

    @task
    def query_vertiports(self):
        self.client.post(
            "/cargo/vertiports", json={"latitude": 0, "longitude": 0}
        )
