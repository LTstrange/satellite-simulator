class SatSimManager:
    def __init__(self, host: str, port: int):
        """
        Connect to the Satellite Simulator with the given host and port.

        Args:
            host (str): The host name or IP address of the SatSim server.
            port (int): The port number of the SatSim server.
        """
        ...

    def clean_all():
        """
        Remove all satellites from the SatSim simulation.
        """
        ...

    def create_orbit(self, apogee_km: int, perigee_km: int,
                     inclination_rad: float):
        """
        Create a new orbit with the given apogee and perigee distances,
        inclination in radians, and add it to the SatSim simulation.

        Args:
            apogee_km (int): The apogee distance in kilometers.
            perigee_km (int): The perigee distance in kilometers.
            inclination_rad (float): The inclination in radians.
        """
        ...
