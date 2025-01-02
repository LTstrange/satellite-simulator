import requests


class Orbiter:
    def __init__(self, host='localhost', port=15702):
        self.host = host
        self.port = port
        self.connection = None

    def __enter__(self):
        return self

    def add_satellite(self, name):
        data = {
            "jsonrpc": "2.0",
            "method": "bevy/add_satellite",
            "id": 1,
            "params": {
                "name": name
            }
        }
        print(self.__send(data))

    def test(self):
        print('test')
        data = {
            "jsonrpc": "2.0",
            "method": "bevy/query",
            "id": 1,
            "params": {
                "data": {
                    "components": ["bevy_transform::components::transform::Transform"],
                },
            }
        }
        print(self.__send(data))

    def __send(self, data):
        response = requests.post(f'http://{self.host}:{self.port}', json=data)
        return response.json()

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.connection:
            self.connection.close()
