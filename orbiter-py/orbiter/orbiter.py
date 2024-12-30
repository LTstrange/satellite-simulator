import requests


class Orbiter:
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self.connection = None

    def __enter__(self):
        self.connection = requests.get(
            f'http://{self.host}:{self.port}')
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.connection:
            self.connection.close()
