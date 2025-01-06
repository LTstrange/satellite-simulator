import requests
from dataclasses import dataclass

standard_gravitational_parameter = 3.986004418e5  # km^3/s^2


class Satellite:
    def __init__(self):
        self.id = ""
        self.elements = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

    def set_apogee_perigee(self, apogee: float, perigee: float):
        assert (apogee > perigee)
        e = (apogee - perigee) / (apogee + perigee)
        mean_motion = (standard_gravitational_parameter /
                       ((apogee + perigee)/2) ** 3) ** 0.5
        self.set_mean_motion(mean_motion)
        self.set_eccentricity(e)

    def set_id(self, id: str):
        self.id = id

    def set_mean_motion(self, mean_motion: float):
        self.elements[0] = mean_motion

    def set_eccentricity(self, eccentricity: float):
        self.elements[1] = eccentricity

    def set_inclination(self, inclination: float):
        self.elements[2] = inclination

    def set_longitude_of_ascending_node(self, longitude_of_ascending_node: float):
        self.elements[3] = longitude_of_ascending_node

    def set_argument_of_periapsis(self, argument_of_periapsis: float):
        self.elements[4] = argument_of_periapsis

    def set_mean_anomaly(self, mean_anomaly: float):
        self.elements[5] = mean_anomaly


class Orbiter:
    def __init__(self, host='localhost', port=12340):
        self.host = host
        self.port = port

    def __enter__(self):
        self.id = 0
        return self

    def __get_id(self):
        self.id += 1
        return self.id

    def add_satellite(self, satellite: Satellite):
        data = {
            "jsonrpc": "2.0",
            "method": "add_satellite",
            "id": self.__get_id(),
            "params": {
                "id": satellite.id,
                "elements": satellite.elements
            }
        }
        print(self.__send(data))

    def __send(self, data):
        response = requests.post(f'http://{self.host}:{self.port}', json=data)
        return response.json()

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass
