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

    def add_satellite(self, name, mean_motion, eccentricity, inclination, longitude_of_ascending_node, argument_of_periapsis, mean_anomaly):
        data = {
            "jsonrpc": "2.0",
            "method": "add_satellite",
            "id": self.__get_id(),
            "params": {
                "id": name,
                "elements": [mean_motion, eccentricity, inclination, longitude_of_ascending_node, argument_of_periapsis, mean_anomaly]
            }
        }
        print(self.__send(data))

    def __send(self, data):
        response = requests.post(f'http://{self.host}:{self.port}', json=data)
        return response.json()

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.connection:
            self.connection.close()
