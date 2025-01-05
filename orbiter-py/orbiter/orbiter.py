import requests


class Orbiter:
    def __init__(self, host='localhost', port=12340):
        self.host = host
        self.port = port
        self.connection = None

    def __enter__(self):
        self.id = 0
        return self

    def __get_id(self):
        self.id += 1
        return self.id

    def add_satellite(self, name):
        data = {
            "jsonrpc": "2.0",
            "method": "bevy/add_satellite",
            "id": self.__get_id(),
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
            "id": self.__get_id(),
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
